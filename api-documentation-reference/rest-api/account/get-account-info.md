# Get account info

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::account::AccountParams,
    response::account::AccountResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
use solana_sdk::pubkey::Pubkey;
```

Method:

```rust
InfoEndpoint::Account
```

Endpoint:

```rust
InfoEndpoint::Account.get();
```

Params:

```rust
AccountParams { 
    account: Pubkey 
};
```

Function:

```rust
InfoClient.account();
async fn account(
    &self,
    account: Pubkey,
) -> Result<DefaultResponse<AccountResponse>, ExchangeError>
```

Response:

```rust
AccountResponse {
    balance: Decimal,
    fee_level: u8,
    account_equity: Decimal,
    available_to_spend: Decimal,
    pending_balance: Decimal,
    total_margin_used: Decimal,
    positions_count: u32,
    orders_count: u32,
    stop_orders_count: u32,
    updated_at: u64,
}
```
