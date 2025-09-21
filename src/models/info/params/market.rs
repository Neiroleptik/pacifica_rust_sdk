use serde::{Deserialize, Serialize};

use crate::{
    common::{
        types::{AggLevel, EmptyParams, Interval},
        utils::Validatable,
    },
    make_validatable,
};
pub type MarketsInfoParams = EmptyParams;
pub type PricesParams = EmptyParams;

make_validatable!(KlineParams);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KlineParams {
    pub symbol: String,
    pub interval: Interval,
    pub start_time: u64,
    pub end_time: Option<u64>,
}

make_validatable!(OrderBookParams);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderBookParams {
    pub symbol: String,
    pub agg_level: Option<AggLevel>,
}

make_validatable!(RecentTradesParams);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecentTradesParams {
    pub symbol: String,
}

make_validatable!(FundingRateHistoryParams);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FundingRateHistoryParams {
    pub symbol: String,
}
