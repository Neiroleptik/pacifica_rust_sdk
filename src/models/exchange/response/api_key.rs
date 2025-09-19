use serde::{Deserialize, Serialize};

use crate::common::types::EmptyResponseData;

pub type RevokeApiKeyResponse = EmptyResponseData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateApiKeyResponse {
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListApiKeysResponse {
    pub active_api_keys: Vec<String>,
    pub api_key_limit: u8,
}
