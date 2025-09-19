use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use solana_sdk::pubkey::Pubkey;

use crate::{common::utils::Validatable, make_validatable};

make_validatable!(BindAgentWalletPayload);
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BindAgentWalletPayload {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub agent_wallet: Pubkey,
}
