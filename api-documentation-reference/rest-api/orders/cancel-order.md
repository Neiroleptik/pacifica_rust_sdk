# Cancel order

Bin:&#x20;

[Rust SDK example](../../../src/bin/cancel_order.rs)

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
};
use pacifica_rust_sdk::models::exchange::{
    payload::order::CancelOrderPayload,
    response::order::CancelOrderResponse
};
use solana_sdk::signature::Keypair;
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};
```

Method:

```rust
Operation::CancelOrder.name()
```

Endpoint:

```rust
Operation::CancelOrder.endpoint();
```

Sign Payload:

```rust
 CancelOrderPayload {
    symbol: String,
    // Required one of order_id or client_order_id
    order_id: Option<u64>,
    client_order_id: Option<Uuid>,
}
```

Final Headers:

```rust
OperationFinalHeaders::Default(
    DefaultFinalHeaders {
        account: Pubkey,
        agent_wallet: Option<Pubkey>,
        signature: PacificSignature,
        timestamp: u64,
        expiry_window: Option<u32>,
    }
);
```

Function:

```rust
ExchangeClient.cancel_order();
async fn cancel_order(
    &self,
    sign_payload: CancelOrderPayload,
    expiry_window: Option<u32>,
) -> Result<DefaultResponse<CancelOrderResponse>, ExchangeError>
```

Response:

```rust
CancelOrderResponse = EmptyResponseData;
```
