# Get prices

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::market::PricesParams,
    response::market::PricesResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
```

Method:

```rust
InfoEndpoint::Prices
```

Endpoint:

```rust
InfoEndpoint::Prices.get();
```

Params:

```rust
PricesParams { };
```

Function:

```rust
InfoClient.prices();
async fn prices(
    &self,
) -> Result<DefaultResponse<PricesResponse>, ExchangeError>
```

Response:

```rust
PricesResponse = Vec<PriceModel>;
PriceModel {
    funding: Decimal,
    mark: Decimal,
    mid: Decimal,
    next_funding: Decimal,
    open_interest: Decimal,
    oracle: Decimal,
    symbol: String,
    timestamp: u64,
    volume_24h: Decimal,
    yesterday_price: Decimal,
}
```
