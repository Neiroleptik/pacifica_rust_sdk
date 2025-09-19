use std::{env, str::FromStr};

use dotenvy::from_filename;
use pacific_rust_sdk::{
    common::{
        errors::ExchangeError,
        types::{OrderSide, Tif, WebSocketOperationResponse},
    },
    exchange::exchange_client::ExchangeClient,
    logging::init_logging_once,
    models::{
        exchange::payload::order::{CancelAllOrdersPayload, CancelOrderPayload, CreateMarketOrderPayload, CreateOrderPayload},
        ws::responses::{
            CancelAllOrdersResponse, CancelOrderResponse, CreateMarketOrderResponse,
            CreateOrderResponse
        },
    },
};
use rust_decimal::Decimal;
use serde_json::from_value;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use tracing::{error, info};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    from_filename("src/bin/EXAMPLE.env").ok();
    init_logging_once("debug");
    let is_mainnet = false;
    let enable_ws = true; // Enable WebSocket
    let api_key: Option<String> = None;
    let agent_pubkey: Option<Pubkey> = None;

    let main_keypair = Keypair::from_base58_string(env::var("TEST_KEY").unwrap().as_str());
    let main_pubkey = main_keypair.pubkey();
    let client = ExchangeClient::new(
        is_mainnet,
        enable_ws,
        api_key,
        main_keypair,
        main_pubkey,
        agent_pubkey,
    )
    .await
    .map_err(|e| format!("failed to init client: {:?}", e))
    .unwrap();

    // That helps to normalize price/amount and other same utils
    let tick_lot_utils = client.info_client.tick_lot_utils.clone();

    info!("Using main account: {}", main_pubkey);

    // entry params
    let symbol: &'static str = "BTC";
    let price_raw: Decimal = Decimal::from_str("110000.123456").unwrap();
    let normalized_price: Decimal = tick_lot_utils.normalize_price(symbol, price_raw).unwrap();
    let expiry_window: Option<u32> = None;
    let amount_raw: Decimal = Decimal::from_str("0.00123456").unwrap();
    let normalized_amount: Decimal = tick_lot_utils.normalize_amount(symbol, amount_raw).unwrap();

    // Create a limit order
    let cloid = Uuid::new_v4();
    {
        let sign_payload = CreateOrderPayload {
            symbol: symbol.to_string(),
            price: normalized_price,
            amount: normalized_amount,
            side: OrderSide::Bid,
            tif: Tif::ALO,
            reduce_only: false,
            client_order_id: Some(cloid),
            take_profit: None,
            stop_loss: None,
        };

        let mut rx = client
            .request_ws_exchange_fn("create_order", sign_payload, expiry_window)
            .await
            .unwrap();

        match rx.recv().await.ok_or(ExchangeError::WebSocketSendRequest(
            "No response".to_string(),
        )) {
            Ok(order_response_value) => {
                let order_response: WebSocketOperationResponse<CreateOrderResponse> =
                    from_value(order_response_value)
                        .map_err(|e| {
                            error!("Failed to deserialize order response: {:?}", e);
                            ExchangeError::Custom(e.to_string())
                        })
                        .unwrap();
                info!("Order placed successfully: {:?}", order_response);
            }
            Err(e) => {
                error!("Failed to place order: {:?}", e);
            }
        }
    }

    // Create a market order
    {
        let cloid = Uuid::new_v4();
        let sign_payload = CreateMarketOrderPayload {
            symbol: symbol.to_string(),
            amount: normalized_amount,
            side: OrderSide::Bid,
            slippage_percent: Decimal::from_str("0.5").unwrap(),
            reduce_only: false,
            client_order_id: Some(cloid),
            take_profit: None,
            stop_loss: None,
        };

        let mut rx = client
            .request_ws_exchange_fn("create_market_order", sign_payload, expiry_window)
            .await
            .unwrap();

        match rx.recv().await.ok_or(ExchangeError::WebSocketSendRequest(
            "No response".to_string(),
        )) {
            Ok(order_response_value) => {
                let order_response: WebSocketOperationResponse<CreateMarketOrderResponse> =
                    from_value(order_response_value)
                        .map_err(|e| {
                            error!("Failed to deserialize order response: {:?}", e);
                            ExchangeError::Custom(e.to_string())
                        })
                        .unwrap();
                info!("Market order placed successfully: {:?}", order_response);
            }
            Err(e) => {
                error!("Failed to place market order: {:?}", e);
            }
        }
    }

    // Cancel order
    {
        let sign_payload = CancelOrderPayload {
            symbol: symbol.to_string(),
            order_id: None, // One of order_id or client_order_id is required
            // order_id: None,
            // client_order_id: Some(Uuid),
            client_order_id: Some(cloid),
        };

        let mut rx = client
            .request_ws_exchange_fn("cancel_order", sign_payload, expiry_window)
            .await
            .unwrap();

        match rx.recv().await.ok_or(ExchangeError::WebSocketSendRequest(
            "No response".to_string(),
        )) {
            Ok(order_response_value) => {
                let order_response: WebSocketOperationResponse<CancelOrderResponse> =
                    from_value(order_response_value)
                        .map_err(|e| {
                            error!("Failed to deserialize cancel order response: {:?}", e);
                            ExchangeError::Custom(e.to_string())
                        })
                        .unwrap();
                info!("Order canceled successfully: {:?}", order_response);
            }
            Err(e) => {
                error!("Failed to cancel order: {:?}", e);
            }
        }
    }

    // Cancel all orders
    {
        let sign_payload = CancelAllOrdersPayload {
            symbol: None,
            all_symbols: true, // if true, symbol must be None
            exclude_reduce_only: false,
        };

        let mut rx = client
            .request_ws_exchange_fn("cancel_all_orders", sign_payload, expiry_window)
            .await
            .unwrap();

        match rx.recv().await.ok_or(ExchangeError::WebSocketSendRequest(
            "No response".to_string(),
        )) {
            Ok(order_response_value) => {
                let order_response: WebSocketOperationResponse<CancelAllOrdersResponse> =
                    from_value(order_response_value)
                        .map_err(|e| {
                            error!("Failed to deserialize cancel all orders response: {:?}", e);
                            ExchangeError::Custom(e.to_string())
                        })
                        .unwrap();
                info!("All orders canceled successfully: {:?}", order_response);
            }
            Err(e) => {
                error!("Failed to cancel all orders: {:?}", e);
            }
        }
    }
}
