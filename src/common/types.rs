use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use solana_sdk::pubkey::Pubkey;
use uuid::Uuid;

use crate::{
    common::{errors::ExchangeError, utils::Validatable},
    make_validatable,
};

make_validatable!(EmptyStruct);
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EmptyStruct {}
pub type EmptyPayload = EmptyStruct;
pub type EmptyParams = EmptyStruct;
pub type EmptyResponseData = EmptyStruct;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[repr(u32)]
#[serde(into = "u32", try_from = "u32")]
pub enum AggLevel {
    L1 = 1,
    L2 = 2,
    L5 = 5,
    L10 = 10,
    L100 = 100,
    L1000 = 1000,
}

impl From<AggLevel> for u32 {
    fn from(v: AggLevel) -> Self {
        v as u32
    }
}

impl TryFrom<u32> for AggLevel {
    type Error = ExchangeError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AggLevel::L1),
            2 => Ok(AggLevel::L2),
            5 => Ok(AggLevel::L5),
            10 => Ok(AggLevel::L10),
            100 => Ok(AggLevel::L100),
            1000 => Ok(AggLevel::L1000),
            other => Err(ExchangeError::InvalidAggLevel(other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Interval {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "3m")]
    ThreeMinutes,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "2h")]
    TwoHours,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "8h")]
    EightHours,
    #[serde(rename = "12h")]
    TwelveHours,
    #[serde(rename = "1d")]
    OneDay,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EquityHistoryInterval {
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "7d")]
    SevenDays,
    #[serde(rename = "30d")]
    Month,
    #[serde(rename = "all")]
    All,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Tif {
    GTC,
    IOC,
    ALO,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    TakeProfitLimit,
    StopLossLimit,
    TakeProfitMarket,
    StopLossMarket,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OrderReason {
    Cancelled,
    Rejected,
    Cancel,
    ForceCancel,
    Expired,
    PostOnlyRejected,
    SelfTradePrevented,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OrderEventType {
    Make,
    Take,
    FulfillMaker,
    FulfillTaker,
    FulfillMarket,
    FulfillLimit,
    Adjust,
    StopCreated,
    StopParentOrderFilled,
    StopTriggered,
    StopUpgrade,
    Cancel,
    ForceCancel,
    Expired,
    PostOnlyRejected,
    SelfTradePrevented,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TradeSide {
    OpenLong,
    OpenShort,
    CloseLong,
    CloseShort,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TradeCause {
    Normal,
    MarketLiquidation,
    BackstopLiquidation,
    Settlement,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AccountEventType {
    Deposit,
    DepositRelease,
    Withdraw,
    Trade,
    MarketLiquidation,
    BackstopLiquidation,
    AdlLiquidation,
    SubaccountTransfer,
    Funding,
    Payout,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PacificSignature {
    Simple(String),
    Hardware(HardwareWalletSignature),
    Raw(RawSignature),
}

fn default_raw_signature_name() -> String {
    "raw".to_string()
}

make_validatable!(RawSignature);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawSignature {
    #[serde(rename = "type", default = "default_raw_signature_name")]
    type_field: String,
    pub signature: String,
}

fn default_hardware_signature_name() -> String {
    "hardware".to_string()
}

make_validatable!(HardwareWalletSignature);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HardwareWalletSignature {
    #[serde(rename = "type", default = "default_hardware_signature_name")]
    type_field: String,
    pub signature: String,
}

// Responses are not required validation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultResponse<P> {
    pub success: Option<bool>,
    pub data: Option<P>,
    pub error: Option<String>,
    pub code: Option<u16>,
}

make_validatable!(DefaultSignatureHeaders);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultSignatureHeaders {
    pub timestamp: u64,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_window: Option<u32>,
}

make_validatable!(DefaultFinalHeaders);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultFinalHeaders {
    #[serde_as(as = "DisplayFromStr")]
    pub account: Pubkey,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub agent_wallet: Option<Pubkey>,
    pub signature: PacificSignature,
    pub timestamp: u64, // need be equal with signature timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_window: Option<u32>, // need be equal with signature expiry_window
}

make_validatable!(SubAccountFinalHeaders);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubAccountFinalHeaders {
    #[serde_as(as = "DisplayFromStr")]
    pub main_account: Pubkey,
    #[serde_as(as = "DisplayFromStr")]
    pub subaccount: Pubkey,
    pub main_signature: PacificSignature,
    pub sub_signature: PacificSignature,
    pub timestamp: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_window: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OperationFinalHeaders {
    Default(DefaultFinalHeaders),
    SubAccountCreate(SubAccountFinalHeaders),
}

// Validation not implemented yet
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinalRequest<P> {
    #[serde(flatten)]
    pub headers: OperationFinalHeaders,
    #[serde(flatten)]
    pub payload: P,
}

// WebSocket structs don't implement validation fn,
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSocketParams<P> {
    pub source: String,
    #[serde(flatten)]
    pub params: P,
}

#[derive(Serialize, Debug, Clone)]
pub struct WebSocketRequest<P> {
    pub id: Uuid,
    // Where String is action_type/method
    pub params: HashMap<String, FinalRequest<P>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultWebSocketMsg<R> {
    pub channel: String,
    pub data: R,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSocketOperationResponse<R> {
    pub code: u16,
    pub data: Option<R>,
    pub err: Option<String>,
    pub id: Option<Uuid>,
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WsMethod {
    Subscribe,
    Unsubscribe,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSocketSubscription<P> {
    pub method: WsMethod,
    pub params: WebSocketParams<P>,
}

pub type WebSocketUnsubscription<P> = WebSocketSubscription<P>;
