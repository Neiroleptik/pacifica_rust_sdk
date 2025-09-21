# Get order history by ID

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::account::OrderHistoryByIdParams,
    response::account::OrderHistoryByIdResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
```

Method:

```rust
InfoEndpoint::OrderHistoryById
```

Endpoint:

```rust
InfoEndpoint::OrderHistoryById.get();
```

Params:

```rust
OrderHistoryByIdParams { 
    order_id: u64 
};
```

Function:

```rust
InfoClient.order_history_by_id();
async fn order_history_by_id(
    &self,
    order_id: u64,
) -> Result<DefaultResponse<OrderHistoryByIdResponse>, ExchangeError>
```

Response:

<pre class="language-rust"><code class="lang-rust">OrderHistoryByIdResponse = Vec&#x3C;OrderHistoryByIdModel>;
<strong>OrderHistoryByIdModel {
</strong>    history_id: u64,
    order_id: u64,
    client_order_id: Option&#x3C;Uuid>,
    symbol: String,
    side: OrderSide,
    price: Decimal,
    initial_amount: Decimal,
    filled_amount: Decimal,
    event_type: OrderEventType,
    order_type: OrderType,
    order_status: OrderStatus,
    stop_price: Option&#x3C;Decimal>,
    stop_parent_order_id: Option&#x3C;u64>,
    reduce_only: bool,
    created_at: u64,
}
</code></pre>
