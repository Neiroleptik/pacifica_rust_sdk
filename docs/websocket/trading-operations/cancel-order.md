# Cancel order

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::exchange_client::ExchangeClient;
use pacifica_rust_sdk::models::exchange::payload::order::CancelOrderPayload;
use pacifica_rust_sdk::models::ws::{responses::CancelOrderResponse, requests::RequestMethod};
use pacifica_rust_sdk::common::types::WebSocketOperationResponse;
use uuid::Uuid;
use serde_json::from_value;
```

Method:

```rust
RequestMethod:CancelOrder
```

Sign Payload:

```rust
CancelOrderPayload {
    pub symbol: String,
    // Required one of order_id or client_order_id
    pub order_id: Option<u64>,
    pub client_order_id: Option<Uuid>,
}

```

Usage:

```rust
let mut rx = client.request_ws_exchange_fn("cancel_order", sign_payload, expiry_window).await.unwrap();
match rx.recv().await {
    Ok(order_response_value) => {
        let order_response: WebSocketOperationResponse<CancelOrderResponse> = from_value(order_response_value).unwrap();
    }
    Err(e) => { /* handle error */ }
}
```

Response:

```rust
CancelOrderResponse = BasicOrderActionResponse;
BasicOrderActionResponse {
    #[serde(rename = "I")]
    pub client_order_id: Option<Uuid>,
    #[serde(rename = "i")]
    pub order_id: Option<u64>,
    #[serde(rename = "s")]
    pub symbol: String,
}
```
