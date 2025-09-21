# Account trades

Dependencies/Paths:

```rust
use pacifica_rust_sdk::ws::ws_client::WebSocketClient;
use pacifica_rust_sdk::models::ws::subscriptions::AccountTrades;
use pacifica_rust_sdk::common::types::{WebSocketParams, WebSocketSubscription, WsMethod, Subscription};
use solana_sdk::pubkey::Pubkey;
```

Source:

```rust
SubscriptionMethod::AccountTrades
```

Params:

```rust
WebSocketParams {
    source: SubscriptionMethod::AccountTrades.to_string(),
    params: AccountTrades { account: Pubkey },
};
```

**Function:**

```rust
WebSocketClient.subscribe_to_account_trades();
async fn subscribe_to_account_trades(
    &self, 
    account: Pubkey
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
