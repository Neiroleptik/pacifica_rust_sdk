# Create market order

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::exchange_client::ExchangeClient;
use pacifica_rust_sdk::models::exchange::payload::order::CreateMarketOrderPayload;
use pacifica_rust_sdk::models::ws::{responses::CreateMarketOrderResponse, requests::RequestMethod};
use pacifica_rust_sdk::common::types::{WebSocketOperationResponse, OrderSide};
use rust_decimal::Decimal;
use uuid::Uuid;
use serde_json::from_value;
```

Method:

```rust
RequestMethod:CreateMarketOrder
```

Sign Payload:

```rust
CreateMarketOrderPayload {
    pub symbol: String,
    pub amount: Decimal,
    pub side: OrderSide,
    pub slippage_percent: Decimal,
    pub reduce_only: bool,
    pub client_order_id: Option<Uuid>, // uuid:Uuid::new_v4().to_string()
    pub take_profit: Option<TpSlWithOrderPayload>,
    pub stop_loss: Option<TpSlWithOrderPayload>,
}
```

Usage:

```rust
let mut rx = client.request_ws_exchange_fn("create_market_order", sign_payload, expiry_window).await.unwrap();
match rx.recv().await {
    Ok(order_response_value) => {
        let order_response: WebSocketOperationResponse<CreateMarketOrderResponse> = from_value(order_response_value).unwrap();
    }
    Err(e) => { /* handle error */ }
}
```

Response:

```rust
CreateMarketOrderResponse = BasicOrderActionResponse;
BasicOrderActionResponse {
    #[serde(rename = "I")]
    pub client_order_id: Option<Uuid>,
    #[serde(rename = "i")]
    pub order_id: Option<u64>,
    #[serde(rename = "s")]
    pub symbol: String,
}
```
