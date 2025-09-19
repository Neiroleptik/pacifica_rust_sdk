use serde::{Deserialize, Serialize};

use crate::{
    common::{types::FinalRequest, utils::Validatable},
    make_validatable,
    models::exchange::payload::order::{
        CancelOrderPayload, CreateMarketOrderPayload, CreateOrderPayload,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum BatchOrderActionType {
    Cancel,
    Create,
    CreateMarket,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BatchOrderActionPayload {
    CreateOrder(CreateOrderPayload),
    CancelOrder(CancelOrderPayload),
    CreateMarketOrder(CreateMarketOrderPayload),
}

make_validatable!(BatchOrderActionsFinalHeaders);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchOrderActionsFinalHeaders {
    #[serde(rename = "type")]
    pub type_field: BatchOrderActionType,
    pub data: FinalRequest<BatchOrderActionPayload>,
}

make_validatable!(BatchOrderFinalRequest);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchOrderFinalRequest {
    pub actions: Vec<BatchOrderActionsFinalHeaders>,
}
