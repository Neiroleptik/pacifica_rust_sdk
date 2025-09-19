use std::{env, str::FromStr};

use dotenvy::from_filename;
use tracing::info;
use pacifica_rust_sdk::{
    common::types::{ OrderSide, Tif},
    exchange::exchange_client::ExchangeClient,
    logging::init_logging_once,
    models::{
        exchange::{
            payload::{
                batch_order::BatchOrderActionPayload,
                order::{CancelOrderPayload, CreateMarketOrderPayload, CreateOrderPayload},
            },
            response::{
                batch_order::BatchOrderModel,
            },
        },
    },
};
use rust_decimal::Decimal;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use tracing::error;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    from_filename("src/bin/EXAMPLE.env").ok();
    init_logging_once("debug");
    let is_mainnet = false;
    let enable_ws = true;
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

    info!("Using main account: {}", main_pubkey);

    let expiry_window: Option<u32> = None;

    // That helps to normalize price/amount and has other same utils
    let tick_lot_utils = client.info_client.tick_lot_utils.clone();

    // entry params
    let symbol: &'static str = "BTC";
    let price_raw: Decimal = Decimal::from_str("110000.123456").unwrap();
    let normalized_price: Decimal = tick_lot_utils.normalize_price(symbol, price_raw).unwrap();
    let amount_raw: Decimal = Decimal::from_str("0.00123456").unwrap();
    let normalized_amount: Decimal = tick_lot_utils.normalize_amount(symbol, amount_raw).unwrap();
    let mut batch_sign_payload: Vec<BatchOrderActionPayload> = Vec::new();

    // Batch Order accepts operations: CancelOrder, CreateMarketOrder, CreateOrder
    // 1. Create a order payload
    let limit_sign_payload = BatchOrderActionPayload::CreateOrder(CreateOrderPayload {
        symbol: symbol.to_string(),
        price: normalized_price,
        amount: normalized_amount,
        side: OrderSide::Bid,
        tif: Tif::ALO,
        reduce_only: false,
        client_order_id: None,
        take_profit: None,
        stop_loss: None,
    });

    batch_sign_payload.push(limit_sign_payload);

    // 2. Create a market order payload
    let cloid = Uuid::new_v4();
    let market_sign_payload =
        BatchOrderActionPayload::CreateMarketOrder(CreateMarketOrderPayload {
            symbol: symbol.to_string(),
            amount: normalized_amount,
            side: OrderSide::Bid,
            slippage_percent: Decimal::from_str("0.5").unwrap(),
            reduce_only: false,
            client_order_id: Some(cloid),
            take_profit: None,
            stop_loss: None,
        });

    batch_sign_payload.push(market_sign_payload);

    // 3. Cancel order
    let cloid_to_cancel = cloid;
    info!("Cancel order by cloid: {0}", &cloid_to_cancel);
    // One of order_id or client_order_id is required
    let cancel_sign_payload = BatchOrderActionPayload::CancelOrder(CancelOrderPayload {
        symbol: symbol.to_string(),
        order_id: None,
        client_order_id: Some(cloid_to_cancel),
    });

    batch_sign_payload.push(cancel_sign_payload);

    let results: Option<Vec<BatchOrderModel>> =
        match client.batch_order(batch_sign_payload, expiry_window).await {
            Ok(batch_response) => {
                if let Some(data) = batch_response.data {
                    info!("Batch placed successfully");
                    Some(data.results)
                } else {
                    error!("Order response had no data");
                    None
                }
            }
            Err(e) => {
                error!("Failed to place order: {:?}", e);
                None
            }
        };

    if results.is_some() {
        for result in results.unwrap() {
            info!("Operation Response: {:?}", result)
        }
    }
}
