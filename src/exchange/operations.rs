use serde::{Deserialize, Serialize};

use crate::common::consts::API_VERSION_ENDPOINT;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubaccountCreateAction {
    Initiate,
    Confirm,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Operation {
    CreateOrder,
    CreateStopOrder,
    CreateMarketOrder,
    CancelOrder,
    CancelAllOrders,
    CancelStopOrder,
    UpdateLeverage,
    UpdateMarginMode,
    SetPositionTpsl,
    Withdraw,
    SubaccountCreate(SubaccountCreateAction),
    SubaccountTransfer,
    BindAgentWallet,
    CreateApiKey,
    RevokeApiKey,
    ListApiKeys,
    BatchOrder,
    GetPoints,
}

impl Operation {
    pub fn name(&self) -> Result<String, String> {
        let result: Result<&str, &str> = match self {
            Operation::CreateOrder => Ok("create_order"),
            Operation::CreateStopOrder => Ok("create_stop_order"),
            Operation::CreateMarketOrder => Ok("create_market_order"),
            Operation::CancelOrder => Ok("cancel_order"),
            Operation::CancelAllOrders => Ok("cancel_all_orders"),
            Operation::CancelStopOrder => Ok("cancel_stop_order"),
            Operation::UpdateLeverage => Ok("update_leverage"),
            Operation::UpdateMarginMode => Ok("update_margin_mode"),
            Operation::SetPositionTpsl => Ok("set_position_tpsl"),
            Operation::Withdraw => Ok("withdraw"),
            Operation::SubaccountCreate(action) => match action {
                SubaccountCreateAction::Initiate => Ok("subaccount_initiate"),
                SubaccountCreateAction::Confirm => Ok("subaccount_confirm"),
            },
            Operation::SubaccountTransfer => Ok("transfer_funds"),
            Operation::BindAgentWallet => Ok("bind_agent_wallet"),
            Operation::CreateApiKey => Ok("create_api_key"),
            Operation::RevokeApiKey => Ok("revoke_api_key"),
            Operation::ListApiKeys => Ok("list_api_keys"),
            Operation::BatchOrder => Err("Batch orders are not signed as a whole, \
                 but rather by its individual actions components."),
            Operation::GetPoints => Ok("get_points"),
        };

        result.map(|s| s.to_string()).map_err(|e| e.to_string())
    }

    pub fn endpoint(&self) -> String {
        let path = match self {
            Operation::CreateOrder => "/orders/create",
            Operation::CreateStopOrder => "/orders/stop/create",
            Operation::CreateMarketOrder => "/orders/create_market",
            Operation::CancelOrder => "/orders/cancel",
            Operation::CancelAllOrders => "/orders/cancel_all",
            Operation::CancelStopOrder => "/orders/stop/cancel",
            Operation::UpdateLeverage => "/account/leverage",
            Operation::UpdateMarginMode => "/account/margin",
            Operation::SetPositionTpsl => "/positions/tpsl",
            Operation::Withdraw => "/account/withdraw",
            Operation::SubaccountCreate(_) => "/account/subaccount/create",
            Operation::SubaccountTransfer => "/account/subaccount/transfer",
            Operation::BindAgentWallet => "/agent/bind",
            Operation::CreateApiKey => "/account/api_keys/create",
            Operation::RevokeApiKey => "/account/api_keys/revoke",
            Operation::ListApiKeys => "/account/api_keys",
            Operation::BatchOrder => "/orders/batch",
            Operation::GetPoints => "/get_points",
        };

        let no_version = matches!(self, Operation::GetPoints);

        if no_version {
            path.to_string()
        } else {
            format!("{API_VERSION_ENDPOINT}{path}")
        }
    }
}
