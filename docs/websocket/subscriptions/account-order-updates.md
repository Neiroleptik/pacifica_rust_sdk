# Account order updates

Dependencies/Paths:

```rust
use pacifica_rust_sdk::ws::ws_client::WebSocketClient;
use pacifica_rust_sdk::models::ws::subscriptions::OrderUpdates;
use pacifica_rust_sdk::common::types::{WebSocketParams, WebSocketSubscription, WsMethod, Subscription};
use solana_sdk::pubkey::Pubkey;
```

Source:

```rust
SubscriptionMethod::AccountOrderUpdates
```

Params:

```rust
WebSocketParams {
    source: SubscriptionMethod::AccountOrderUpdates.to_string(),
    params: OrderUpdates { account: Pubkey },
};
```

Function:

```rust
WebSocketClient.subscribe_to_order_updates();
async fn subscribe_to_order_updates(
    &self, 
    account: Pubkey
) -> Result<Subscription, ExchangeError>
```

Response:

```rust
OrderUpdatesResponse = Vec<OrderUpdateModel>;
OrderUpdateModel {
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "I")]
    pub client_order_id: Option<Uuid>,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "d")]
    pub side: OrderSide,
    #[serde(rename = "ip")]
    pub initial_price: Decimal,
    #[serde(rename = "p")]
    pub average_filled_price: Decimal,
    #[serde(rename = "a")]
    pub original_amount: Decimal,
    #[serde(rename = "f")]
    pub filled_amount: Decimal,
    #[serde(rename = "os")]
    pub order_status: OrderStatus,
    #[serde(rename = "ot")]
    pub order_type: OrderType,
    #[serde(rename = "sp")]
    pub stop_price: Option<Decimal>,
    #[serde(rename = "si")]
    pub stop_order_id: Option<Decimal>,
    #[serde(rename = "r")]
    pub reduce_only: bool,
    #[serde(rename = "u")]
    pub account: Pubkey,
    #[serde(rename = "oe")]
    pub event_type: OrderEventType,
    #[serde(rename = "ut")]
    pub updated_at: u64,
    #[serde(rename = "ct")]
    pub created_at: u64,
}
```
