use std::{env, str::FromStr};

use dotenvy::from_filename;
use pacifica_rust_sdk::{
    common::types::DefaultResponse,
    exchange::exchange_client::ExchangeClient,
    logging::init_logging_once,
    models::exchange::{
        payload::subaccount::SubaccountTransferPayload,
        response::subaccount::SubaccountTransferResponse,
    },
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

    let sub_account: Keypair = Keypair::from_base58_string(""); // Put your sub-account key as ::from_base56_str

    info!("Using main account: {}", main_pubkey);

    let expiry_window: Option<u32> = None;

    let to_account = sub_account.pubkey();
    let amount = Decimal::from_str("420.69").unwrap();

    // Subaccount Transfer
    let response: DefaultResponse<SubaccountTransferResponse> = client
        .subaccount_transfer(
            SubaccountTransferPayload { to_account, amount },
            expiry_window,
        )
        .await
        .map_err(|e| format!("failed to transfer: {:?}", e))
        .unwrap();

    if response.data.is_some() && response.data.as_ref().unwrap().success {
        info!(
            "Successfully transfered {2} from {0} to sub-account: {1}",
            main_pubkey,
            sub_account.pubkey(),
            amount
        );
    } else {
        info!("Failed to transfer {:?}", response);
    }
}
