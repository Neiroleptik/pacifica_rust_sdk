use serde::{Deserialize, Serialize};

use crate::{common::utils::Validatable, make_validatable};

make_validatable!(UpdateMarginModePayload);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMarginModePayload {
    pub symbol: String,
    pub is_isolated: bool,
}

make_validatable!(UpdateLeveragePayload);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateLeveragePayload {
    pub symbol: String,
    pub leverage: u16,
}
