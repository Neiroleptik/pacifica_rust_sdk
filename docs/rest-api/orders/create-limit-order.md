# Create limit order

**Bin:**&#x20;

[Rust SDK example](../../../src/bin/order.rs)

#### Dependencies/Paths:

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
};
use pacifica_rust_sdk::models::exchange::{
    payload::order::CreateOrderPayload,
    response::order::CreateOrderResponse
};
use solana_sdk::signature::Keypair;
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};
```

**Method:**

```rust
Operation::CreateOrder.name()
```

**Endpoint:**

```rust
Operation::CreateOrder.endpoint();
```

**Sign Payload**

```rust
CreateOrderPayload {
    symbol: String,
    price: Decimal,
    amount: Decimal,
    side: OrderSide,
    tif: Tif,
    reduce_only: bool,
    client_order_id: Option<Uuid>,
    take_profit: Option<TpSlWithOrderPayload>,
    stop_loss: Option<TpSlWithOrderPayload>,
}
```

**Final Headers:**

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

**Function**:

```rust
ExchangeClient.order();
async fn order(
    &self,
    sign_payload: CreateOrderPayload,
    expiry_window: Option<u32>,
) -> Result<DefaultResponse<CreateOrderResponse>, ExchangeError>
```

**Response:**

```rust
CreateOrderResponse {
    order_id: u64,
}
```
