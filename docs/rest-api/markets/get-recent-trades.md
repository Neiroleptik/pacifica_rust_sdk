# Get recent trades

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::market::RecentTradesParams,
    response::market::RecentTradesResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
```

Method:

```rust
InfoEndpoint::RecentTrades
```

Endpoint:

```rust
InfoEndpoint::RecentTrades.get();
```

Params:

```rust
RecentTradesParams { 
    symbol: String 
};
```

Function:

```rust
InfoClient.recent_trades();
async fn recent_trades(
    &self,
    symbol: String,
) -> Result<DefaultResponse<RecentTradesResponse>, ExchangeError>
```

Response:

```rust
RecentTradesResponse = Vec<RecentTradeModel>;
RecentTradeModel {
    event_type: OrderEventType,
    price: Decimal,
    amount: Decimal,
    side: TradeSide,
    cause: TradeCause,
    created_at: u64,
}
```
