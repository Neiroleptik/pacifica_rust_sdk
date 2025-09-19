use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use solana_sdk::pubkey::Pubkey;

use crate::{
    common::{
        types::EquityHistoryInterval,
        utils::Validatable,
    },
    make_validatable,
};

make_validatable!(AccountParams);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountParams {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

make_validatable!(AccountSettingsParams);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountSettingsParams {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

make_validatable!(PositionsParams);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PositionsParams {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

make_validatable!(TradesHistoryParams);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradesHistoryParams {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

make_validatable!(AccountFundingHistoryParams);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountFundingHistoryParams {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

make_validatable!(EquityHistoryParams);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EquityHistoryParams {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
    pub time_range: EquityHistoryInterval,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity_in_minutes: Option<u8>, // mb increase to u16?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

make_validatable!(BalanceHistoryParams);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalanceHistoryParams {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

make_validatable!(OpenedOrdersParams);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenedOrdersParams {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

make_validatable!(OrderHistoryParams);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderHistoryParams {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

make_validatable!(OrderHistoryByIdParams);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderHistoryByIdParams {
    pub order_id: u64,
}
