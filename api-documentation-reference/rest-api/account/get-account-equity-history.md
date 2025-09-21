# Get account equity history

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::account::EquityHistoryParams,
    response::account::EquityHistoryResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
```

Method:

```rust
InfoEndpoint::EquityHistory
```

Endpoint:

```rust
InfoEndpoint::EquityHistory.get();
```

Params:

```rust
EquityHistoryParams {
    account: Pubkey,
    time_range: EquityHistoryInterval,
    start_time: Option<u64>,
    end_time: Option<u64>,
    granularity_in_minutes: Option<u8>, // mb increase to u16?
    limit: Option<u32>,
    offset: Option<u32>,
}
```

Function:

```rust
InfoClient.equity_history();
async fn equity_history(
    &self,
    equity_history_params: EquityHistoryParams,
) -> Result<DefaultResponse<EquityHistoryResponse>, ExchangeError>
```

Response:

```rust
EquityHistoryResponse = Vec<EquityHistoryModel>;
EquityHistoryModel {
    account_equity: Decimal,
    pnl: Decimal,
    timestamp: u64,
}
```
