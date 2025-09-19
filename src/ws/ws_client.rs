use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use tracing::error;
use serde::Serialize;
use serde_json::{Value, json};
use solana_sdk::pubkey::Pubkey;
use tokio::{
    net::TcpStream,
    sync::{Mutex, mpsc},
};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{
        Utf8Bytes, client::IntoClientRequest, protocol::Message,
    },
};
use tracing::debug;
use uuid::Uuid;

use crate::{
    common::{
        errors::ExchangeError,
        types::{
            AggLevel, FinalRequest, Interval, WebSocketParams, WebSocketRequest,
            WebSocketSubscription, WsMethod,
        },
    },
    models::{
        ws::subscriptions::{
            AccountInfo, AccountTrades, Balance, Candle, Leverage, Margin, OrderBook, OrderUpdates,
            Orders, Positions, Prices, SubscriptionMethod, Trades,
        },
    },
};

#[derive(Clone)]
pub struct WebSocketClient(Arc<Inner>);

#[derive(Clone)]
struct Inner {
    url: String,
    api_key: Option<String>,
    write: Arc<Mutex<Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>>,
    subscribers: Arc<Mutex<HashMap<String, mpsc::Sender<Value>>>>,
    active_subscriptions: Arc<Mutex<HashMap<String, Value>>>,
    pending_requests: Arc<Mutex<HashMap<String, mpsc::Sender<Value>>>>,
    closed: Arc<AtomicBool>,
}

impl std::ops::Deref for Inner {
    type Target = Mutex<
        Option<
            futures_util::stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        >,
    >;

    fn deref(&self) -> &Self::Target {
        &self.write
    }
}

impl WebSocketClient {
    pub async fn new(url: &str, api_key: Option<String>) -> Result<Self, ExchangeError> {
        let inner = Arc::new(Inner {
            url: url.to_string(),
            api_key,
            write: Arc::new(Mutex::new(None)),
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

    pub async fn subscribe_to_prices(&self) -> Result<Subscription, ExchangeError> {
        let p = Prices {};
        let params = WebSocketParams {
            source: SubscriptionMethod::Prices.to_string(),
            params: p,
        };
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params,
        };
        let channel = SubscriptionMethod::Prices.to_string();
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0.subscribers.lock().await.insert(channel.clone(), tx);
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(channel.clone(), serde_json::to_value(&sub.params)?);
        Ok(Subscription {
            client: self.clone(),
            channel,
            rx,
        })
    }

    pub async fn subscribe_to_orderbook(
        &self,
        symbol: &str,
        agg_level: AggLevel,
    ) -> Result<Subscription, ExchangeError> {
        let p = OrderBook {
            symbol: symbol.to_string(),
            agg_level,
        };
        let params = WebSocketParams {
            source: SubscriptionMethod::Book.to_string(),
            params: p,
        };
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params,
        };
        let channel = SubscriptionMethod::Book.to_string();
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0.subscribers.lock().await.insert(channel.clone(), tx);
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(channel.clone(), serde_json::to_value(&sub.params)?);
        Ok(Subscription {
            client: self.clone(),
            channel,
            rx,
        })
    }

    pub async fn subscribe_to_trades(&self, symbol: &str) -> Result<Subscription, ExchangeError> {
        let p = Trades {
            symbol: symbol.to_string(),
        };
        let params = WebSocketParams {
            source: SubscriptionMethod::Trades.to_string(),
            params: p,
        };
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params,
        };
        let channel = SubscriptionMethod::Trades.to_string();
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0.subscribers.lock().await.insert(channel.clone(), tx);
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(channel.clone(), serde_json::to_value(&sub.params)?);
        Ok(Subscription {
            client: self.clone(),
            channel,
            rx,
        })
    }

    pub async fn subscribe_to_candle(
        &self,
        symbol: &str,
        interval: Interval,
    ) -> Result<Subscription, ExchangeError> {
        let p = Candle {
            symbol: symbol.to_string(),
            interval,
        };
        let params = WebSocketParams {
            source: SubscriptionMethod::Candle.to_string(),
            params: p,
        };
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params,
        };
        let channel = SubscriptionMethod::Candle.to_string();
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0.subscribers.lock().await.insert(channel.clone(), tx);
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(channel.clone(), serde_json::to_value(&sub.params)?);
        Ok(Subscription {
            client: self.clone(),
            channel,
            rx,
        })
    }

    pub async fn subscribe_to_balance(
        &self,
        account: Pubkey,
    ) -> Result<Subscription, ExchangeError> {
        let p = Balance { account };
        let params = WebSocketParams {
            source: SubscriptionMethod::Balance.to_string(),
            params: p,
        };
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params,
        };
        let channel = SubscriptionMethod::Balance.to_string();
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0.subscribers.lock().await.insert(channel.clone(), tx);
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(channel.clone(), serde_json::to_value(&sub.params)?);
        Ok(Subscription {
            client: self.clone(),
            channel,
            rx,
        })
    }

    pub async fn subscribe_to_margin(
        &self,
        account: Pubkey,
    ) -> Result<Subscription, ExchangeError> {
        let p = Margin { account };
        let params = WebSocketParams {
            source: SubscriptionMethod::Margin.to_string(),
            params: p,
        };
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params,
        };
        let channel = SubscriptionMethod::Margin.to_string();
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0.subscribers.lock().await.insert(channel.clone(), tx);
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(channel.clone(), serde_json::to_value(&sub.params)?);
        Ok(Subscription {
            client: self.clone(),
            channel,
            rx,
        })
    }

    pub async fn subscribe_to_leverage(
        &self,
        account: Pubkey,
    ) -> Result<Subscription, ExchangeError> {
        let p = Leverage { account };
        let params = WebSocketParams {
            source: SubscriptionMethod::Leverage.to_string(),
            params: p,
        };
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params,
        };
        let channel = SubscriptionMethod::Leverage.to_string();
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0.subscribers.lock().await.insert(channel.clone(), tx);
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(channel.clone(), serde_json::to_value(&sub.params)?);
        Ok(Subscription {
            client: self.clone(),
            channel,
            rx,
        })
    }

    pub async fn subscribe_to_account_info(
        &self,
        account: Pubkey,
    ) -> Result<Subscription, ExchangeError> {
        let p = AccountInfo { account };
        let params = WebSocketParams {
            source: SubscriptionMethod::AccountInfo.to_string(),
            params: p,
        };
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params,
        };
        let channel = SubscriptionMethod::AccountInfo.to_string();
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0.subscribers.lock().await.insert(channel.clone(), tx);
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(channel.clone(), serde_json::to_value(&sub.params)?);
        Ok(Subscription {
            client: self.clone(),
            channel,
            rx,
        })
    }

    pub async fn subscribe_to_positions(
        &self,
        account: Pubkey,
    ) -> Result<Subscription, ExchangeError> {
        let p = Positions { account };
        let params = WebSocketParams {
            source: SubscriptionMethod::Positions.to_string(),
            params: p,
        };
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params,
        };
        let channel = SubscriptionMethod::Positions.to_string();
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0.subscribers.lock().await.insert(channel.clone(), tx);
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(channel.clone(), serde_json::to_value(&sub.params)?);
        Ok(Subscription {
            client: self.clone(),
            channel,
            rx,
        })
    }

    pub async fn subscribe_to_orders(
        &self,
        account: Pubkey,
    ) -> Result<Subscription, ExchangeError> {
        let p = Orders { account };
        let params = WebSocketParams {
            source: SubscriptionMethod::Orders.to_string(),
            params: p,
        };
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params,
        };
        let channel = SubscriptionMethod::Orders.to_string();
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0.subscribers.lock().await.insert(channel.clone(), tx);
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(channel.clone(), serde_json::to_value(&sub.params)?);
        Ok(Subscription {
            client: self.clone(),
            channel,
            rx,
        })
    }

    pub async fn subscribe_to_order_updates(
        &self,
        account: Pubkey,
    ) -> Result<Subscription, ExchangeError> {
        let p = OrderUpdates { account };
        let params = WebSocketParams {
            source: SubscriptionMethod::AccountOrderUpdates.to_string(),
            params: p,
        };
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params,
        };
        let channel = SubscriptionMethod::AccountOrderUpdates.to_string();
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0.subscribers.lock().await.insert(channel.clone(), tx);
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(channel.clone(), serde_json::to_value(&sub.params)?);
        Ok(Subscription {
            client: self.clone(),
            channel,
            rx,
        })
    }

    pub async fn subscribe_to_account_trades(
        &self,
        account: Pubkey,
    ) -> Result<Subscription, ExchangeError> {
        let p = AccountTrades { account };
        let params = WebSocketParams {
            source: SubscriptionMethod::AccountTrades.to_string(),
            params: p,
        };
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params,
        };
        let channel = SubscriptionMethod::AccountTrades.to_string();
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0.subscribers.lock().await.insert(channel.clone(), tx);
        self.0
            .active_subscriptions
            .lock()
            .await
            .insert(channel.clone(), serde_json::to_value(&sub.params)?);
        Ok(Subscription {
            client: self.clone(),
            channel,
            rx,
        })
    }

    pub async fn unsubscribe(&self, channel: &str) -> Result<(), ExchangeError> {
        let mut active = self.0.active_subscriptions.lock().await;
        if let Some(params_value) = active.remove(channel) {
            let params: WebSocketParams<Value> = serde_json::from_value(params_value)?;
            let unsub = WebSocketSubscription {
                method: WsMethod::Unsubscribe,
                params,
            };
            self.0.send(serde_json::to_string(&unsub)?).await?;
        }
        self.0.subscribers.lock().await.remove(channel);
        Ok(())
    }

    pub async fn subscribe<P: Serialize + Clone>(
        &self,
        params: WebSocketParams<P>,
        channel: &str,
    ) -> Result<mpsc::Receiver<Value>, ExchangeError> {
        let mut active = self.0.active_subscriptions.lock().await;
        active.insert(channel.to_string(), json!(params));
        drop(active);
        let sub = WebSocketSubscription {
            method: WsMethod::Subscribe,
            params: params.clone(),
        };
        self.0.send(serde_json::to_string(&sub)?).await?;
        let (tx, rx) = mpsc::channel(32);
        self.0
            .subscribers
            .lock()
            .await
            .insert(channel.to_string(), tx);
        Ok(rx)
    }

    pub async fn close(&self) -> Result<(), ExchangeError> {
        self.0.closed.store(true, Ordering::Relaxed);
        let mut guard = self.0.write.lock().await;
        if let Some(sink) = guard.as_mut() {
            sink.send(Message::Close(None)).await?;
        }
        Ok(())
    }

    pub async fn send_exchange_request<P: Serialize>(
        &self,
        request_id: Option<Uuid>,
        request_method: &str,
        request: FinalRequest<P>,
    ) -> Result<mpsc::Receiver<Value>, ExchangeError> {
        let request_id = request_id.unwrap_or(Uuid::new_v4());
        let mut params = HashMap::new();
        params.insert(request_method.to_string(), request);
        let web_socket_request: WebSocketRequest<P> = WebSocketRequest {
            id: request_id.clone(),
            params,
        };
        let msg = json!(web_socket_request).to_string();
        self.0.send(msg.clone()).await?;
        let (tx, rx) = mpsc::channel(1);
        self.0
            .pending_requests
            .lock()
            .await
            .insert(request_id.to_string(), tx);

        Ok(rx)
    }

    // For future: Type () is required for T
    pub(crate) async fn set_api_key(&self, api_key: String) -> Result<String, ExchangeError> { 
         Ok(api_key)
    }
}


pub struct Subscription {
    client: WebSocketClient,
    channel: String,
    rx: mpsc::Receiver<Value>,
}

impl Subscription {
    pub async fn recv(&mut self) -> Option<Value> {
        self.rx.recv().await
    }

    pub fn attach_callback<F>(mut self, callback: F)
    where
        F: FnMut(Value) + Send + 'static,
    {
        tokio::spawn(async move {
            let mut cb = callback;
            while let Some(event) = self.rx.recv().await {
                cb(event);
            }
        });
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        let client = self.client.clone();
        let channel = self.channel.clone();
        tokio::spawn(async move {
            client.unsubscribe(&channel).await.ok();
        });
    }
}

impl Inner {
    async fn run(&self) {
        debug!("WebSocketClient run loop started");
        loop {
            if self.closed.load(Ordering::Relaxed) {
                debug!("WebSocketClient closed, exiting run loop");
                break;
            }
            match self.connect_and_loop().await {
                Ok(_) => debug!("WebSocketClient connection loop exited normally"),
                Err(e) => tracing::error!("WebSocketClient connection error: {:?}", e),
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
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
        let (ws_stream, _) = connect_async(req).await?;
        debug!("WebSocket connection established");
        let (sink, stream) = ws_stream.split();
        {
            let mut write_guard = self.write.lock().await;
            *write_guard = Some(sink);
        }
        let self_clone = self.clone();
        let ping_handle = tokio::spawn(async move {
            loop {
                if self_clone.closed.load(Ordering::Relaxed) {
                    debug!("Ping task noticed closed flag, exiting");
                    break;
                }
                tokio::time::sleep(Duration::from_secs(50)).await;
                debug!("Sending ping to WebSocket");
                if let Err(e) = self_clone.send(json!({"method":"ping"}).to_string()).await {
                    debug!("Ping send error (will continue): {:?}", e);
                }
            }
        });

        let active = self.active_subscriptions.lock().await.clone();
        debug!("Resubscribing to {} active subscriptions", active.len());
        for (chan, params) in active.iter() {
            debug!("Resubscribing to {} with params: {:?}", chan, params);
            match serde_json::from_value::<WebSocketParams<Value>>(params.clone()) {
                Ok(deser_params) => {
                    let sub = WebSocketSubscription {
                        method: WsMethod::Subscribe,
                        params: deser_params,
                    };
                    if let Err(e) = self.send(json!(sub).to_string()).await {
                        debug!("Failed to resubscribe to {}: {:?}", chan, e);
                    }
                }
                Err(e) => {
                    debug!(
                        "Cannot deserialize stored subscription for {}: {:?}",
                        chan, e
                    );
                }
            }
        }

        let res = self.receive_loop(stream).await;

        debug!("WebSocket receive loop exited, aborting ping task");

        {
            let mut write_guard = self.write.lock().await;
            *write_guard = None;
        }

        ping_handle.abort();

        res
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
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    async fn send(&self, text: String) -> Result<(), ExchangeError> {
        let _ = self.wait_for_write(Duration::from_secs(5)).await;

        let msg = Message::Text(Utf8Bytes::from(text));

        match self.send_message(msg.clone()).await {
            Ok(_) => Ok(()),
            Err(e) if matches!(e, ExchangeError::WebSocketLostConnection) => {
                debug!(
                    "send: write missing or lost, will return error (run loop should reconnect)"
                );
                Err(e)
            }
            Err(e) => Err(e),
        }
    }

    async fn send_message(&self, msg: Message) -> Result<(), ExchangeError> {
        debug!("Sending WS message: {:?}", msg);
        let mut guard = self.write.lock().await;
        if let Some(sink) = guard.as_mut() {
            match sink.send(msg).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    error!("WebSocket send error: {:?}", e);
                    Err(ExchangeError::WebSocket(e))
                }
            }
        } else {
            debug!("send_message: write is None => WebSocketLostConnection");
            Err(ExchangeError::WebSocketLostConnection)
        }
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
                    return Err(ExchangeError::WebSocket(e));
                }
            };
            debug!("Received WS message: {:?}", msg);
            match msg {
                Message::Text(text) => {
                    debug!("Received WS text: {}", text);
                    let value_res = serde_json::from_str::<Value>(&text);
                    let value = match value_res {
                        Ok(v) => v,
                        Err(e) => {
                            debug!("Failed to parse WS JSON: {} | raw: {}", e, text);
                            continue;
                        }
                    };

                    if value.get("channel") == Some(&json!("pong")) {
                        debug!("Received pong message");
                        continue;
                    }

                    if let Some(id) = value.get("id").and_then(|v| v.as_str()) {
                        debug!("Received response for request id: {}", id);
                        let mut pending = self.pending_requests.lock().await;
                        if let Some(tx) = pending.remove(id) {
                            tx.send(value.clone()).await.ok();
                        } else {
                            debug!("No pending requester for id {}", id);
                        }
                        continue;
                    }

                    if value.get("code").is_some() && value.get("err").is_some() {
                        if let Some(id) = value.get("id").and_then(|v| v.as_str()) {
                            debug!("Received error response for id {}: {}", id, value);
                            let mut pending = self.pending_requests.lock().await;
                            if let Some(tx) = pending.remove(id) {
                                tx.send(value.clone()).await.ok();
                                continue;
                            }
                        }

                        let subs = self.subscribers.lock().await;
                        if let Some(err_tx) = subs.get("__errors__") {
                            debug!(
                                "Forwarding server error to __errors__ subscriber: {}",
                                value
                            );
                            err_tx.send(value.clone()).await.ok();
                            continue;
                        }

                        debug!("Server error (no id, no __errors__ subscriber): {}", value);
                        continue;
                    }

                    if let Some(channel) = value.get("channel").and_then(|v| v.as_str()) {
                        debug!("Received message for channel: {}", channel);
                        let subs = self.subscribers.lock().await;
                        if let Some(tx) = subs.get(channel) {
                            tx.send(value.clone()).await.ok();
                        } else {
                            debug!("No subscriber for channel {}", channel);
                        }
                        continue;
                    }

                    debug!("Received unrecognized WS JSON: {}", value);
                }
                Message::Ping(data) => {
                    debug!("Received WS ping");
                    self.send_message(Message::Pong(data)).await.ok();
                }
                Message::Close(frame) => {
                    debug!("Received WS close: {:?}", frame);
                    break;
                }
                other => {
                    debug!("Received WS other message: {:?}", other);
                }
            }
        }
        Ok(())
    }
}
