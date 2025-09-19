use std::env;

use dotenvy::from_filename;
use tracing::info;
use pacifica_rust_sdk::{
    common::types::DefaultResponse,
    exchange::exchange_client::ExchangeClient,
    logging::init_logging_once,
    models::exchange::{
        payload::api_key::{CreateApiKeyPayload, ListApiKeysPayload, RevokeApiKeyPayload},
        response::api_key::ListApiKeysResponse,
    },
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

    // Create API Key
    let create_api_key_response = client
        .create_api_key(CreateApiKeyPayload {}, expiry_window)
        .await
        .map_err(|e| format!("failed to create api key: {:?}", e))
        .unwrap();
    if create_api_key_response.success == Some(true) {
        info!(
            "Successfully create api key: {:?}",
            create_api_key_response.data.unwrap().api_key
        );
    } else {
        info!("Failed to create api key {:?}", create_api_key_response);
    }

    // List API Keys
    let list_api_keys_response: DefaultResponse<ListApiKeysResponse> = client
        .get_list_api_keys(ListApiKeysPayload {}, expiry_window)
        .await
        .map_err(|e| format!("failed to list api keys: {:?}", e))
        .unwrap();
    if list_api_keys_response.success == Some(true) {
        info!(
            "Successfully list api keys: {:?}",
            list_api_keys_response.data.unwrap()
        );
    } else {
        info!("Failed to list api keys {:?}", list_api_keys_response);

        // Revoke API Key
        if let Some(api_keys) = list_api_keys_response.data {
            if let Some(first_api_key) = api_keys.active_api_keys.first() {
                let revoke_api_key_response = client
                    .revoke_api_key(
                        RevokeApiKeyPayload {
                            api_key: first_api_key.clone(),
                        },
                        expiry_window,
                    )
                    .await
                    .map_err(|e| format!("failed to revoke api key: {:?}", e))
                    .unwrap();
                if revoke_api_key_response.success == Some(true) {
                    info!(
                        "Successfully revoked api key: {:?}",
                        revoke_api_key_response
                    );
                } else {
                    info!("Failed to revoke api key {:?}", revoke_api_key_response);
                }
            } else {
                info!("No API keys found to revoke.");
            }
        } else {
            info!("No API keys found.");
        }
    }
}
