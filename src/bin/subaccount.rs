use std::env;

use dotenvy::{from_filename};
use tracing::info;
use pacifica_rust_sdk::{
    exchange::exchange_client::ExchangeClient,
    logging::init_logging_once,
};
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

    let expiry_window: Option<u32> = None;

    let sub_account = Keypair::new(); // Put your sub-account key as ::from_base56_str

    // Create subaccount
    let response = client
        .subaccount_create(&sub_account, expiry_window)
        .await
        .map_err(|e| format!("failed to create subaccount: {:?}", e))
        .unwrap();

    if response.success == Some(true) {
        info!(
            "Successfully created sub-account: {0}",
            sub_account.to_base58_string()
        );
    } else {
        info!("Failed to create sub-account: {:?}", response);
    }
}
