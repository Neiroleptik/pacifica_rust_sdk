# Get trade history

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::account::TradesHistoryParams,
    response::account::TradesHistoryResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
use solana_sdk::pubkey::Pubkey;
```

Method:

```rust
InfoEndpoint::TradesHistory
```

Endpoint:

```rust
InfoEndpoint::TradesHistory.get();
```

Params:

```rust
TradesHistoryParams { 
    account: Pubkey, 
    symbol: Option<String>, 
    start_time: Option<u64>, 
    end_time: Option<u64>, 
    limit: Option<u32>, 
    offset: Option<u32> 
};
```

Function:

```rust
InfoClient.trade_history();
async fn trade_history(
    &self,
    account: Pubkey,
    symbol: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<DefaultResponse<TradesHistoryResponse>, ExchangeError>
```

Response:

```rust
TradesHistoryResponse = Vec<TradeHistoryModel>;
TradeHistoryModel {
    history_id: u64,
    order_id: u64,
    client_order_id: Option<Uuid>, // Uuid.to_string()
    symbol: String,
    amount: Decimal,
    price: Decimal,
    entry_price: Decimal,
    fee: Decimal,
    pnl: Decimal,
    event_type: OrderEventType,
    side: TradeSide,
    created_at: u64,
    counter_party: Pubkey,
    cause: TradeCause,
}
```
