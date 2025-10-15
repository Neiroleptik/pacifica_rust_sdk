use std::{collections::HashMap, str::FromStr};

use rust_decimal::prelude::*;

use crate::{common::errors::ExchangeError, models::info::response::market::MarketModel};

#[derive(Clone, Debug)]
pub struct TickLot {
    market_cache: HashMap<String, MarketModel>,
}

impl TickLot {
    pub fn new(market_cache: HashMap<String, MarketModel>) -> Self {
        Self { market_cache }
    }

    pub fn normalize_price<S: AsRef<str>>(
        &self,
        symbol: S,
        price: Decimal,
    ) -> Result<Decimal, ExchangeError> {
        let key: &str = symbol.as_ref();

        if price <= Decimal::ZERO {
            return Err(ExchangeError::InvalidPriceValue(format!(
                "price must be positive, got {}",
                price
            )));
        }

        if let Some(market) = self.market_cache.get(key) {
            if market.tick_size.is_zero() {
                return Err(ExchangeError::Custom(format!(
                    "tick_size for {} is zero",
                    key
                )));
            }
            Ok(round_price_down_to_tick(price, market.tick_size))
        } else {
            Err(ExchangeError::SymbolNotFound(key.to_string()))
        }
    }

    pub fn normalize_amount<S: AsRef<str>>(
        &self,
        symbol: S,
        amount: Decimal,
    ) -> Result<Decimal, ExchangeError> {
        let key: &str = symbol.as_ref();

        if amount <= Decimal::ZERO {
            return Err(ExchangeError::InvalidAmountValue(format!(
                "amount must be positive, got {}",
                amount
            )));
        }

        if let Some(market) = self.market_cache.get(key) {
            if market.lot_size.is_zero() {
                return Err(ExchangeError::Custom(format!(
                    "lot_size for {} is zero",
                    key
                )));
            }
            let rounded = round_amount_down_to_lot(amount, market.lot_size);
            if rounded.is_zero() {
                return Err(ExchangeError::Custom(format!(
                    "amount {} rounds down to zero with lot_size {}",
                    amount, market.lot_size
                )));
            }
            Ok(rounded)
        } else {
            Err(ExchangeError::SymbolNotFound(key.to_string()))
        }
    }
}

pub fn tick_size_from_price(price: Decimal) -> Decimal {
    let abs = price.abs();
    let int_part = abs.trunc();
    let mut s = int_part.to_string();
    if let Some(idx) = s.find('.') {
        s.truncate(idx);
    }
    let s = s.trim_start_matches('-');

    let int_digits: u32 = if s.is_empty() || s == "0" {
        1
    } else {
        s.len() as u32
    };

    let sig_figs = std::cmp::max(5_u32, int_digits);
    let decimal_places = sig_figs.saturating_sub(int_digits);
    Decimal::from_i128_with_scale(1i128, decimal_places)
}

pub fn compute_lot_size_from_tick(tick_size: Decimal, product: Decimal) -> Option<Decimal> {
    if tick_size.is_zero() {
        return None;
    }
    Some(product / tick_size)
}

pub fn guess_lot_sizes(tick_size: Decimal) -> (Decimal, Decimal) {
    let p1 = Decimal::from_str("0.0001").unwrap();
    let p2 = Decimal::from_str("0.00001").unwrap();

    let lot1 = p1 / tick_size;
    let lot2 = p2 / tick_size;
    (lot1, lot2)
}

pub fn round_price_down_to_tick(price: Decimal, tick_size: Decimal) -> Decimal {
    if tick_size.is_zero() {
        return price;
    }
    let divided = price / tick_size;
    let floored = divided.floor();
    floored * tick_size
}

pub fn round_amount_down_to_lot(amount: Decimal, lot_size: Decimal) -> Decimal {
    if lot_size.is_zero() {
        return amount;
    }
    let divided = amount / lot_size;
    let floored = divided.floor();
    floored * lot_size
}

/// value ?= step (value % step == 0)
pub fn is_multiple_of(value: Decimal, step: Decimal) -> bool {
    if step.is_zero() {
        return false;
    }
    let rem = value % step;
    rem == Decimal::ZERO
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;
    use crate::info::info_client::InfoClient;

    #[test]
    fn test_tick_from_price_examples() {
        let p1 = Decimal::from_str("123.45").unwrap();
        assert_eq!(tick_size_from_price(p1), Decimal::from_str("0.01").unwrap());

        let p2 = Decimal::from_str("123456").unwrap();
        assert_eq!(tick_size_from_price(p2), Decimal::from_str("1").unwrap());

        let p3 = Decimal::from_str("0.123456").unwrap();
        // int_digits = 1 -> sig_figs = 5 -> decimal_places = 4 -> tick = 0.0001
        assert_eq!(
            tick_size_from_price(p3),
            Decimal::from_str("0.0001").unwrap()
        );
    }

    #[test]
    fn test_round_down_price_and_amount() {
        let tick = Decimal::from_str("1").unwrap(); // BTC tick_size = 1
        let lot = Decimal::from_str("0.00001").unwrap();

        let bad_price = Decimal::from_str("100000.5").unwrap();
        assert!(!is_multiple_of(bad_price, tick));
        assert_eq!(
            round_price_down_to_tick(bad_price, tick),
            Decimal::from_str("100000").unwrap()
        );

        let a1 = Decimal::from_str("0.000005").unwrap();
        assert_eq!(
            round_amount_down_to_lot(a1, lot),
            Decimal::from_str("0").unwrap()
        );

        let a2 = Decimal::from_str("0.00002").unwrap();
        assert_eq!(
            round_amount_down_to_lot(a2, lot),
            Decimal::from_str("0.00002").unwrap()
        );
    }

    #[tokio::test]
    async fn test_exchange_normalize_methods() {
        let client: InfoClient = crate::info::info_client::InfoClient::new(false, false, None)
            .await
            .unwrap();
        let ex = TickLot {
            market_cache: client.market_cache,
        };

        // price rounds down
        let p = Decimal::from_str("100000.5").unwrap();
        let np = ex.normalize_price("BTC", p).unwrap();
        assert_eq!(np, Decimal::from_str("100000").unwrap());

        // amount rounds down to zero => error
        let a = Decimal::from_str("0.000005").unwrap();
        assert!(ex.normalize_amount("BTC", a).is_err());

        // amount accepted
        let a2 = Decimal::from_str("0.00003").unwrap();
        let na2 = ex.normalize_amount("BTC", a2).unwrap();
        assert_eq!(na2, Decimal::from_str("0.00003").unwrap());
    }
}
