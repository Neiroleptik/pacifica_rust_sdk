# Cancel all orders

Bin:&#x20;

[Rust SDK example](../../../src/bin/cancel_all_orders.rs)

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
};
use pacifica_rust_sdk::models::exchange::{
    payload::order::CancelAllOrdersPayload,
    response::order::CancelAllOrdersResponse
};
use solana_sdk::signature::Keypair;
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};
```

Method:

```rust
Operation::CancelAllOrders.name()
```

Endpoint:

```rust
Operation::CancelAllOrders.endpoint();
```

Sign Payload:

```rust
CancelAllOrdersPayload {
    all_symbols: bool,
    exclude_reduce_only: bool,
    symbol: Option<String>, // required if all_symbols is False
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
ExchangeClient.cancel_all_orders();
async fn cancel_all_orders(
    &self,
    sign_payload: CancelAllOrdersPayload,
    expiry_window: Option<u32>,
) -> Result<DefaultResponse<CancelAllOrdersResponse>, ExchangeError>
```

Response:

```rust
CancelAllOrdersResponse {
    cancelled_count: u32,
}
```
