use std::{env, str::FromStr};

use dotenvy::from_filename;
use pacifica_rust_sdk::{
    common::types::OrderSide, exchange::exchange_client::ExchangeClient,
    logging::init_logging_once, models::exchange::payload::order::CreateMarketOrderPayload,
};
use tracing::info;

use rust_decimal::Decimal;
use solana_sdk::signature::{Keypair, Signer};
use tracing::error;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    from_filename("src/bin/EXAMPLE.env").ok();
    init_logging_once("debug");
    let is_mainnet = false;
    let enable_ws = true;
    let api_key: Option<String> = None;
    let main_keypair = Keypair::from_base58_string(env::var("TEST_KEY").unwrap().as_str());
    let main_pubkey = main_keypair.pubkey();

    let client = ExchangeClient::new(
        is_mainnet,
        enable_ws,
        api_key.clone(),
        main_keypair,
        main_pubkey,
        None,
    )
    .await
    .map_err(|e| format!("failed to init client: {:?}", e))
    .unwrap();

    info!("Using main account: {}", main_pubkey);

    // That helps to normalize price/amount and has other same utils
    let tick_lot_utils = client.info_client.tick_lot_utils.clone();

    // entry params
    let symbol: &'static str = "BTC";
    let expiry_window: Option<u32> = None;
    let amount_raw: Decimal = Decimal::from_str("0.00123456").unwrap();
    let normalized_amount: Decimal = tick_lot_utils.normalize_amount(symbol, amount_raw).unwrap();

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
    match client.market_order(sign_payload, expiry_window).await {
        Ok(order_response) => {
            if let Some(data) = order_response.data {
                info!("Market Order placed successfully: {:?}", &data.order_id);
                Some(data.order_id)
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
}
