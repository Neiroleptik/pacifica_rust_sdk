# Create limit order

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::exchange_client::ExchangeClient;
use pacifica_rust_sdk::models::exchange::payload::order::CreateOrderPayload;
use pacifica_rust_sdk::models::ws::{responses::CreateOrderResponse, requests::RequestMethod};
use pacifica_rust_sdk::common::types::{WebSocketOperationResponse, OrderSide, Tif};
use rust_decimal::Decimal;
use uuid::Uuid;
use serde_json::from_value;
```

Method:

```rust
RequestMethod:CreateOrder
```

Sign Payload:s

```rust
CreateOrderPayload {
    pub symbol: String,
    pub price: Decimal,
    pub amount: Decimal,
    pub side: OrderSide,
    pub tif: Tif,
    pub reduce_only: bool,
    pub client_order_id: Option<Uuid>,
    pub take_profit: Option<TpSlWithOrderPayload>,
    pub stop_loss: Option<TpSlWithOrderPayload>,
}
```

Usage:

```rust
let mut rx = client.request_ws_exchange_fn("create_order", sign_payload, expiry_window).await.unwrap();
match rx.recv().await {
    Ok(order_response_value) => {
        let order_response: WebSocketOperationResponse<CreateOrderResponse> = 
            from_value(order_response_value).unwrap();
    }
    Err(e) => { /* handle error */ }
}
```

Response:

```rust
CreateOrderResponse = BasicOrderActionResponse;
BasicOrderActionResponse {
    #[serde(rename = "I")]
    pub client_order_id: Option<Uuid>,
    #[serde(rename = "i")]
    pub order_id: Option<u64>,
    #[serde(rename = "s")]
    pub symbol: String,
}
```
