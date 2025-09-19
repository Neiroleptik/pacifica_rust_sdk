use serde::{Deserialize, Serialize};

use crate::{
    common::{types::EmptyPayload, utils::Validatable},
    make_validatable,
};

pub type CreateApiKeyPayload = EmptyPayload;
pub type ListApiKeysPayload = EmptyPayload;

make_validatable!(RevokeApiKeyPayload);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RevokeApiKeyPayload {
    pub api_key: String,
}
