# Prices

Dependencies/Paths:

```rust
use pacifica_rust_sdk::ws::ws_client::WebSocketClient;
use pacifica_rust_sdk::models::ws::subscriptions::Prices;
use pacifica_rust_sdk::common::types::{WebSocketParams, WebSocketSubscription, WsMethod, Subscription};
```

Source:

```rust
SubscriptionMethod::Prices
```

Params:

```rust
WebSocketParams {
    source: SubscriptionMethod::Prices.to_string(),
    params: Prices {},
};
```

Function:

<pre class="language-rust"><code class="lang-rust">WebSocketClient.subscribe_to_prices();
async fn subscribe_to_prices(
<strong>    &#x26;self,
</strong>) -> Result&#x3C;Subscription, ExchangeError>
</code></pre>

Response:

```rust
PricesResponse = Vec<PriceModel>;
PriceModel {
    pub funding: Decimal,
    pub mark: Decimal,
    pub mid: Decimal,
    pub next_funding: Decimal,
    pub open_interest: Decimal,
    pub oracle: Decimal,
    pub symbol: String,
    pub timestamp: u64,
    pub volume_24h: Decimal,
    pub yesterday_price: Decimal,
}
```
