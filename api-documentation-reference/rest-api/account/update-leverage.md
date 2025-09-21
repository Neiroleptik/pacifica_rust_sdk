# Update leverage

#### If you specify the same leverage as set, you will get an error.

**Bin:**

[Rust SDK example](../../../src/bin/update_leverage.rs)

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
};
use pacifica_rust_sdk::models::exchange::{
    payload::market_settings::UpdateLeveragePayload,
    response::market_settings::UpdateLeverageResponse
};
use solana_sdk::signature::Keypair;
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};
```

Method:

```rust
Operation::UpdateLeverage.name()
```

Endpoint:

```rust
Operation::UpdateLeverage.endpoint();
```

Sign Payload:

```rust
UpdateLeveragePayload {
    symbol: String,
    leverage: u16,
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
ExchangeClient.update_leverage();
async fn update_leverage(
    &self,
    sign_payload: UpdateLeveragePayload,
    expiry_window: Option<u32>,
) -> Result<DefaultResponse<UpdateLeverageResponse>, ExchangeError>
```

Response:

```rust
UpdateLeverageResponse = EmptyResponseData;
```
