# Batch Order

Bin:

[Rust SDK example](../../../src/bin/batch_order.rs)

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
};
use pacifica_rust_sdk::models::exchange::{
    payload::batch_order::{BatchOrderActionPayload, BatchOrderActionType, BatchOrderActionsFinalHeaders, BatchOrderFinalRequest},
    response::batch_order::BatchOrderResponse
};
use solana_sdk::signature::Keypair;
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};
```

Method:

```rust
Operation::BatchOrder.name()
```

**Endpoint:**

```rust
Operation::BatchOrder.endpoint();
```

**Sing Payload:**

```rust
enum BatchOrderActionPayload {
    CreateOrder(CreateOrderPayload),
    CancelOrder(CancelOrderPayload),
    CreateMarketOrder(CreateMarketOrderPayload),
}
Vec<BatchOrderActionPayload>;
```

**Final Headers:**

```rust
BatchOrderActionsFinalHeaders { 
    type_field: BatchOrderActionType, 
    data: FinalRequest<BatchOrderActionPayload>,
};

BatchOrderFinalRequest {
    actions: Vec<BatchOrderActionsFinalHeaders>,
}
```

**Function**:

```rust
ExchangeClient.batch_order();
async fn batch_order(
    &self,
    orders: Vec<BatchOrderActionPayload>,
    expiry_window: Option<u32>,
) -> Result<DefaultResponse<BatchOrderResponse>, ExchangeError>
```

Response:

```rust
BatchOrderModel {
    success: bool,
    order_id: Option<u64>,
    error: Option<String>,
}

pub struct BatchOrderResponse {
    results: Vec<BatchOrderModel>,
}
```
