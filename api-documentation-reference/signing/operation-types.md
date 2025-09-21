---
description: >-
  The following table provides a list of all "type"s required by the signature
  header and their corresponding API endpoints
---

# Operation Types

#### Exchange Operation types Enum location:

`path`:  `/src/exchange/operations.rs`\
`use pacifica_rust_sdk::exchange::operations::{Operation, SubaccountCreateAction}`

| Operation Type (.name())            | API Endpoint (.endpoint())            | Enum Type                                                                                       |
| ----------------------------------- | ------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `"create_order"`                    | `/api/v1/orders/create`               | `Operation::CreateOrder`                                                                        |
| `"create_stop_order"`               | `/api/v1/orders/stop/create`          | `Operation::CreateStopOrder`                                                                    |
| `"cancel_order"`                    | `/api/v1/orders/cancel`               | `Operation::CancelOrder`                                                                        |
| `"cancel_all_orders"`               | `/api/v1/orders/cancel_all`           | `Operation::CancelAllOrders`                                                                    |
| `"cancel_stop_order"`               | `/api/v1/orders/stop/cancel`          | `Operation::CancelStopOrder`                                                                    |
| `"update_leverage"`                 | `/api/v1/account/leverage`            | `Operation::UpdateLeverage`                                                                     |
| `"update_margin_mode"`              | `/api/v1/account/margin`              | `Operation::UpdateMarginMode`                                                                   |
| `"set_position_tpsl"`               | `/api/v1/positions/tpsl`              | `Operation::SetPositionTpsl`                                                                    |
| `"withdraw"`                        | `/api/v1/account/withdraw`            | `Operation::Withdraw`                                                                           |
| `"subaccount_initiate"`             | `/api/v1/account/subaccount/create`   | `Operation::SubaccountCreate::`[`(`](#user-content-fn-1)[^1]`SubaccountCreateAction::Initiate)` |
| `"subaccount_confirm"`              | `/api/v1/account/subaccount/create`   | `Operation::SubaccountCreate::(SubaccountCreateAction::Confirm)`                                |
| `"create_market_order"`             | `/api/v1/orders/create_market_order`  | `Operation::CreateMarketOrder`                                                                  |
| **`"`**`subaccount_transfer`**`"`** | `/api/v1/account/subaccount/transfer` | `Operation::SubaccountTransfer`                                                                 |
| "`bind_agent_wallet`"               | `/api/v1/agent/bind`                  | `Operation:BindAgentWallet`                                                                     |
| `"create_api_key"`                  | `/api/v1/account/api_keys/create`     | `Operation::CreateApiKey`                                                                       |
| `"revoke_api_key"`                  | `/api/v1/account/api_keys/revoke`     | `Operation::RevokeApiKey`                                                                       |
| "`list_api_keys"`                   | `/api/v1/account/api_keys`            | `Operation::CreateApiKey`                                                                       |

Note: Pacifica's batch order endpoint\[`/api/v1/orders/batch`] does NOT have a corresponding operation type as all individual operations within the batch are signed independently with their own operation types.&#x20;

[^1]: 
