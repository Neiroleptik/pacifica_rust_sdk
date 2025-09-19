use std::{env, str::FromStr};

use dotenvy::from_filename;
use tracing::info;
use pacifica_rust_sdk::{
    common::types::{OrderSide, Tif},

    exchange::exchange_client::ExchangeClient,
    logging::init_logging_once,
    models::exchange::{
        payload::{
            order::{
                CancelOrderPayload,
                CreateOrderPayload,
            },
        },
    },
};
use rust_decimal::Decimal;
use solana_sdk::signature::{Keypair, Signer};
use tracing::error;

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
    let price_raw: Decimal = Decimal::from_str("110000.123456").unwrap();
    let normalized_price: Decimal = tick_lot_utils.normalize_price(symbol, price_raw).unwrap();
    let expiry_window: Option<u32> = None;
    let amount_raw: Decimal = Decimal::from_str("0.00123456").unwrap();
    let normalized_amount: Decimal = tick_lot_utils.normalize_amount(symbol, amount_raw).unwrap();

    // 1. Create a order
    let order_id_to_cancel: Option<_> = {
        let sign_payload = CreateOrderPayload {
            symbol: symbol.to_string(),
            price: normalized_price,
            amount: normalized_amount,
            side: OrderSide::Bid,
            tif: Tif::ALO,
            reduce_only: false,
            client_order_id: None,
            take_profit: None,
            stop_loss: None,
        };

        match client.order(sign_payload, expiry_window).await {
            Ok(order_response) => {
                if let Some(data) = order_response.data {
                    info!("Order placed successfully: {:?}", &data.order_id);
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
        }
    };

    if order_id_to_cancel.is_none() {
        error!("No order to cancel. Finish.");
        return;
    }

    // 2. Cancel order
    {
        let sign_payload = CancelOrderPayload {
            symbol: symbol.to_string(),
            order_id: order_id_to_cancel, // One of order_id or client_order_id is required
            // order_id: None,
            // client_order_id: Some(Uuid),
            client_order_id: None,
        };

        match client.cancel_order(sign_payload, expiry_window).await {
            Ok(cancel_response) => {
                info!("Order cancelled successfully: {:?}", cancel_response);
            }
            Err(e) => {
                error!("Failed to place order: {:?}", e);
            }
        }
    }
}
