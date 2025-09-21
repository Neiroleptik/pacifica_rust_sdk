# Orderbook

Dependencies/Paths:

```rust
use pacifica_rust_sdk::ws::ws_client::WebSocketClient;
use pacifica_rust_sdk::models::ws::subscriptions::OrderBook;
use pacifica_rust_sdk::common::types::{WebSocketParams, WebSocketSubscription, WsMethod, Subscription, AggLevel};
```

Source:

```rust
SubscriptionMethod::Book
```

Params:

```rust
WebSocketParams {
    source: SubscriptionMethod::Book.to_string(),
    params: OrderBook { symbol: String, agg_level: AggLevel },
};
```

Function:

```rust
WebSocketClient.subscribe_to_orderbook();
async fn subscribe_to_orderbook(
    &self, symbol: &str, 
    agg_level: AggLevel
) -> Result<Subscription, ExchangeError>
```
