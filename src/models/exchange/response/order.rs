use serde::{Deserialize, Serialize};

use crate::common::types::EmptyResponseData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrderResponse {
    pub order_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelAllOrdersResponse {
    pub cancelled_count: u32,
}

pub type CreateMarketOrderResponse = CreateOrderResponse;
pub type CreateStopOrderResponse = CreateOrderResponse;
pub type SetPositionTPSLResponse = EmptyResponseData;
pub type CancelOrderResponse = EmptyResponseData;
pub type CancelStopOrderResponse = EmptyResponseData;
