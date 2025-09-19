use std::env;

use dotenvy::from_filename;
use tracing::info;
use pacifica_rust_sdk::{
    common::types::DefaultResponse,
    exchange::exchange_client::ExchangeClient,
    logging::init_logging_once,
    models::exchange::{
        payload::agent_wallet::BindAgentWalletPayload,
        response::agent_wallet::BindAgentWalletResponse,
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

    let agent_keypair = Keypair::from_base58_string("YOUR_AGENT_WALLET_KEYPAIR_IN_BASE58");
    // Or Keypair::new();
    // Do not forget to print it!
    // println!("{}", agent_keypair.to_base58_string());

    let agent_pubkey = agent_keypair.pubkey();

    info!("Using main account: {}", main_pubkey);
    info!("Binding Agent Wallet: {0}", agent_pubkey);

    let expiry_window: Option<u32> = None;

    let bind_response: DefaultResponse<BindAgentWalletResponse> = client
        .bind_agent_wallet(
            BindAgentWalletPayload {
                agent_wallet: agent_pubkey.clone(),
            },
            expiry_window,
        )
        .await
        .map_err(|e| format!("failed to bind agent wallet: {:?}", e))
        .unwrap();
    if bind_response.success == Some(true) {
        info!("Successfully bound agent wallet: {0}", agent_pubkey);
    } else {
        info!("Failed to bind agent wallet {0}", agent_pubkey);
    };
}
