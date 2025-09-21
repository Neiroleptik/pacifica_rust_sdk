# Account leverage

Dependencies/Paths:

```rust
use pacifica_rust_sdk::ws::ws_client::WebSocketClient;
use pacifica_rust_sdk::models::ws::subscriptions::Leverage;
use pacifica_rust_sdk::common::types::{WebSocketParams, WebSocketSubscription, WsMethod, Subscription};
use solana_sdk::pubkey::Pubkey;
```

Source:

```rust
SubscriptionMethod::Leverage
```

Params:

```rust
WebSocketParams {
    source: SubscriptionMethod::Leverage.to_string(),
    params: Leverage { account: Pubkey },
};
```

**Function:**

```rust
WebSocketClient.subscribe_to_leverage();
async fn subscribe_to_leverage(
    &self, 
    account: Pubkey
) -> Result<Subscription, ExchangeError>
```

Response:

```rust
LeverageResponse {
    #[serde(rename = "u")]
    pub account: Pubkey,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "l")]
    pub leverage: Decimal,
    #[serde(rename = "t")]
    pub timestamp: u64,
}
```
