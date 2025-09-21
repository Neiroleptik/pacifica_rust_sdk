use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RequestMethod {
    CreateOrder,
    CreateMarketOrder,
    CancelOrder,
    CancelAllOrders,
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RequestMethod::CreateOrder => "create_order",
            RequestMethod::CreateMarketOrder => "create_market_order",
            RequestMethod::CancelOrder => "cancel_order",
            RequestMethod::CancelAllOrders => "cancel_all_orders",
        };
        write!(f, "{}", s)
    }
}
