use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use serde::Serialize;
use serde_json::{Value, json};
use tokio::{
    net::TcpStream,
    sync::{Mutex, broadcast, oneshot},
    time::sleep,
};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Bytes, Utf8Bytes, client::IntoClientRequest, protocol::Message},
};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{
    common::{
        errors::ExchangeError,
        types::{FinalRequest, WebSocketParams, WebSocketRequest, WsMethod},
    },
    models::ws::subscriptions::{
        AccountInfo, AccountTrades, Balance, Candle, Leverage, Margin, OrderBook, OrderUpdates,
        Orders, Positions, Prices, SubscriptionMethod, Trades,
    },
};

/// All messages emitted by the client over the single event bus.
#[derive(Debug, Clone)]
pub enum WsEvent {
    /// A server push bound to a channel (e.g., "trades:BTC-PERP").
    Channel { channel: String, payload: Value },
    /// A response for a request/response API (also delivered via oneshot to the caller).
    Response { id: String, payload: Value },
    /// A server/application error (optionally for a request id).
    Error { id: Option<String>, payload: Value },
    /// App-level pong (if the server sends {"channel": "pong"}).
    Pong,
    /// Underlying websocket closed.
    Closed,
}

/// Public client that users interact with.
#[derive(Clone)]
pub struct WebSocketClient(Arc<Inner>);

// Simpler alias to avoid clippy::type_complexity on the write sink type.
type WriteSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

struct Inner {
    url: String,
    api_key: Option<String>,

    /// Write half of the websocket (None when disconnected).
    write: Arc<Mutex<Option<WriteSink>>>,

    /// Broadcast bus for all events.
    bus_tx: broadcast::Sender<WsEvent>,

    /// Per-remote-channel subscribers for backwards-compatible channel APIs.
    subscribers: Arc<Mutex<HashMap<String, broadcast::Sender<Value>>>>,

    /// Active subscriptions remembered for resubscribe on reconnect.
    /// Key = unique chan_key (e.g., "trades:BTC-PERP").
    active_subscriptions: Arc<Mutex<HashMap<String, WebSocketParams<Value>>>>,

    /// Pending request/response futures keyed by id.
    pending_requests: Arc<Mutex<HashMap<String, oneshot::Sender<Value>>>>,

    /// Close flag for the run loop and ping task.
    closed: Arc<AtomicBool>,
}

/// RAII subscription handle: unsubscribes on drop.
pub struct SubHandle {
    client: WebSocketClient,
    chan_key: String,
    rx: broadcast::Receiver<Value>,
}

impl Drop for SubHandle {
    fn drop(&mut self) {
        let client = self.client.clone();
        let chan = self.chan_key.clone();
        tokio::spawn(async move {
            client.unsubscribe(&chan).await.ok();
        });
    }
}

impl SubHandle {
    pub async fn recv(&mut self) -> Option<Value> {
        self.rx.recv().await.ok()
    }

    pub fn attach_callback<F>(mut self, callback: F)
    where
        F: FnMut(Value) + Send + 'static,
    {
        tokio::spawn(async move {
            let mut cb = callback;
            while let Ok(value) = self.rx.recv().await {
                cb(value);
            }
        });
    }
}

impl WebSocketClient {
    /// Create and start the client run loop immediately.
    pub async fn new(url: &str, api_key: Option<String>) -> Result<Self, ExchangeError> {
        let (bus_tx, _) = broadcast::channel(1024);

        let inner = Arc::new(Inner {
            url: url.to_string(),
            api_key,
            write: Arc::new(Mutex::new(None)),
            bus_tx,
            subscribers: Arc::new(Mutex::new(HashMap::new())),
            active_subscriptions: Arc::new(Mutex::new(HashMap::new())),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            closed: Arc::new(AtomicBool::new(false)),
        });

        let inner_clone = inner.clone();
        tokio::spawn(async move {
            inner_clone.run().await;
        });

        Ok(Self(inner))
    }

    /// Subscribe to the global event stream (each caller gets its own receiver).
    pub fn events(&self) -> broadcast::Receiver<WsEvent> {
        self.0.bus_tx.subscribe()
    }

    /// Close the client (connection will drop and loop will exit).
    pub async fn close(&self) -> Result<(), ExchangeError> {
        self.0.closed.store(true, Ordering::Relaxed);
        let mut guard = self.0.write.lock().await;
        if let Some(sink) = guard.as_mut() {
            sink.send(Message::Close(None)).await?;
        }
        Ok(())
    }

    /// Optionally set/replace API key (example keeps it simple).
    pub async fn set_api_key(&self, api_key: String) -> Result<String, ExchangeError> {
        // You may want to reconnect to apply the header; here we just store it.
        Ok(api_key)
    }

    // =========================================================================
    // Request/Response
    // =========================================================================

    /// Send a request to the exchange's WS RPC and get a oneshot receiver for the response.
    pub async fn send_exchange_request<P: Serialize>(
        &self,
        request_id: Option<Uuid>,
        request_method: &str,
        request: FinalRequest<P>,
    ) -> Result<oneshot::Receiver<Value>, ExchangeError> {
        let request_id = request_id.unwrap_or(Uuid::new_v4());
        let mut params = HashMap::new();
        params.insert(request_method.to_string(), request);

        let web_socket_request: WebSocketRequest<P> = WebSocketRequest {
            id: request_id,
            params,
        };
        let msg_text = json!(web_socket_request).to_string();

        // Create oneshot and register before sending
        let (tx, rx) = oneshot::channel();
        self.0
            .pending_requests
            .lock()
            .await
            .insert(request_id.to_string(), tx);

        self.0.send_text(msg_text).await?;
        Ok(rx)
    }

    // =========================================================================
    // High-level subscription helpers (RAII handles)
    // Each returns SubHandle and publishes data on the global bus.
    // =========================================================================

    pub async fn subscribe_to_prices(&self) -> Result<SubHandle, ExchangeError> {
        let p = Prices {};
        self.subscribe_raw(SubscriptionMethod::Prices, &p, "prices")
            .await
    }

    pub async fn subscribe_to_orderbook(
        &self,
        symbol: &str,
        agg_level: Option<crate::common::types::AggLevel>,
    ) -> Result<SubHandle, ExchangeError> {
        let p = OrderBook {
            symbol: symbol.to_string(),
            agg_level,
        };
        let key = format!(
            "{}:{}{}",
            SubscriptionMethod::Book,
            symbol,
            agg_suffix(agg_level)
        );
        self.subscribe_raw(SubscriptionMethod::Book, &p, &key).await
    }

    pub async fn subscribe_to_trades(&self, symbol: &str) -> Result<SubHandle, ExchangeError> {
        let p = Trades {
            symbol: symbol.to_string(),
        };
        let key = format!("{}:{}", SubscriptionMethod::Trades, symbol);
        self.subscribe_raw(SubscriptionMethod::Trades, &p, &key)
            .await
    }

    pub async fn subscribe_to_candle(
        &self,
        symbol: &str,
        interval: crate::common::types::Interval,
    ) -> Result<SubHandle, ExchangeError> {
        let p = Candle {
            symbol: symbol.to_string(),
            interval: interval.clone(),
        };
        let key = format!("{}:{}:{:?}", SubscriptionMethod::Candle, symbol, interval);
        self.subscribe_raw(SubscriptionMethod::Candle, &p, &key)
            .await
    }

    pub async fn subscribe_to_balance(
        &self,
        account: solana_sdk::pubkey::Pubkey,
    ) -> Result<SubHandle, ExchangeError> {
        let p = Balance { account };
        let key = format!("{}:{}", SubscriptionMethod::Balance, account);
        self.subscribe_raw(SubscriptionMethod::Balance, &p, &key)
            .await
    }

    pub async fn subscribe_to_margin(
        &self,
        account: solana_sdk::pubkey::Pubkey,
    ) -> Result<SubHandle, ExchangeError> {
        let p = Margin { account };
        let key = format!("{}:{}", SubscriptionMethod::Margin, account);
        self.subscribe_raw(SubscriptionMethod::Margin, &p, &key)
            .await
    }

    pub async fn subscribe_to_leverage(
        &self,
        account: solana_sdk::pubkey::Pubkey,
    ) -> Result<SubHandle, ExchangeError> {
        let p = Leverage { account };
        let key = format!("{}:{}", SubscriptionMethod::Leverage, account);
        self.subscribe_raw(SubscriptionMethod::Leverage, &p, &key)
            .await
    }

    pub async fn subscribe_to_account_info(
        &self,
        account: solana_sdk::pubkey::Pubkey,
    ) -> Result<SubHandle, ExchangeError> {
        let p = AccountInfo { account };
        let key = format!("{}:{}", SubscriptionMethod::AccountInfo, account);
        self.subscribe_raw(SubscriptionMethod::AccountInfo, &p, &key)
            .await
    }

    pub async fn subscribe_to_positions(
        &self,
        account: solana_sdk::pubkey::Pubkey,
    ) -> Result<SubHandle, ExchangeError> {
        let p = Positions { account };
        let key = format!("{}:{}", SubscriptionMethod::Positions, account);
        self.subscribe_raw(SubscriptionMethod::Positions, &p, &key)
            .await
    }

    pub async fn subscribe_to_orders(
        &self,
        account: solana_sdk::pubkey::Pubkey,
    ) -> Result<SubHandle, ExchangeError> {
        let p = Orders { account };
        let key = format!("{}:{}", SubscriptionMethod::Orders, account);
        self.subscribe_raw(SubscriptionMethod::Orders, &p, &key)
            .await
    }

    pub async fn subscribe_to_order_updates(
        &self,
        account: solana_sdk::pubkey::Pubkey,
    ) -> Result<SubHandle, ExchangeError> {
        let p = OrderUpdates { account };
        let key = format!("{}:{}", SubscriptionMethod::AccountOrderUpdates, account);
        self.subscribe_raw(SubscriptionMethod::AccountOrderUpdates, &p, &key)
            .await
    }

    pub async fn subscribe_to_account_trades(
        &self,
        account: solana_sdk::pubkey::Pubkey,
    ) -> Result<SubHandle, ExchangeError> {
        let p = AccountTrades { account };
        let key = format!("{}:{}", SubscriptionMethod::AccountTrades, account);
        self.subscribe_raw(SubscriptionMethod::AccountTrades, &p, &key)
            .await
    }

    /// Generic subscribe when you already have params + a chan_key you want to use.
    pub async fn subscribe<P: Serialize + Clone>(
        &self,
        params: WebSocketParams<P>,
        chan_key: &str,
    ) -> Result<SubHandle, ExchangeError> {
        let remote_channel = params.source.clone();
        let params_value = serde_json::to_value(&params.params)?;
        let stored: WebSocketParams<Value> = WebSocketParams {
            source: remote_channel.clone(),
            params: params_value,
        };

        // Remember for resubscribe
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(chan_key.to_string(), stored.clone());

        // Send subscribe
        let sub = json!({
            "method": WsMethod::Subscribe,
            "params": stored,
        })
        .to_string();

        self.0.send_text(sub).await?;

        // Get or create broadcast sender for this remote channel
        let mut subs = self.0.subscribers.lock().await;
        let rx = match subs.get(&remote_channel) {
            Some(tx) => tx.subscribe(),
            None => {
                let (tx, rx) = broadcast::channel(32);
                subs.insert(remote_channel.clone(), tx);
                rx
            }
        };
        drop(subs);

        Ok(SubHandle {
            client: self.clone(),
            chan_key: chan_key.to_string(),
            rx,
        })
    }

    /// Unsubscribe by channel key (used by RAII handle on Drop).
    pub async fn unsubscribe(&self, chan_key: &str) -> Result<(), ExchangeError> {
        let mut active = self.0.active_subscriptions.lock().await;
        if let Some(params) = active.remove(chan_key) {
            let unsub = json!({
                "method": WsMethod::Unsubscribe,
                "params": params,
            })
            .to_string();
            self.0.send_text(unsub).await?;
        }
        Ok(())
    }

    // =========================================================================
    // Internals
    // =========================================================================

    async fn subscribe_raw<P: Serialize>(
        &self,
        method: SubscriptionMethod,
        p: &P,
        chan_key: &str,
    ) -> Result<SubHandle, ExchangeError> {
        let params = WebSocketParams {
            source: method.to_string(),
            params: p,
        };
        self.subscribe(params, chan_key).await
    }
}

impl Inner {
    /// Main run loop: connect, read, reconnect with backoff until closed.
    async fn run(&self) {
        debug!("WebSocketClient run loop started");

        let mut backoff_secs = 1u64;
        const MAX_BACKOFF: u64 = 30;

        while !self.closed.load(Ordering::Relaxed) {
            match self.connect_and_loop().await {
                Ok(_) => {
                    info!("WebSocket connection loop exited normally");
                    break;
                }
                Err(e) => {
                    warn!("WebSocket connection error: {:?}", e);
                }
            }

            if self.closed.load(Ordering::Relaxed) {
                break;
            }

            // Exponential backoff (with cap)
            debug!("Reconnecting in {}s", backoff_secs);
            sleep(Duration::from_secs(backoff_secs)).await;
            backoff_secs = (backoff_secs * 2).min(MAX_BACKOFF);
        }

        debug!("WebSocketClient run loop finished");
    }

    async fn connect_and_loop(&self) -> Result<(), ExchangeError> {
        debug!("Connecting to WebSocket: {}", self.url);
        let ws_url = &self.url;
        let mut req = ws_url.into_client_request()?;

        if let Some(ref api_key) = self.api_key {
            req.headers_mut().insert("PF-API-KEY", api_key.parse()?);
        }

        let (ws_stream, _resp) = connect_async(req).await?;
        debug!("WebSocket connection established");

        let (sink, stream) = ws_stream.split();
        {
            let mut write_guard = self.write.lock().await;
            *write_guard = Some(sink);
        }

        // Ping task using WS Ping frames
        let ping_self = self.clone();
        let ping_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                if ping_self.closed.load(Ordering::Relaxed) {
                    debug!("Ping task noticed closed flag, exiting");
                    break;
                }
                if let Err(e) = ping_self.send_message(Message::Ping(Bytes::new())).await {
                    debug!("Ping send error: {:?}", e);
                }
            }
        });

        // Resubscribe to active subscriptions
        let snapshot = self.active_subscriptions.lock().await.clone();
        debug!("Resubscribing to {} active subscriptions", snapshot.len());
        for (chan, params) in snapshot {
            let sub = json!({
                "method": WsMethod::Subscribe,
                "params": params,
            })
            .to_string();
            if let Err(e) = self.send_text(sub).await {
                warn!("Failed to resubscribe to {}: {:?}", chan, e);
            }
        }

        // Read loop
        let res = self.receive_loop(stream).await;

        debug!("WebSocket receive loop exited, aborting ping task");
        {
            let mut write_guard = self.write.lock().await;
            *write_guard = None;
        }
        ping_handle.abort();

        res
    }

    async fn receive_loop(
        &self,
        mut stream: futures_util::stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) -> Result<(), ExchangeError> {
        while let Some(msg_res) = stream.next().await {
            let msg = match msg_res {
                Ok(m) => m,
                Err(e) => {
                    debug!("WebSocket read error: {:?}", e);
                    return Err(ExchangeError::WebSocket(Box::new(e)));
                }
            };

            match msg {
                Message::Text(text) => {
                    debug!("WS text: {}", text);
                    let value: Value = match serde_json::from_str(&text) {
                        Ok(v) => v,
                        Err(e) => {
                            debug!("Failed to parse WS JSON: {} | raw: {}", e, text);
                            continue;
                        }
                    };

                    // App-level pong
                    if value.get("channel") == Some(&json!("pong")) {
                        let _ = self.bus_tx.send(WsEvent::Pong);
                        continue;
                    }

                    // Request/response
                    if let Some(id) = value.get("id").and_then(|v| v.as_str()) {
                        {
                            let mut pending = self.pending_requests.lock().await;
                            if let Some(tx) = pending.remove(id) {
                                let _ = tx.send(value.clone());
                            }
                        }
                        let _ = self.bus_tx.send(WsEvent::Response {
                            id: id.to_string(),
                            payload: value,
                        });
                        continue;
                    }

                    // Error (maybe with id)
                    if value.get("code").is_some() && value.get("err").is_some() {
                        let id = value
                            .get("id")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let _ = self.bus_tx.send(WsEvent::Error { id, payload: value });
                        continue;
                    }

                    // Channel push
                    if let Some(channel) = value.get("channel").and_then(|v| v.as_str()) {
                        // Fan out to per-channel subscribers for backward compatibility
                        let subs = self.subscribers.lock().await;
                        if let Some(tx) = subs.get(channel) {
                            let _ = tx.send(value.clone());
                        }

                        let _ = self.bus_tx.send(WsEvent::Channel {
                            channel: channel.to_string(),
                            payload: value,
                        });
                        continue;
                    }

                    // Unknown payload shape — still forward if desired
                    let _ = self.bus_tx.send(WsEvent::Channel {
                        channel: "__unknown__".into(),
                        payload: value,
                    });
                }
                Message::Ping(data) => {
                    debug!("Received WS ping");
                    self.send_message(Message::Pong(data)).await.ok();
                }
                Message::Close(frame) => {
                    info!("Received WS close: {:?}", frame);
                    let _ = self.bus_tx.send(WsEvent::Closed);
                    break;
                }
                other => {
                    debug!("WS other message: {:?}", other);
                }
            }
        }
        Ok(())
    }

    async fn send_text(&self, text: String) -> Result<(), ExchangeError> {
        self.wait_for_write(Duration::from_secs(5)).await?;
        let msg = Message::Text(Utf8Bytes::from(text));
        self.send_message(msg).await
    }

    async fn send_message(&self, msg: Message) -> Result<(), ExchangeError> {
        debug!("Sending WS message: {:?}", msg);
        let mut guard = self.write.lock().await;
        if let Some(sink) = guard.as_mut() {
            sink.send(msg).await.map_err(|e| {
                error!("WebSocket send error: {:?}", e);
                ExchangeError::WebSocket(Box::new(e))
            })
        } else {
            debug!("send_message: write is None => WebSocketLostConnection");
            Err(ExchangeError::WebSocketLostConnection)
        }
    }

    async fn wait_for_write(&self, timeout: Duration) -> Result<(), ExchangeError> {
        use std::time::Instant;
        let start = Instant::now();
        loop {
            {
                let guard = self.write.lock().await;
                if guard.is_some() {
                    return Ok(());
                }
            }
            if start.elapsed() >= timeout {
                return Err(ExchangeError::WebSocketLostConnection);
            }
            sleep(Duration::from_millis(50)).await;
        }
    }
}

impl Clone for Inner {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            api_key: self.api_key.clone(),
            write: self.write.clone(),
            bus_tx: self.bus_tx.clone(),
            subscribers: self.subscribers.clone(),
            active_subscriptions: self.active_subscriptions.clone(),
            pending_requests: self.pending_requests.clone(),
            closed: self.closed.clone(),
        }
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn agg_suffix(agg: Option<crate::common::types::AggLevel>) -> String {
    match agg {
        Some(a) => format!(":{:?}", a),
        None => String::new(),
    }
}
