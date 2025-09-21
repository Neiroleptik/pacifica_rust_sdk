# Update margin mode

Bin:&#x20;

[Rust SDK example](../../../src/bin/update_margin_mode.rs)

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
};
use pacifica_rust_sdk::models::exchange::{
    payload::market_settings::UpdateMarginModePayload,
    response::market_settings::UpdateMarginModeResponse
};
use solana_sdk::signature::Keypair;
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};
```

Method:

```rust
Operation::UpdateMarginMode.name()
```

Endpoint:

```rust
Operation::UpdateMarginMode.endpoint();
```

Sign Payload:

```rust
UpdateMarginModePayload {
    symbol: String,
    is_isolated: bool,
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
ExchangeClient.update_margin_mode();
async fn update_margin_mode(
    &self,
    sign_payload: UpdateMarginModePayload,
    expiry_window: Option<u32>,
) -> Result<DefaultResponse<UpdateMarginModeResponse>, ExchangeError>
```

Response:

```rust
UpdateMarginModeResponse = EmptyResponseData;
```
