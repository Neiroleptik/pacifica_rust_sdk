# Get market info

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::market::MarketsInfoParams,
    response::market::MarketsInfoResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
```

Method:

```rust
InfoEndpoint::MarketsInfo
```

Endpoint:

```rust
InfoEndpoint::MarketsInfo.get();
```

Params:

```rust
MarketsInfoParams { };
```

Function:

```rust
InfoClient.get_markets_info();
async fn get_markets_info(
    &self,
) -> Result<DefaultResponse<MarketsInfoResponse>, ExchangeError>
```

Response:

```rust
MarketsInfoResponse = Vec<MarketModel>;
MarketModel {
    symbol: String,
    tick_size: Decimal,
    min_tick: Decimal,
    max_tick: Decimal,
    lot_size: Decimal,
    max_leverage: Decimal,
    isolated_only: bool,
    min_order_size: Decimal,
    max_order_size: Decimal,
    funding_rate: Decimal,
    next_funding_rate: Decimal,
};
```
