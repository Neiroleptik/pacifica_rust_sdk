---
description: use pacifica_rust_sdk::info::info_client::InfoClient;
---

# InfoClient

### Info models (except defaults) located at:

```rust
use pacifica_rust_sdk::models::info::{params, response};
```

### Client

<pre class="language-rust"><code class="lang-rust">
use pacifica_rust_sdk::{
    common::{ utils::tick_lot::TickLot, errors::ExchangeError }
    rest::rest_client::RestClient,
    ws::ws_client::WebSocketClient,
    models::info::response::market::MarketModel
}


InfoClient {
<strong>    pub base_url: &#x26;'static str,
</strong>    pub market_cache: HashMap&#x3C;String, MarketModel>,
    pub tick_lot_utils: TickLot,
    pub web_socket_client: Option&#x3C;WebSocketClient>,
    api_key: Option&#x3C;String>,
    default_headers: HeaderMap,
    http_client: RestClient,
}

impl InfoClient {
    pub async fn new(
        is_mainnet: bool,
        enable_ws: bool,
        api_key: Option&#x3C;String>,
    ) -> Result&#x3C;Self, ExchangeError>
</code></pre>

### Binary Examples:

Rest: [Rust SDK Example](src/bin/info.rs)

With WebSocket: [WS Rust SDK Example](src/bin/ws_info.rs)
