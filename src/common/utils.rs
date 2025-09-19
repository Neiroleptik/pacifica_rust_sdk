use std::{
    fmt::Debug,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use rust_decimal::prelude::ToPrimitive;
use serde::{Serialize, Serializer};
use serde_json::{Map, Value};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use tracing::debug;
use uuid::Uuid;

use crate::common::{
    errors::ExchangeError,
    signing::sign_message,
    types::{
        DefaultFinalHeaders, DefaultSignatureHeaders, FinalRequest, OperationFinalHeaders,
        PacificSignature,
    },
};

pub fn get_timestamp_ms() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_epoch.as_millis().to_u64().unwrap()
}

pub fn sort_json_keys(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted = Map::new();
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            for k in keys {
                if let Some(v) = map.get(&k) {
                    sorted.insert(k, sort_json_keys(v));
                }
            }
            Value::Object(sorted)
        }
        Value::Array(arr) => {
            let new_arr = arr
                .iter()
                .map(|v| sort_json_keys(v))
                .collect::<Vec<Value>>();
            Value::Array(new_arr)
        }
        _ => value.clone(),
    }
}

fn ensure_pubkey(s: &str) -> Result<(), String> {
    Pubkey::from_str(s)
        .map(|_| ())
        .map_err(|_| format!("Not valid public key: {}", s))
}

pub fn validate_pubkey<S>(pubkey: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ensure_pubkey(pubkey).map_err(|e| serde::ser::Error::custom(e))?;
    serializer.serialize_str(pubkey)
}

pub fn validate_pubkey_option<S>(pubkey: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match pubkey {
        Some(s) => {
            ensure_pubkey(s).map_err(|e| serde::ser::Error::custom(e))?;
            serializer.serialize_some(s)
        }
        None => serializer.serialize_none(),
    }
}

pub fn validate_at_least_one<'a, P1, P2>(
    first: &'a Option<P1>,
    second: &'a Option<P2>,
    first_name: &str,
    second_name: &str,
) -> Result<(), String> {
    if first.is_none() && second.is_none() {
        Err(format!(
            "At least one of {} or {} must be set",
            first_name, second_name
        ))
    } else {
        Ok(())
    }
}

pub fn validate_uuid<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(uuid_str) = value {
        Uuid::parse_str(uuid_str)
            .map_err(|_| serde::ser::Error::custom(format!("Not valid UUID: {}", uuid_str)))?;
        serializer.serialize_str(uuid_str)
    } else {
        serializer.serialize_none()
    }
}

pub fn match_both_some<T, P>(one: &Option<T>, second: &Option<P>) -> Result<(), ExchangeError> {
    match (one, second) {
        (Some(_), Some(_)) | (None, None) => Ok(()),
        _ => Err(ExchangeError::Validation(
            "Both params must be set together or not set at all.".into(),
        )),
    }
}

pub trait Validatable {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[macro_export]
macro_rules! make_validatable {
    ($name:ident) => {
        impl Validatable for $name {
            fn validate(&self) -> Result<(), String> {
                Ok(())
            }
        }
    };
}

pub async fn prepare_final_request<P>(
    request_method: &str,
    sign_payload: P,
    expiry_window: Option<u32>,
    keypair: &Keypair,
    main_pubkey: &Pubkey,
    agent_pubkey: &Option<Pubkey>,
) -> Result<FinalRequest<P>, ExchangeError>
where
    P: Serialize + Debug,
{
    let sign_headers = DefaultSignatureHeaders {
        timestamp: get_timestamp_ms(),
        expiry_window: expiry_window,
        type_field: request_method.to_string(),
    };
    let (_message, signature) = sign_message(&sign_headers, &sign_payload, keypair)?;
    let final_headers = OperationFinalHeaders::Default(DefaultFinalHeaders {
        account: main_pubkey.clone(),
        agent_wallet: agent_pubkey.clone(),
        signature: PacificSignature::Simple(signature),
        expiry_window: sign_headers.expiry_window,
        timestamp: sign_headers.timestamp,
    });
    let final_request: FinalRequest<P> = FinalRequest {
        headers: final_headers,
        payload: sign_payload,
    };

    debug!("create_final_request: final_request={:?}", &final_request);

    Ok(final_request)
}
