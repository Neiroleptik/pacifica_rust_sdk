# Get kline (candle) data

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::market::KlineParams,
    response::market::KlineResponse
};
use pacifica_rust_sdk::common::types::{DefaultResponse, Interval};
```

Method:

```rust
InfoEndpoint::Kline
```

Endpoint:

```rust
InfoEndpoint::Kline.get();
```

Params:

```rust
KlineParams { 
    symbol: String, 
    interval: Interval, 
    start_time: u64, 
    end_time: Option<u64> 
};
```

Function:

```rust
InfoClient.kline();
async fn kline(
    &self,
    symbol: String,
    interval: Interval,
    start_time: u64,
    end_time: Option<u64>,
) -> Result<DefaultResponse<KlineResponse>, ExchangeError>
```

Response:

```rust
KlineResponse = Vec<CandleModel>;
CandleModel {
    #[serde(rename = "t")]
    start_time: u64,
    #[serde(rename = "T")]
    end_time: u64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "i")]
    interval: Interval,
    #[serde(rename = "o")]
    open: Decimal,
    #[serde(rename = "c")]
    close: Decimal,
    #[serde(rename = "h")]
    high: Decimal,
    #[serde(rename = "l")]
    low: Decimal,
    #[serde(rename = "v")]
    volume: Decimal,
    #[serde(rename = "n")]
    trades: u64,
}
```
