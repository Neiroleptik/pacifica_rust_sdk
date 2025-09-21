# Cancel all orders

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::exchange_client::ExchangeClient;
use pacifica_rust_sdk::models::exchange::payload::order::CancelAllOrdersPayload;
use pacifica_rust_sdk::models::ws::{responses::CancelAllOrdersResponse, requests::RequestMethod};
use pacifica_rust_sdk::common::types::WebSocketOperationResponse;
use serde_json::from_value;
```

Method:

```rust
RequestMethod:CancelAllOrders
```

Sign Payload:

```rust
CancelAllOrdersPayload {
    pub all_symbols: bool,
    pub exclude_reduce_only: bool,
    pub symbol: Option<String>, // required if all_symbols is False
}
```

Usage:

```rust
let mut rx = client.request_ws_exchange_fn("cancel_all_orders", sign_payload, expiry_window).await.unwrap();
match rx.recv().await {
    Ok(order_response_value) => {
        let order_response: WebSocketOperationResponse<CancelAllOrdersResponse> = from_value(order_response_value).unwrap();
    }
    Err(e) => { /* handle error */ }
}
```

Response:

```rust
CancelAllOrdersResponse {
    cancelled_count: u32,
}
```
