# Get historical funding

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::market::FundingRateHistoryParams,
    response::market::FundingRateHistoryResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
```

Method:

```rust
InfoEndpoint::FundingRateHistory
```

Endpoint:

```rust
InfoEndpoint::FundingRateHistory.get();
```

Params:

```rust
FundingRateHistoryParams { 
    symbol: String 
};
```

Function:

```rust
InfoClient.funding_rate_history();
async fn funding_rate_history(
    &self,
    symbol: String,
) -> Result<DefaultResponse<FundingRateHistoryResponse>, ExchangeError>
```

Response:

```rust
FundingRateHistoryResponse = Vec<FundingRateHistoryModel>;
FundingRateHistoryModel {
    oracle_price: Decimal,
    bid_impact_price: Decimal,
    ask_impact_price: Decimal,
    funding_rate: Decimal,
    next_funding_rate: Decimal,
    created_at: u64,
}
```
