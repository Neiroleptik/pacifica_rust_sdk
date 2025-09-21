# Account positions

Dependencies/Paths:

```rust
use pacifica_rust_sdk::ws::ws_client::WebSocketClient;
use pacifica_rust_sdk::models::ws::subscriptions::Positions;
use pacifica_rust_sdk::common::types::{WebSocketParams, WebSocketSubscription, WsMethod, Subscription};
use solana_sdk::pubkey::Pubkey;
```

Source:

```rust
SubscriptionMethod::Positions
```

Params:

```rust
WebSocketParams {
    source: SubscriptionMethod::Positions.to_string(),
    params: Positions { account: Pubkey },
};
```

Function:

```rust
WebSocketClient.subscribe_to_positions();
async fn subscribe_to_positions(
    &self, 
    account: Pubkey
) -> Result<Subscription, ExchangeError>
```

Response:

```rust
PositionsResponse = Vec<PositionModel>;
PositionModel {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub amount: Decimal,
    #[serde(rename = "p")]
    pub entry_price: Decimal,
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "d")]
    pub side: OrderSide,
    #[serde(rename = "m")]
    pub margin: Option<Decimal>,
    #[serde(rename = "f")]
    pub funding: Decimal,
    #[serde(rename = "i")]
    pub isolated: bool,
}
```
