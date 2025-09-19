use std::env;

use dotenvy::from_filename;
use tracing::info;
use pacifica_rust_sdk::{
    exchange::exchange_client::ExchangeClient,
    logging::init_logging_once,
    models::exchange::{
        payload::{
            market_settings::UpdateMarginModePayload,
        }
    },
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

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
        api_key.clone(),
        main_keypair,
        main_pubkey,
        agent_pubkey,
    )
    .await
    .map_err(|e| format!("failed to init client: {:?}", e))
    .unwrap();

    info!("Using main account: {}", main_pubkey);

    let expiry_window: Option<u32> = None;

    // Update Margin Mode
    let is_isolated = true; // true for isolated, false for cross
    let symbol = "ETH".to_string();

    let update_margin_mode_response = client
        .update_margin_mode(
            UpdateMarginModePayload {
                symbol,
                is_isolated,
            },
            expiry_window,
        )
        .await
        .map_err(|e| format!("failed to update margin mode: {:?}", e))
        .unwrap();
    if update_margin_mode_response.success == Some(true) {
        info!(
            "Successfully updated margin mode: {:?}",
            update_margin_mode_response
        );
    } else {
        info!(
            "Failed to update margin mode {:?}",
            update_margin_mode_response
        );
    }
}
