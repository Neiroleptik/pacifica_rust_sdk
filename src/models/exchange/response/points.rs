use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use solana_sdk::pubkey::Pubkey;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[serde_as]
pub enum GetPointsResponse {
    Success {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        user: Pubkey,
        point: Decimal,
    },
    Error {
        error: String,
    },
}

impl GetPointsResponse {
    pub fn is_error(&self) -> bool {
        matches!(self, GetPointsResponse::Error { .. })
    }

    pub fn error(&self) -> Option<&str> {
        match self {
            GetPointsResponse::Error { error } => Some(error),
            _ => None,
        }
    }

    pub fn success(&self) -> Option<(&Pubkey, Decimal)> {
        match self {
            GetPointsResponse::Success { user, point } => Some((user, *point)),
            _ => None,
        }
    }
}
