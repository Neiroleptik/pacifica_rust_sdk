use std::env;

use dotenvy::from_filename;
use pacifica_rust_sdk::{
    exchange::exchange_client::ExchangeClient, logging::init_logging_once,
    models::exchange::payload::order::CancelAllOrdersPayload,
};
use solana_sdk::signature::{Keypair, Signer};
use tracing::error;
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

    // entry params
    let expiry_window: Option<u32> = None;

    // Cancel all orders
    {
        let sign_payload = CancelAllOrdersPayload {
            symbol: None,
            all_symbols: true, // if true, symbol must be None
            exclude_reduce_only: false,
        };

        match client.cancel_all_orders(sign_payload, expiry_window).await {
            Ok(order_response) => {
                info!("All order canceled successfully: {:?}", order_response);
            }
            Err(e) => {
                error!("Failed to cancel all orders: {:?}", e);
            }
        }
    }
}
