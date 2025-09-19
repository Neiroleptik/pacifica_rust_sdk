use std::{env, str::FromStr};

use dotenvy::from_filename;
use pacifica_rust_sdk::{
    common::types::OrderSide,
    exchange::exchange_client::ExchangeClient,
    logging::init_logging_once,
    models::exchange::payload::order::{SetPositionTpslPayload, TpSlWithOrderPayload},
};
use rust_decimal::Decimal;
use solana_sdk::signature::{Keypair, Signer};
use tracing::info;

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
    let side = OrderSide::Ask;

    let take_price_raw: Decimal = Decimal::from_str("122500.123456").unwrap();
    let normalized_take_price: Decimal = tick_lot_utils
        .normalize_price(symbol, take_price_raw)
        .unwrap();

    let limit_take_price_raw: Decimal = Decimal::from_str("122000.123456").unwrap();
    let normalized_limit_take_price: Decimal = tick_lot_utils
        .normalize_price(symbol, limit_take_price_raw)
        .unwrap();
    // mid px 115000
    let stop_price_raw: Decimal = Decimal::from_str("109500.123456").unwrap();
    let normalized_stop_price: Decimal = tick_lot_utils
        .normalize_price(symbol, stop_price_raw)
        .unwrap();

    let limit_stop_price_raw: Decimal = Decimal::from_str("110000.123456").unwrap();
    let normalized_limit_stop_price: Decimal = tick_lot_utils
        .normalize_price(symbol, limit_stop_price_raw)
        .unwrap();

    // Создаем payload
    let sign_payload = SetPositionTpslPayload {
        symbol: symbol.to_string(),
        side,
        take_profit: TpSlWithOrderPayload {
            stop_price: normalized_take_price,
            limit_price: Some(normalized_limit_take_price),
            client_order_id: None,
        },
        stop_loss: TpSlWithOrderPayload {
            stop_price: normalized_stop_price,
            limit_price: Some(normalized_limit_stop_price),
            client_order_id: None,
        },
    };
    let response = client
        .set_position_tpsl(sign_payload, expiry_window)
        .await
        .map_err(|e| format!("failed to set position tpsl: {:?}", e))
        .unwrap();
    if response.success == Some(true) {
        info!("Successfully set postion tpsl: {:?}", response);
    } else {
        info!("Failed to  set postion tpsl {:?}", response);
    }
}
