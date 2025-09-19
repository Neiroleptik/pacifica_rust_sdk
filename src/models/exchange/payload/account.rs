use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{common::utils::Validatable, make_validatable};

make_validatable!(WithdrawPayload);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WithdrawPayload {
    pub amount: Decimal,
}
