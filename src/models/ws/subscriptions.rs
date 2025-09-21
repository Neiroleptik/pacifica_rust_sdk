use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use solana_sdk::pubkey::Pubkey;

use crate::common::types::{AggLevel, Interval};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionMethod {
    Prices,
    Trades,
    Book,
    Candle,
    Balance,
    Margin,
    Leverage,
    AccountInfo,
    Positions,
    Orders,
    AccountOrderUpdates,
    AccountTrades,
}
impl fmt::Display for SubscriptionMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SubscriptionMethod::Prices => "prices",
            SubscriptionMethod::Trades => "trades",
            SubscriptionMethod::Book => "book",
            SubscriptionMethod::Candle => "candle",
            SubscriptionMethod::Balance => "balance",
            SubscriptionMethod::Margin => "margin",
            SubscriptionMethod::Leverage => "leverage",
            SubscriptionMethod::AccountInfo => "account_info",
            SubscriptionMethod::Positions => "positions",
            SubscriptionMethod::Orders => "orders",
            SubscriptionMethod::AccountOrderUpdates => "account_order_updates",
            SubscriptionMethod::AccountTrades => "account_trades",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prices {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderBook {
    pub symbol: String,
    pub agg_level: AggLevel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trades {
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Candle {
    pub symbol: String,
    pub interval: Interval,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Margin {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Leverage {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Positions {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Orders {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderUpdates {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountTrades {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}
