# Pacifica Rust SDK

This is a community Rust SDK for the Pacifica exchange.\
It provides both Asynchronous REST and WebSocket clients, utilities for tick/lot handling, signing, etc., and typed models for working with the API.

***

## ⚠️ API Stability Notice

This SDK interacts with an exchange that is currently in beta.  
The API changes frequently and without prior notice, including modifications to fields, methods, and data structures.

Because of this:
- Major versions will continue to change as long as fields or methods are added, removed, or modified.
- Backward compatibility is not guaranteed between releases.
- Some modules or endpoints may change or be removed at any time.
- Always review the changelog before updating.

The SDK is actively maintained to keep up with upstream changes,  
but API instability should be expected while the exchange remains in beta.

***

* [Documentation](SUMMARY.md)
* [Changelog](CHANGELOG.md)

***

## Table of Contents

1. Installation
2. Usage example
3. Project structure

***

## Installation

Add this SDK as a dependency in `Cargo.toml`:

For the latest development version from GitHub:
```toml
[dependencies]
pacifica_rust_sdk = { git = "https://github.com/Neiroleptik/pacifica_rust_sdk.git", branch = "main" }
```

For the latest stable version on [crates.io](https://crates.io/crates/pacifica_rust_sdk):
```toml
pacifica_rust_sdk = "x.y.z" 
```
> **Note:** The GitHub version may contain the latest features and fixes, but it could be unstable.  
> The crates.io version is stable and recommended for production use.

***

## Usage example

```rust
use pacifica_rust_sdk::common::errors::ExchangeError;
use pacifica_rust_sdk::rest::rest_client::RestClient;
use pacifica_rust_sdk::info::info_client::InfoClient;
use pacifica_rust_sdk::models::info::response::market::MarketModel;

#[tokio::main]
async fn main() -> Result<(), ExchangeError> {
    // Create InfoClient for mainnet without WebSocket
    let info = InfoClient::new(
        true,        // is_mainnet
        false,       // enable_ws
        None,        // api_key
    ).await?;

    // Access market cache
    let markets: &std::collections::HashMap<String, MarketModel> = &info.market_cache;

    for (symbol, m) in markets {
        println!("{}: {:?}", symbol, m);
    }

    Ok(())
}
```

If WebSocket is enabled, you can subscribe to channels and receive live updates.

***

## Project structure

* `rest` - REST client and HTTP utilities
* `ws` - WebSocket client and subscriptions
* `info` - information client for market metadata
* `exchange` - exchange client (info client included)
* `common` - errors, tick/lot utils, helpers
* `models` - typed request/response structure
* `bin` - examples for using all SDK methods

***

## 💡 Contributions are welcome!  
If you encounter outdated structures, missing fields, or other inconsistencies -  
feel free to open an issue or submit a pull request.

***