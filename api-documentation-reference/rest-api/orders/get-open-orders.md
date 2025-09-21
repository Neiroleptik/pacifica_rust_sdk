# Get open orders

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::account::OpenedOrdersParams,
    response::account::OpenedOrdersResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
use solana_sdk::pubkey::Pubkey;
```

Method:

```rust
InfoEndpoint::OpenedOrders
```

Endpoint:

```rust
InfoEndpoint::OpenedOrders.get();
```

Params:s

```rust
OpenedOrdersParams { 
    account: Pubkey 
};
```

Function:

```rust
InfoClient.opened_orders();
async fn opened_orders(
    &self,
    account: Pubkey,
) -> Result<DefaultResponse<OpenedOrdersResponse>, ExchangeError>
```

Response:

```rust
OpenedOrdersResponse = Vec<OpenedOrderModel>;
OpenedOrderModel {
    order_id: u64,
    client_order_id: Option<Uuid>,
    symbol: String,
    side: OrderSide,
    price: Decimal,
    initial_amount: Decimal,
    filled_amount: Decimal,
    cancelled_amount: Decimal,
    stop_price: Option<Decimal>,
    order_type: OrderType,
    stop_parent_order_id: Option<u64>,
    reduce_only: bool,
    created_at: u64,
    updated_at: u64,
}
```
