# Cancel stop order

Bin:&#x20;

[Rust SDK example](../../../src/bin/cancel_stop_order.rs)

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
};
use pacifica_rust_sdk::models::exchange::{
    payload::order::CancelStopOrderPayload,
    response::order::CancelStopOrderResponse
};
use solana_sdk::signature::Keypair;
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};
```

Method:

```rust
Operation::CancelStopOrder.name()
```

Endpoint:

```rust
Operation::CancelStopOrder.endpoint();
```

Sign Payload:

```rust
CancelStopOrderPayload {
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
ExchangeClient.cancel_stop_order();
async fn cancel_stop_order(
    &self,
    sign_payload: CancelStopOrderPayload,
    expiry_window: Option<u32>,
) -> Result<DefaultResponse<CancelStopOrderResponse>, ExchangeError>
```

Response:

```rust
CancelStopOrderResponse = EmptyResponseData;
```
