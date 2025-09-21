# Account info

Dependencies/Paths:

```rust
use pacifica_rust_sdk::ws::ws_client::WebSocketClient;
use pacifica_rust_sdk::models::ws::subscriptions::AccountInfo;
use pacifica_rust_sdk::common::types::{WebSocketParams, WebSocketSubscription, WsMethod, Subscription};
use solana_sdk::pubkey::Pubkey;
```

Source:

```rust
SubscriptionMethod::AccountInfo
```

Params:

```rust
WebSocketParams {
    source: SubscriptionMethod::AccountInfo.to_string(),
    params: AccountInfo { account: Pubkey },
};
```

Function:

```rust
WebSocketClient.subscribe_to_account_info();
async fn subscribe_to_account_info(
    &self, 
    account: Pubkey
) -> Result<Subscription, ExchangeError>
```

#### Response:

```rust
AccountInfoResponse {
    #[serde(rename = "ae")]
    pub account_equity: Decimal,
    #[serde(rename = "as")]
    pub available_to_spend: Decimal,
    #[serde(rename = "b")]
    pub balance: Decimal,
    #[serde(rename = "f")]
    pub fee_tier: u8,
    #[serde(rename = "mu")]
    pub margin_used: Decimal,
    #[serde(rename = "oc")]
    pub orders_count: u32,
    #[serde(rename = "pb")]
    pub pending_balance: Decimal,
    #[serde(rename = "pc")]
    pub positions_count: u32,
    #[serde(rename = "sc")]
    pub stop_orders_count: u32,
    #[serde(rename = "t")]
    pub timestamp: u64,
}
```
