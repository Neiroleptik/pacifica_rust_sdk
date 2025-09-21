# Get positions

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::account::PositionsParams,
    response::account::PositionsResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
use solana_sdk::pubkey::Pubkey;
```

Method:

```rust
InfoEndpoint::Positions
```

Endpoint:

```rust
InfoEndpoint::Positions.get();
```

Params:

```rust
PositionsParams { 
    account: Pubkey 
};
```

Function:

```rust
InfoClient.positions();
async fn positions(
    &self,
    account: Pubkey,
) -> Result<DefaultResponse<PositionsResponse>, ExchangeError>
```

Response:

```rust
PositionsResponse = Vec<PositionModel>;
PositionModel {
    symbol: String,
    side: OrderSide,
    amount: Decimal,
    entry_price: Decimal,
    margin: Option<Decimal>,
    funding: Decimal,
    isolated: bool,
    created_at: u64,
    updated_at: u64,
}

```
