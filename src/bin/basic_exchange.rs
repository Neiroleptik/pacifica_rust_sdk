use std::env;

use dotenvy::from_filename;
use pacific_rust_sdk::{
    exchange::exchange_client::ExchangeClient,
    logging::init_logging_once,
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
    let api_key: Option<String> = None; // Or Some("YourApiKeyString".to_string());
    let agent_pubkey: Option<Pubkey> = None;
    let main_keypair = Keypair::from_base58_string(env::var("TEST_KEY").unwrap().as_str());
    let main_pubkey = main_keypair.pubkey();

    {
        ExchangeClient::new(
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
        }
}
