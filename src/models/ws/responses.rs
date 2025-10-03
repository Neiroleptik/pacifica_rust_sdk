use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use solana_sdk::pubkey::Pubkey;
use uuid::Uuid;

use crate::{
    common::types::{
        Interval, OrderEventType, OrderSide, OrderStatus, OrderType, TradeCause, TradeSide,
    },
    models::info::response::market::OrderLevelModel,
};

pub type PositionsResponse = Vec<PositionModel>;
pub type OrdersResponse = Vec<OrderModel>;
pub type OrderUpdatesResponse = Vec<OrderUpdateModel>;
pub type TradesResponse = Vec<TradeModel>;
pub type PricesResponse = Vec<PriceModel>;
pub type AccountTradesResponse = Vec<AccountTradeModel>;

pub type CreateOrderResponse = BasicOrderActionResponse;
pub type CreateMarketOrderResponse = BasicOrderActionResponse;
pub type CancelOrderResponse = BasicOrderActionResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BasicOrderActionResponse {
    #[serde(rename = "I")]
    pub client_order_id: Option<Uuid>,
    #[serde(rename = "i")]
    pub order_id: Option<u64>,
    #[serde(rename = "s")]
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CancelAllOrdersResponse {
    cancelled_count: u32,
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
pub struct OrderBookResponse {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "l")]
    pub levels: Vec<Vec<OrderLevelModel>>,
    #[serde(rename = "t")]
    pub timestamp: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeModel {
    #[serde(rename = "a")]
    pub amount: Decimal,
    #[serde(rename = "d")]
    pub side: TradeSide,
    #[serde(rename = "e")]
    pub event_type: OrderEventType,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "tc")]
    pub cause: TradeCause,
    #[serde(rename = "u")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CandleResponse {
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
pub struct BalanceResponse {
    pub total: Decimal,
    pub available: Decimal,
    pub locked: Decimal,
    #[serde(rename = "t")]
    pub timestamp: u64,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarginResponse {
    #[serde(rename = "u")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub margin_mode: bool,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeverageResponse {
    #[serde(rename = "u")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "l")]
    pub leverage: Decimal,
    #[serde(rename = "t")]
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountInfoResponse {
    #[serde(rename = "ae")]
    pub account_equity: Decimal,
    #[serde(rename = "as")]
    pub available_to_spend: Decimal,
    #[serde(rename = "aw")]
    pub available_to_withdraw: Decimal,
    #[serde(rename = "b")]
    pub balance: Decimal,
    #[serde(rename = "f")]
    pub fee_tier: u8,
    #[serde(rename = "mu")]
    pub margin_used: Decimal,
    #[serde(rename = "oc")]
    pub orders_count: u32,
    #[serde(rename = "pb")]
    pub pending_balance: Decimal,
    #[serde(rename = "pc")]
    pub positions_count: u32,
    #[serde(rename = "sc")]
    pub stop_orders_count: u32,
    #[serde(rename = "t")]
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PositionModel {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub amount: Decimal,
    #[serde(rename = "p")]
    pub entry_price: Decimal,
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "d")]
    pub side: OrderSide,
    #[serde(rename = "m")]
    pub margin: Option<Decimal>,
    #[serde(rename = "f")]
    pub funding: Decimal,
    #[serde(rename = "i")]
    pub isolated: bool,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderModel {
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "I")]
    pub client_order_id: Option<Uuid>, // Uuid.to_string()
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "d")]
    pub side: OrderSide,
    #[serde(rename = "ip")]
    pub initial_price: Decimal,
    #[serde(rename = "p")]
    pub average_filled_price: Decimal,
    #[serde(rename = "a")]
    pub original_amount: Decimal,
    #[serde(rename = "f")]
    pub filled_amount: Decimal,
    #[serde(rename = "c")]
    pub cancelled_amount: Decimal,
    #[serde(rename = "os")]
    pub order_status: OrderStatus,
    #[serde(rename = "ot")]
    pub order_type: OrderType,
    #[serde(rename = "sp")]
    pub stop_price: Option<Decimal>,
    #[serde(rename = "st")]
    pub stop_type: Option<String>, // Will be changed to StopType Enum idk
    #[serde(rename = "ro")]
    pub reduce_only: bool,
    #[serde(rename = "u")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
    #[serde(rename = "oe")]
    pub event_type: OrderEventType,
    #[serde(rename = "t")]
    pub timestamp: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderUpdateModel {
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "I")]
    pub client_order_id: Option<Uuid>,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "d")]
    pub side: OrderSide,
    #[serde(rename = "ip")]
    pub initial_price: Decimal,
    #[serde(rename = "p")]
    pub average_filled_price: Decimal,
    #[serde(rename = "a")]
    pub original_amount: Decimal,
    #[serde(rename = "f")]
    pub filled_amount: Decimal,
    #[serde(rename = "os")]
    pub order_status: OrderStatus,
    #[serde(rename = "ot")]
    pub order_type: OrderType,
    #[serde(rename = "sp")]
    pub stop_price: Option<Decimal>,
    #[serde(rename = "si")]
    pub stop_order_id: Option<Decimal>,
    #[serde(rename = "r")]
    pub reduce_only: bool,
    #[serde(rename = "u")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
    #[serde(rename = "oe")]
    pub event_type: OrderEventType,
    #[serde(rename = "ut")]
    pub updated_at: u64,
    #[serde(rename = "ct")]
    pub created_at: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountTradeModel {
    #[serde(rename = "h")]
    pub history_id: u64,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "I")]
    pub client_order_id: Option<Uuid>,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub amount: Decimal,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "o")]
    pub entry_price: Decimal,
    #[serde(rename = "f")]
    pub fee: Decimal,
    #[serde(rename = "n")]
    pub pnl: Decimal,
    #[serde(rename = "te")]
    pub event_type: OrderEventType,
    #[serde(rename = "ts")]
    pub side: TradeSide,
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "c")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub counter_party: Pubkey,
    #[serde(rename = "tc")]
    pub cause: TradeCause,
}
