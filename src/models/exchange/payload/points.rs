use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use solana_sdk::pubkey::Pubkey;

use crate::{common::utils::Validatable, make_validatable};

make_validatable!(GetPointsPayload);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPointsPayload {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub user: Pubkey,
}
