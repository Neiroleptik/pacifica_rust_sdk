# Get orderbook

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::market::OrderBookParams,
    response::market::OrderBookResponse
};
use pacifica_rust_sdk::common::types::{DefaultResponse, AggLevel};
```

Method:

```rust
InfoEndpoint::OrderBook
```

Endpoint:

```rust
InfoEndpoint::OrderBook.get();
```

Params:

```rust
OrderBookParams { 
    symbol: String, 
    agg_level: Option<AggLevel> 
};
```

Function:

```rust
InfoClient.order_book();
async fn order_book(
    &self,
    symbol: String,
    agg_level: Option<AggLevel>,
) -> Result<DefaultResponse<OrderBookResponse>, ExchangeError>
```

Response:

```rust
OrderBookResponse {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "l")]
    levels: Vec<Vec<OrderLevelModel>>,
    #[serde(rename = "t")]
    timestamp: u64,
}

OrderLevelModel {
    #[serde(rename = "p")]
    price: Decimal,
    #[serde(rename = "a")]
    amount: Decimal,
    #[serde(rename = "n")]
    num_orders: u32,
}
```
