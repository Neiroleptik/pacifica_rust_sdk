# Create position TP/SL

Bin:&#x20;

[Rust SDK example](../../../src/bin/tpsl.rs)

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
};
use pacifica_rust_sdk::models::exchange::{
    payload::order::SetPositionTpslPayload,
    response::order::SetPositionTPSLResponse
};
use solana_sdk::signature::Keypair;
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};
```

Method:

```rust
Operation::SetPositionTpsl.name()
```

Endpoint:

```rust
Operation::SetPositionTpsl.endpoint();
```

Sign Payload:

```rust
SetPositionTpslPayload {
    pub symbol: String,
    pub side: OrderSide,
    pub take_profit: TpSlWithOrderPayload,
    pub stop_loss: TpSlWithOrderPayload,
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
ExchangeClient.set_position_tpsl();
async fn set_position_tpsl(
    &self,
    sign_payload: SetPositionTpslPayload,
    expiry_window: Option<u32>,
) -> Result<DefaultResponse<SetPositionTPSLResponse>, ExchangeError>
```

Response:

```rust
SetPositionTPSLResponse = EmptyResponseData;
```
