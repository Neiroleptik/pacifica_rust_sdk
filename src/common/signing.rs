use std::process::Command;

use bs58;
use serde::Serialize;
use serde_json::Value;
use solana_sdk::{
    signature::{Keypair, Signature},
    signer::Signer,
};
use tracing::debug;

use super::utils::sort_json_keys;
use crate::common::errors::ExchangeError;

pub fn prepare_message<T: Serialize, U: Serialize>(
    header: &T,
    payload: &U,
) -> Result<(Value, Vec<u8>), ExchangeError> {
    let data = serde_json::to_value(header)?;
    if !data.is_object() {}
    match data {
        Value::Object(mut map) => {
            map.insert("data".to_string(), serde_json::to_value(payload)?);

            for key in ["type", "timestamp", "expiry_window"] {
                if !map.contains_key(key) {
                    map.insert(key.to_string(), Value::Null);
                }
            }
            let data = Value::Object(map);
            let sorted_data = sort_json_keys(&data);
            let message_bytes = serde_json::to_vec(&sorted_data)?;
            Ok((sorted_data, message_bytes))
        }
        _ => Err(ExchangeError::Custom(
            "header must serialize to a JSON object".to_string(),
        )),
    }
}

pub fn sign_message<T: Serialize, U: Serialize>(
    header: &T,
    payload: &U,
    keypair: &Keypair,
) -> Result<(Value, String), ExchangeError> {
    let (message, message_bytes) = prepare_message(header, payload)?;
    debug!("Message to sign: {}", message);
    let signature: Signature = keypair.sign_message(&message_bytes);
    let signature_base58 = bs58::encode(signature.as_ref()).into_string();
    Ok((message, signature_base58))
}

// don't tested
pub fn sign_with_hardware_wallet<T: Serialize, U: Serialize>(
    header: &T,
    payload: &U,
    hardware_wallet_path: &str,
) -> Result<(Value, String), ExchangeError> {
    let (message_value, message_bytes) = prepare_message(header, payload)?;
    let message_str = String::from_utf8(message_bytes)
        .map_err(|e| ExchangeError::LedgerSigningFailed(format!("Invalid UTF-8: {}", e)))?;
    let output = Command::new("solana")
        .arg("sign-offchain-message")
        .arg("-k")
        .arg(hardware_wallet_path)
        .arg(&message_str)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ExchangeError::LedgerSigningFailed(stderr.to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let signature_line = stdout.lines().last().ok_or(ExchangeError::NoSignature)?;
    Ok((message_value, signature_line.to_string()))
}
