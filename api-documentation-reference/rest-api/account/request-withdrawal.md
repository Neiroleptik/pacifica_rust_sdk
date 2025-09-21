# Request withdrawal

Minimal withdrawal amount is 10$

**Bin:**&#x20;

[Rust SDK example](../../../src/bin/withdraw.rs)

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
};
use pacifica_rust_sdk::models::exchange::{
    payload::account::WithdrawPayload,
    response::account::WithdrawResponse
};
use solana_sdk::signature::Keypair;
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};
```

Method:

```rust
Operation::Withdraw.name()
```

Endpoint:

```rust
Operation::Withdraw.endpoint();
```

Sign Payload:

```rust
WithdrawPayload {
    amount: Decimal,
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
ExchangeClient.withdraw();
async fn withdraw(
    &self,
    sign_payload: WithdrawPayload,
    expiry_window: Option<u32>,
) -> Result<DefaultResponse<WithdrawResponse>, ExchangeError>
```

Response:

```rust
WithdrawResponse = EmptyResponseData;
```
