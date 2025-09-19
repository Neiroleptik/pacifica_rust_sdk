use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchOrderModel {
    pub success: bool,
    pub order_id: Option<u64>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchOrderResponse {
    pub results: Vec<BatchOrderModel>,
}
