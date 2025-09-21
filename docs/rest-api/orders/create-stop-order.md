# Create stop order

Bin:&#x20;

[Rust SDK example](https://app.gitbook.com/u/dJuR2dIjmNb6xwUdoFRDdN1z9SE3)

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
};
use pacifica_rust_sdk::models::exchange::{
    payload::order::CreateStopOrderPayload,
    response::order::CreateStopOrderResponse
};
use solana_sdk::signature::Keypair;
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};
```

Method:

```rust
Operation::CreateStopOrder.name()
```

Endpoint:

```rust
Operation::CreateStopOrder.endpoint();
```

Sign Payload:

```rust
CreateStopOrderPayload {
    symbol: String,
    side: OrderSide,
    reduce_only: bool,
    stop_order: TpSlAlonePayload,
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
ExchangeClient.stop_order();
async fn stop_order(
    &self,
    sign_payload: CreateStopOrderPayload,
    expiry_window: Option<u32>,
) -> Result<DefaultResponse<CreateStopOrderResponse>, ExchangeError>
```

Response:

```rust
CancelStopOrderResponse = EmptyResponseData;
```
