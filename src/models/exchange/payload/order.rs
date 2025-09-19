use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::{
        errors::ExchangeError,
        types::{OrderSide, Tif},
        utils::{Validatable, validate_at_least_one},
    },
    make_validatable,
};

make_validatable!(SetPositionTpslPayload);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPositionTpslPayload {
    pub symbol: String,
    pub side: OrderSide,
    pub take_profit: TpSlWithOrderPayload,
    pub stop_loss: TpSlWithOrderPayload,
}

make_validatable!(CreateOrderPayload);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrderPayload {
    pub symbol: String,
    pub price: Decimal,
    pub amount: Decimal,
    pub side: OrderSide,
    pub tif: Tif,
    pub reduce_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<Uuid>, // uuid:Uuid::new_v4().to_string() [features("v4",)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<TpSlWithOrderPayload>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<TpSlWithOrderPayload>,
}
// May be required both tpsl
// impl CreateOrderPayload {
//     pub fn validate(&self) -> Result<(), ExchangeError> {
//         match_both_some(&self.take_profit, &self.stop_loss)
//
//     }
// }
make_validatable!(CreateStopOrderPayload);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateStopOrderPayload {
    pub symbol: String,
    pub side: OrderSide,
    pub reduce_only: bool,
    pub stop_order: TpSlAlonePayload,
}

make_validatable!(TpSlAlonePayload);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TpSlAlonePayload {
    pub stop_price: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<Uuid>,
    pub amount: Decimal,
}

make_validatable!(TpSlWithOrderPayload);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TpSlWithOrderPayload {
    pub stop_price: Decimal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<Uuid>,
}

make_validatable!(CreateMarketOrderPayload);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMarketOrderPayload {
    pub symbol: String,
    pub amount: Decimal,
    pub side: OrderSide,
    pub slippage_percent: Decimal,
    pub reduce_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<Uuid>, // uuid:Uuid::new_v4().to_string() [features("v4",)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<TpSlWithOrderPayload>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<TpSlWithOrderPayload>,
}
// May be required both tpsl
// impl CreateMarketOrderPayload {
//     pub fn validate(&self) -> Result<(), ExchangeError> {
//         match_both_some(&self.take_profit, &self.stop_loss)
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelOrderPayload {
    pub symbol: String,
    // Required one of order_id or client_order_id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<Uuid>,
}

impl CancelOrderPayload {
    pub fn validate(&self) -> Result<(), ExchangeError> {
        validate_at_least_one(
            &self.order_id,
            &self.client_order_id,
            "order_id",
            "client_order_id",
        )
        .unwrap();
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelAllOrdersPayload {
    pub all_symbols: bool,
    pub exclude_reduce_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>, // required if all_symbols is False
}

impl CancelAllOrdersPayload {
    pub fn validate(&self) -> Result<(), ExchangeError> {
        if !self.all_symbols && self.symbol.is_none() {
            return Err(ExchangeError::Validation(
                "symbol is required when all_symbols is false".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelStopOrderPayload {
    pub symbol: String,
    // Required one of order_id or client_order_id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<Uuid>,
}

impl CancelStopOrderPayload {
    pub fn validate(&self) -> Result<(), ExchangeError> {
        validate_at_least_one(
            &self.order_id,
            &self.client_order_id,
            "order_id",
            "client_order_id",
        )
        .unwrap();
        Ok(())
    }
}
