use std::{env, str::FromStr};

use dotenvy::from_filename;
use tracing::info;
use pacifica_rust_sdk::{
    common::{
        types::OrderSide,
    },
    exchange::exchange_client::ExchangeClient,
    logging::init_logging_once,
    models::exchange::{
        payload::order::{CreateStopOrderPayload, TpSlAlonePayload},
        },
};

use rust_decimal::Decimal;
use solana_sdk::signature::{Keypair, Signer};

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

    let expiry_window: Option<u32> = None;

    // entry params
    let symbol = "BTC";
    let limit_price_raw: Decimal = Decimal::from_str("110000.123456").unwrap();
    let normalized_limit_price: Decimal = tick_lot_utils
        .normalize_price(symbol, limit_price_raw)
        .unwrap();

    let stop_price_raw: Decimal = Decimal::from_str("109500.123456").unwrap();
    let normalized_stop_price: Decimal = tick_lot_utils
        .normalize_price(symbol, stop_price_raw)
        .unwrap();

    let amount_raw: Decimal = Decimal::from_str("0.00123456").unwrap();
    let normalized_amount: Decimal = tick_lot_utils.normalize_amount(symbol, amount_raw).unwrap();

    let sign_payload = CreateStopOrderPayload {
        symbol: symbol.to_string(),
        side: OrderSide::Ask,
        reduce_only: true,
        stop_order: TpSlAlonePayload {
            stop_price: normalized_stop_price,
            limit_price: Some(normalized_limit_price),
            client_order_id: None,
            amount: normalized_amount,
        },
    };

    // Create Stop Order
    let response = client
        .stop_order(sign_payload, expiry_window)
        .await
        .map_err(|e| format!("failed to create stop order: {:?}", e))
        .unwrap();

    if response.success == Some(true) {
        info!(
            "Successfully created stop order: {:?}",
            response.data.unwrap().order_id
        );
    } else {
        info!("Failed to create stop order: {:?}", response);
    }
}
