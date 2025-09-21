use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use solana_sdk::pubkey::Pubkey;
use uuid::Uuid;

use crate::common::types::{
    AccountEventType, OrderEventType, OrderReason, OrderSide, OrderStatus, OrderType, TradeCause,
    TradeSide,
};

pub type OpenedOrdersResponse = Vec<OpenedOrderModel>;
pub type OrderHistoryResponse = Vec<OrderHistoryModel>;
pub type OrderHistoryByIdResponse = Vec<OrderHistoryByIdModel>;
pub type AccountFundingHistoryResponse = Vec<AccountFundingHistoryModel>;
pub type AccountSettingsResponse = Vec<AccountSettingsModel>;
pub type EquityHistoryResponse = Vec<EquityHistoryModel>;
pub type BalanceHistoryResponse = Vec<BalanceHistoryModel>;
pub type PositionsResponse = Vec<PositionModel>;
pub type TradesHistoryResponse = Vec<TradeHistoryModel>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountResponse {
    pub balance: Decimal,
    pub fee_level: u8,
    pub account_equity: Decimal,
    pub available_to_spend: Decimal,
    pub available_to_withdraw: Decimal,
    pub pending_balance: Decimal,
    pub total_margin_used: Decimal,
    pub positions_count: u32,
    pub orders_count: u32,
    pub stop_orders_count: u32,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountSettingsModel {
    pub symbol: String,
    pub isolated: bool,
    pub leverage: Decimal,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountFundingHistoryModel {
    pub history_id: u64,
    pub symbol: String,
    pub side: OrderSide,
    pub amount: Decimal,
    pub payout: Decimal,
    pub rate: Decimal,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenedOrderModel {
    pub order_id: u64,
    pub client_order_id: Option<Uuid>, // Uuid.to_string()
    pub symbol: String,
    pub side: OrderSide,
    pub price: Decimal,
    pub initial_amount: Decimal,
    pub filled_amount: Decimal,
    pub cancelled_amount: Decimal,
    pub stop_price: Option<Decimal>,
    pub order_type: OrderType,
    pub stop_parent_order_id: Option<u64>,
    pub reduce_only: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderHistoryModel {
    pub order_id: u64,
    pub client_order_id: Option<Uuid>, // Uuid.to_string()
    pub symbol: String,
    pub side: OrderSide,
    pub initial_price: Decimal,
    pub average_filled_price: Decimal,
    pub amount: Decimal,
    pub filled_amount: Decimal,
    pub order_status: OrderStatus,
    pub order_type: OrderType,
    pub stop_price: Option<Decimal>,
    pub stop_parent_order_id: Option<u64>,
    pub reduce_only: bool,
    pub reason: Option<OrderReason>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderHistoryByIdModel {
    pub history_id: u64,
    pub order_id: u64,
    pub client_order_id: Option<Uuid>, // Uuid.to_string()
    pub symbol: String,
    pub side: OrderSide,
    pub price: Decimal,
    pub initial_amount: Decimal,
    pub filled_amount: Decimal,
    pub cancelled_amount: Decimal,
    pub event_type: OrderEventType,
    pub order_type: OrderType,
    pub order_status: OrderStatus,
    pub stop_price: Option<Decimal>,
    pub stop_parent_order_id: Option<u64>,
    pub reduce_only: bool,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalanceHistoryModel {
    pub amount: Decimal,
    pub balance: Decimal,
    pub pending_balance: Decimal,
    pub event_type: AccountEventType,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EquityHistoryModel {
    pub account_equity: Decimal,
    pub pnl: Decimal,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PositionModel {
    pub symbol: String,
    pub side: OrderSide,
    pub amount: Decimal,
    pub entry_price: Decimal,
    pub margin: Option<Decimal>,
    pub funding: Decimal,
    pub isolated: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeHistoryModel {
    pub history_id: u64,
    pub order_id: u64,
    pub client_order_id: Option<Uuid>, // Uuid.to_string()
    pub symbol: String,
    pub amount: Decimal,
    pub price: Decimal,
    pub entry_price: Decimal,
    pub fee: Decimal,
    pub pnl: Decimal,
    pub event_type: OrderEventType,
    pub side: TradeSide,
    pub created_at: u64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub counter_party: Pubkey,
    pub cause: TradeCause,
}
