use std::env;

use dotenvy::{from_filename};
use pacifica_rust_sdk::{
    exchange::exchange_client::ExchangeClient,
    logging::init_logging_once,
    models::exchange::payload::market_settings::UpdateLeveragePayload
};

use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use tracing::info;

#[tokio::main]
async fn main() {
    from_filename("src/bin/EXAMPLE.env").ok();
    init_logging_once("debug");
    let is_mainnet = false;
    let enable_ws = true;
    let api_key: Option<String> = None; // Or Some("YourApiKeyString".to_string());
    let agent_pubkey: Option<Pubkey> = None; // Or Some(Pubkey::from_str("YourAgentWalletPubkey").unwrap());
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

    let lev_response = client
        .update_leverage(
            UpdateLeveragePayload {
                symbol: "SOL".to_string(),
                leverage: 2,
            },
            None,
        )
        .await
        .unwrap();
    info!("Leverage update: {:?}", lev_response);
}
