use std::env;

use dotenvy::from_filename;
use pacifica_rust_sdk::{
    exchange::exchange_client::ExchangeClient, logging::init_logging_once,
    models::exchange::payload::order::CancelOrderPayload,
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
    let symbol: &'static str = "BTC";
    let expiry_window: Option<u32> = None;
    let order_id_to_cancel = Some(123456789); // Put id

    // Cancel stop order
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
