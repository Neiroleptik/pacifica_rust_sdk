use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::common::types::{Interval, OrderEventType, TradeCause, TradeSide};

pub type MarketsInfoResponse = Vec<MarketModel>;
pub type KlineResponse = Vec<CandleModel>;
pub type PricesResponse = Vec<PriceModel>;
pub type FundingRateHistoryResponse = Vec<FundingRateHistoryModel>;
pub type RecentTradesResponse = Vec<RecentTradeModel>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderBookResponse {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "l")]
    pub levels: Vec<Vec<OrderLevelModel>>,
    #[serde(rename = "t")]
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecentTradeModel {
    pub event_type: OrderEventType,
    pub price: Decimal,
    pub amount: Decimal,
    pub side: TradeSide,
    pub cause: TradeCause,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketModel {
    pub symbol: String,
    pub tick_size: Decimal,
    pub min_tick: Decimal,
    pub max_tick: Decimal,
    pub lot_size: Decimal,
    pub max_leverage: Decimal,
    pub isolated_only: bool,
    pub min_order_size: Decimal,
    pub max_order_size: Decimal,
    pub funding_rate: Decimal,
    pub next_funding_rate: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceModel {
    pub funding: Decimal,
    pub mark: Decimal,
    pub mid: Decimal,
    pub next_funding: Decimal,
    pub open_interest: Decimal,
    pub oracle: Decimal,
    pub symbol: String,
    pub timestamp: u64,
    pub volume_24h: Decimal,
    pub yesterday_price: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CandleModel {
    #[serde(rename = "t")]
    pub start_time: u64,
    #[serde(rename = "T")]
    pub end_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: Interval,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "v")]
    pub volume: Decimal,
    #[serde(rename = "n")]
    pub trades: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderLevelModel {
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "a")]
    pub amount: Decimal,
    #[serde(rename = "n")]
    pub num_orders: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FundingRateHistoryModel {
    pub oracle_price: Decimal,
    pub bid_impact_price: Decimal,
    pub ask_impact_price: Decimal,
    pub funding_rate: Decimal,
    pub next_funding_rate: Decimal,
    pub created_at: u64,
}
