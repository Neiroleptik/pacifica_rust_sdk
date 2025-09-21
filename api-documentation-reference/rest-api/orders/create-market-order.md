# Create market order

Bin:&#x20;

[Rust SDK example](../../../src/bin/market_order.rs)

Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
};
use pacifica_rust_sdk::models::exchange::{
    payload::order::CreateMarketOrderPayload,
    response::order::CreateMarketOrderResponse
};
use solana_sdk::signature::Keypair;
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};
```

Method:

```rust
Operation::CreateMarketOrder.name()
```

Endpoint:

```rust
Operation::CreateMarketOrder.endpoint();
```

Sign Payload:

```rust
CreateMarketOrderPayload {
    symbol: String,
    amount: Decimal,
    side: OrderSide,
    slippage_percent: Decimal,
    reduce_only: bool,
    client_order_id: Option<Uuid>,
    take_profit: Option<TpSlWithOrderPayload>,
    stop_loss: Option<TpSlWithOrderPayload>,
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
ExchangeClient.market_order();
async fn market_order(
    &self,
    sign_payload: CreateMarketOrderPayload,
    expiry_window: Option<u32>,
) -> Result<DefaultResponse<CreateMarketOrderResponse>, ExchangeError>
Response:
```

Response:

```rust
CreateMarketOrderResponse = CreateOrderResponse;
CreateOrderResponse {
    order_id: u64,
}
```
