---
description: use pacifica_rust_sdk::exchange::exchange_client::ExchangeClient;
---

# ExchangeClient

### Exchange models (except defaults) located at:

```rust
use pacifica_rust_sdk::models::exchange::{payload, response};
```

### Client

```rust
use pacifica_rust_sdk::info::info_client::InfoClient;
use pacifica_rust_sdk::rest::rest_client::RestClient;
use solana_sdk::{signer::Keypair, pubkey::Pubkey};

ExchangeClient {
    pub base_url: &'static str,
    pub info_client: InfoClient,
    signer_keypair: Keypair,
    main_pubkey: Pubkey,
    agent_pubkey: Option<Pubkey>,
    pub api_key: Option<String>,
    http_client: RestClient,
    default_headers: HeaderMap,
}

impl ExchangeClient {
    pub async fn new(
        is_mainnet: bool,
        enable_ws: bool,
        api_key: Option<String>,
        signer_keypair: Keypair,
        main_pubkey: Pubkey,
        agent_pubkey: Option<Pubkey>,
    ) -> Result<Self, ExchangeError>
```

### Binary Examples:

`Rest`: [Rust SDK Example](../src/bin/basic_exchange.rs)\
`Rest with Agent`: [Rust SDK Example](../src/bin/basic_exchange_with_agent.rs)\
`With WebSocket:` [Rust SDK Example](../src/bin/ws_exchange.rs)
