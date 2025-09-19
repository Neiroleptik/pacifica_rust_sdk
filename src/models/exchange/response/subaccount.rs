use serde::{Deserialize, Serialize};

use crate::common::types::EmptyResponseData;

pub type SubaccountCreateResponse = EmptyResponseData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubaccountTransferResponse {
    pub success: bool,
    pub error: Option<String>,
}
