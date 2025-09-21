# Account balance

Dependencies/Paths:

```rust
use pacifica_rust_sdk::ws::ws_client::WebSocketClient;
use pacifica_rust_sdk::models::ws::subscriptions::Balance;
use pacifica_rust_sdk::common::types::{WebSocketParams, WebSocketSubscription, WsMethod, Subscription};
use solana_sdk::pubkey::Pubkey;
```

Method:

```rust
SubscriptionMethod::Balance
```

Params:

```rust
WebSocketParams {
    source: SubscriptionMethod::Balance.to_string(),
    params: Balance { account: Pubkey },
};
```

Function:

```rust
WebSocketClient.subscribe_to_balance();
async fn subscribe_to_balance(
    &self, 
    account: Pubkey
) -> Result<Subscription, ExchangeError>
```

Response:

```rust
BalanceResponse {
    pub total: Decimal,
    pub available: Decimal,
    pub locked: Decimal,
    #[serde(rename = "t")]
    pub timestamp: u64,
}
```
