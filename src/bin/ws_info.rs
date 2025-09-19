use std::str::FromStr;

use dotenvy::from_filename;
use pacifica_rust_sdk::{
    common::{
        types::{AggLevel, DefaultWebSocketMsg, Interval},
    },
    info::info_client::InfoClient,
    logging::init_logging_once,
    models::{
        ws::responses::{
            AccountInfoResponse, AccountTradesResponse, BalanceResponse, CandleResponse,
            LeverageResponse, MarginResponse, OrderBookResponse, OrderUpdatesResponse,
            OrdersResponse, PositionsResponse, PricesResponse, TradesResponse,
        },
    },
};

use solana_sdk::pubkey::Pubkey;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    from_filename("src/bin/EXAMPLE.env").ok();
    init_logging_once("debug");

    let is_mainnet = false;
    let enable_ws = true;
    let api_key: Option<String> = Some("YOUR_API_KEY_HERE".to_string());
    let client = InfoClient::new(is_mainnet, enable_ws, api_key.clone())
        .await
        .map_err(|e| format!("failed to init client: {:?}", e))
        .unwrap();

    info!("InfoClient initialized");

    // Test Pubkey
    let account: Pubkey = Pubkey::from_str("94HjQxftTdgKkTLGVEQm11pNr1A8RNSQij2M1fZpdyfn").unwrap();
    let symbol = "BTC";

    // WebSocket subscriptions
    // response has fields : channel, data, ts; Can use as: response, response.channel, response.ts, response.data
    // 1. Prices
    let sub_prices_result = match client.web_socket_client.as_ref() {
        Some(ws_client) => ws_client.subscribe_to_prices().await,
        None => Err("WebSocket client not initialized".into()),
    };

    match sub_prices_result {
        Ok(sub) => {
            sub.attach_callback(|data: serde_json::Value| {
                match serde_json::from_value::<DefaultWebSocketMsg<PricesResponse>>(data.clone()) {
                    Ok(response) => {
                        info!("Price update (callback): {:?}", response);
                    }
                    Err(e) => {
                        error!("Failed to deserialize Prices: {:?} :data {:?}", e, data);
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to subscribe to Prices: {:?}", e);
            return;
        }
    }

    // 2. Trades
    let sub_trades_result = match client.web_socket_client.as_ref() {
        Some(ws_client) => ws_client.subscribe_to_trades(symbol).await,
        None => Err("WebSocket client not initialized".into()),
    };

    match sub_trades_result {
        Ok(sub) => {
            sub.attach_callback(|data: serde_json::Value| {
                match serde_json::from_value::<DefaultWebSocketMsg<TradesResponse>>(data.clone()) {
                    Ok(response) => {
                        info!("Trades (callback): {:?}", response);
                    }
                    Err(e) => {
                        error!("Failed to deserialize Trades: {:?} :data {:?}", e, data);
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to subscribe to Trades: {:?}", e);
            return;
        }
    }

    // 3. Order Updates
    let sub_ou_result = match client.web_socket_client.as_ref() {
        Some(ws_client) => ws_client.subscribe_to_order_updates(account).await,
        None => Err("WebSocket client not initialized".into()),
    };

    match sub_ou_result {
        Ok(sub) => {
            sub.attach_callback(|data: serde_json::Value| {
                match serde_json::from_value::<DefaultWebSocketMsg<OrderUpdatesResponse>>(
                    data.clone(),
                ) {
                    Ok(response) => {
                        info!("Order Updates (callback): {:?}", response);
                    }
                    Err(e) => {
                        error!(
                            "Failed to deserialize OrderUpdates: {:?} :data {:?}",
                            e, data
                        );
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to subscribe to Order Updates: {:?}", e);
            return;
        }
    }

    // 4. Account Info
    let sub_acc_info_result = match client.web_socket_client.as_ref() {
        Some(ws_client) => ws_client.subscribe_to_account_info(account).await,
        None => Err("WebSocket client not initialized".into()),
    };

    match sub_acc_info_result {
        Ok(sub) => {
            sub.attach_callback(|data: serde_json::Value| {
                match serde_json::from_value::<DefaultWebSocketMsg<AccountInfoResponse>>(
                    data.clone(),
                ) {
                    Ok(response) => {
                        info!("Account Info (callback): {:?}", response);
                    }
                    Err(e) => {
                        error!(
                            "Failed to deserialize AccountInfo: {:?} :data {:?}",
                            e, data
                        );
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to subscribe to Account Info: {:?}", e);
            return;
        }
    }

    // 5. Account Trades
    let sub_acc_trades_result = match client.web_socket_client.as_ref() {
        Some(ws_client) => ws_client.subscribe_to_account_trades(account).await,
        None => Err("WebSocket client not initialized".into()),
    };

    match sub_acc_trades_result {
        Ok(sub) => {
            sub.attach_callback(|data: serde_json::Value| {
                match serde_json::from_value::<DefaultWebSocketMsg<AccountTradesResponse>>(
                    data.clone(),
                ) {
                    Ok(response) => {
                        info!("Account Trades (callback): {:?}", response);
                    }
                    Err(e) => {
                        error!(
                            "Failed to deserialize AccountTrades: {:?} :data {:?}",
                            e, data
                        );
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to subscribe to Account Trades: {:?}", e);
            return;
        }
    }

    // 6. Balance
    let sub_balance_result = match client.web_socket_client.as_ref() {
        Some(ws_client) => ws_client.subscribe_to_balance(account).await,
        None => Err("WebSocket client not initialized".into()),
    };

    match sub_balance_result {
        Ok(sub) => {
            sub.attach_callback(|data: serde_json::Value| {
                match serde_json::from_value::<DefaultWebSocketMsg<BalanceResponse>>(data.clone()) {
                    Ok(response) => {
                        info!("Balance (callback): {:?}", response);
                    }
                    Err(e) => {
                        error!("Failed to deserialize Balance: {:?} :data {:?}", e, data);
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to subscribe to Balance: {:?}", e);
            return;
        }
    }

    // 7. Candle
    let sub_candle_result = match client.web_socket_client.as_ref() {
        Some(ws_client) => {
            ws_client
                .subscribe_to_candle(symbol, Interval::FiveMinutes)
                .await
        }
        None => Err("WebSocket client not initialized".into()),
    };

    match sub_candle_result {
        Ok(sub) => {
            sub.attach_callback(|data: serde_json::Value| {
                match serde_json::from_value::<DefaultWebSocketMsg<CandleResponse>>(data.clone()) {
                    Ok(response) => {
                        info!("Candle (callback): {:?}", response);
                    }
                    Err(e) => {
                        error!("Failed to deserialize Candle: {:?} :data {:?}", e, data);
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to subscribe to Candle: {:?}", e);
            return;
        }
    }

    // 8. Leverage
    let sub_leverage_result = match client.web_socket_client.as_ref() {
        Some(ws_client) => ws_client.subscribe_to_leverage(account).await,
        None => Err("WebSocket client not initialized".into()),
    };

    match sub_leverage_result {
        Ok(sub) => {
            sub.attach_callback(|data: serde_json::Value| {
                match serde_json::from_value::<DefaultWebSocketMsg<LeverageResponse>>(data.clone())
                {
                    Ok(response) => {
                        info!("Leverage (callback): {:?}", response.channel);
                    }
                    Err(e) => {
                        error!("Failed to deserialize Leverage: {:?} :data {:?}", e, data);
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to subscribe to Leverage: {:?}", e);
            return;
        }
    }

    // 9. Margin
    let sub_margin_result = match client.web_socket_client.as_ref() {
        Some(ws_client) => ws_client.subscribe_to_margin(account).await,
        None => Err("WebSocket client not initialized".into()),
    };

    match sub_margin_result {
        Ok(sub) => {
            sub.attach_callback(|data: serde_json::Value| {
                match serde_json::from_value::<DefaultWebSocketMsg<MarginResponse>>(data.clone()) {
                    Ok(response) => {
                        info!("Margin (callback): {:?}", response);
                    }
                    Err(e) => {
                        error!("Failed to deserialize Margin: {:?} :data {:?}", e, data);
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to subscribe to Margin: {:?}", e);
            return;
        }
    }

    // 10. OrderBook
    let sub_orderbook_result = match client.web_socket_client.as_ref() {
        Some(ws_client) => ws_client.subscribe_to_orderbook(symbol, AggLevel::L1).await,
        None => Err("WebSocket client not initialized".into()),
    };

    match sub_orderbook_result {
        Ok(sub) => {
            sub.attach_callback(|data: serde_json::Value| {
                match serde_json::from_value::<DefaultWebSocketMsg<OrderBookResponse>>(data.clone())
                {
                    Ok(response) => {
                        info!("OrderBook (callback): {:?}", response.channel);
                    }
                    Err(e) => {
                        error!("Failed to deserialize OrderBook: {:?} :data {:?}", e, data);
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to subscribe to OrderBook: {:?}", e);
            return;
        }
    }

    // 11. Orders
    let sub_orders_result = match client.web_socket_client.as_ref() {
        Some(ws_client) => ws_client.subscribe_to_orders(account).await,
        None => Err("WebSocket client not initialized".into()),
    };

    match sub_orders_result {
        Ok(sub) => {
            sub.attach_callback(|data: serde_json::Value| {
                match serde_json::from_value::<DefaultWebSocketMsg<OrdersResponse>>(data.clone()) {
                    Ok(response) => {
                        info!("Orders (callback): {:?}", response.channel);
                    }
                    Err(e) => {
                        error!("Failed to deserialize Orders: {:?} :data {:?}", e, data);
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to subscribe to Orders: {:?}", e);
            return;
        }
    }

    // 12. Positions
    let sub_positions_result = match client.web_socket_client.as_ref() {
        Some(ws_client) => ws_client.subscribe_to_positions(account).await,
        None => Err("WebSocket client not initialized".into()),
    };

    match sub_positions_result {
        Ok(sub) => {
            sub.attach_callback(|data: serde_json::Value| {
                match serde_json::from_value::<DefaultWebSocketMsg<PositionsResponse>>(data.clone())
                {
                    Ok(response) => {
                        info!("Positions (callback): {:?}", response);
                    }
                    Err(e) => {
                        error!("Failed to deserialize Positions: {:?} :data {:?}", e, data);
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to subscribe to Positions: {:?}", e);
            return;
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
}
