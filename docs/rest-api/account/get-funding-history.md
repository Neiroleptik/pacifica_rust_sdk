# Get funding history

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::account::AccountFundingHistoryParams,
    response::account::AccountFundingHistoryResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
use solana_sdk::pubkey::Pubkey;
```

Method:

```rust
InfoEndpoint::AccountFundingHistory
```

Endpoint:

```rust
InfoEndpoint::AccountFundingHistory.get();
```

Params:

```rust
AccountFundingHistoryParams { 
    account: Pubkey, 
    limit: Option<u32>, 
    offset: Option<u32> 
};
```

Function:

```rust
InfoClient.account_funding_history();
async fn account_funding_history(
    &self,
    account: Pubkey,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<DefaultResponse<AccountFundingHistoryResponse>, ExchangeError>
```

Response:

```rust
AccountFundingHistoryResponse = Vec<AccountFundingHistoryModel>;
AccountFundingHistoryModel {
    history_id: u64,
    symbol: String,
    side: OrderSide,
    amount: Decimal,
    payout: Decimal,
    rate: Decimal,
    created_at: u64,
}
```

