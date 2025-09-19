use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use solana_sdk::pubkey::Pubkey;

use crate::{common::utils::Validatable, make_validatable};

make_validatable!(SubaccountInitiatePayload);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubaccountInitiatePayload {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub account: Pubkey,
}

make_validatable!(SubaccountConfirmPayload);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubaccountConfirmPayload {
    pub signature: String,
}

make_validatable!(SubaccountTransferPayload);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubaccountTransferPayload {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub to_account: Pubkey,
    pub amount: Decimal,
}
