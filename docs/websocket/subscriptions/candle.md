# Candle

Dependencies/Paths:

```rust
use pacifica_rust_sdk::ws::ws_client::WebSocketClient;
use pacifica_rust_sdk::models::ws::subscriptions::Candle;
use pacifica_rust_sdk::common::types::{WebSocketParams, WebSocketSubscription, WsMethod, Subscription, Interval};
```

Source:

```rust
SubscriptionMethod::Candle
```

Params:

```rust
WebSocketParams {
    source: SubscriptionMethod::Candle.to_string(),
    params: Candle { symbol: String, interval: Interval },
};
```

Function:

```rust
WebSocketClient.subscribe_to_candle();
async fn subscribe_to_candle(
    &self, 
    symbol: &str, 
    interval: Interval
) -> Result<Subscription, ExchangeError>
```

Response:

```rust
CandleResponse {
    #[serde(rename = "t")]
    pub start_time: u64,
    #[serde(rename = "T")]
    pub end_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: Interval,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "v")]
    pub volume: Decimal,
    #[serde(rename = "n")]
    pub trades: u64,
}

```
