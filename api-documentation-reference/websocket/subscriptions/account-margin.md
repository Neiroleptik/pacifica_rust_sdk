# Account margin

Dependencies/Paths:

```rust
use pacifica_rust_sdk::ws::ws_client::WebSocketClient;
use pacifica_rust_sdk::models::ws::subscriptions::Margin;
use pacifica_rust_sdk::common::types::{WebSocketParams, WebSocketSubscription, WsMethod, Subscription};
use solana_sdk::pubkey::Pubkey;
```

Source:

```rust
SubscriptionMethod::Margin
```

Params:

```rust
WebSocketParams {
    source: SubscriptionMethod::Margin.to_string(),
    params: Margin { account: Pubkey },
};
```

Function:

```rust
WebSocketClient.subscribe_to_margin();
async fn subscribe_to_margin(
    &self, 
    account: Pubkey
) -> Result<Subscription, ExchangeError>
```

Response:

```rust
MarginResponse {
    #[serde(rename = "u")]
    pub account: Pubkey,
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub margin_mode: bool,
}
```
