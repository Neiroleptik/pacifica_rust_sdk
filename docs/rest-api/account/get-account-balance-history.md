# Get account balance history

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::account::BalanceHistoryParams,
    response::account::BalanceHistoryResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
use solana_sdk::pubkey::Pubkey;
```

Method:

```rust
InfoEndpoint::BalanceHistory
```

Endpoint:

```rust
InfoEndpoint::BalanceHistory.get();
```

Params:

```rust
BalanceHistoryParams { 
    account: Pubkey, 
    limit: Option<u32>, 
    offset: Option<u32> 
};
```

Function:

```rust
InfoClient.balance_history();
async fn balance_history(
    &self,
    account: Pubkey,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<DefaultResponse<BalanceHistoryResponse>, ExchangeError>
```

Response:

```rust
BalanceHistoryResponse = Vec<BalanceHistoryModel>;
BalanceHistoryModel {
    amount: Decimal,
    balance: Decimal,
    pending_balance: Decimal,
    event_type: AccountEventType,
created_at: u64,
}
```
