# Get order history

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::account::OrderHistoryParams,
    response::account::OrderHistoryResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
use solana_sdk::pubkey::Pubkey;
```

Method:

```rust
InfoEndpoint::OrderHistory
```

Endpoint:

```rust
InfoEndpoint::OrderHistory.get();
```

Params:

```rust
OrderHistoryParams { 
    account: Pubkey, 
    limit: Option<u32>, 
    offset: Option<u32> 
};
```

Function:

```rust
InfoClient.order_history();
async fn order_history(
    &self,
    account: Pubkey,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<DefaultResponse<OrderHistoryResponse>, ExchangeError>
```

Response:

```rust
OrderHistoryResponse = Vec<OrderHistoryModel>;
OrderHistoryModel {
    order_id: u64,
    client_order_id: Option<Uuid>,()
    symbol: String,
    side: OrderSide,
    initial_price: Decimal,
    average_filled_price: Decimal,
    amount: Decimal,
    filled_amount: Decimal,
    order_status: OrderStatus,
    order_type: OrderType,
    stop_price: Option<Decimal>,
    stop_parent_order_id: Option<u64>,
    reduce_only: bool,
    reason: Option<OrderReason>,
    created_at: u64,
    updated_at: u64,
}
```
