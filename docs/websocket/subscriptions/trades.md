# Trades

Dependencies/Paths:

```rust
use pacifica_rust_sdk::ws::ws_client::WebSocketClient;
use pacifica_rust_sdk::models::ws::subscriptions::Trades;
use pacifica_rust_sdk::common::types::{WebSocketParams, WebSocketSubscription, WsMethod, Subscription};
```

Source:

```rust
SubscriptionMethod::Trades
```

Params:

```rust
WebSocketParams {
    source: SubscriptionMethod::Trades.to_string(),
    params: Trades { symbol: String },
};
```

Function:

```rust
WebSocketClient.subscribe_to_trades();
async fn subscribe_to_trades(
    &self, 
    symbol: &str
) -> Result<Subscription, ExchangeError>
```

Response:

```rust
TradesResponse = Vec<TradeModel>;
TradeModel {
    #[serde(rename = "a")]
    pub amount: Decimal,
    #[serde(rename = "c")]
    pub counter_party: Pubkey,
    #[serde(rename = "d")]
    pub side: TradeSide,
    #[serde(rename = "e")]
    pub event_type: OrderEventType,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "tc")]
    pub cause: TradeCause,
    #[serde(rename = "u")]
    pub account: Pubkey,
}
```
