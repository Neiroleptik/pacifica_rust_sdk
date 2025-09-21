# Get account settings

Account settings will not return anything, and leverage will be equal to the maximum leverage of the market - until you change the leverage.

Dependencies/Paths:

```rust
use pacifica_rust_sdk::info::{
    info_client::InfoClient,
    info_endpoint::InfoEndpoint,
};
use pacifica_rust_sdk::models::info::{
    params::account::AccountSettingsParams,
    response::account::AccountSettingsResponse
};
use pacifica_rust_sdk::common::types::DefaultResponse;
use solana_sdk::pubkey::Pubkey;
```

Method:

```rust
InfoEndpoint::AccountSettings
```

Endpoint:

```rust
InfoEndpoint::AccountSettings.get();
```

Params:

```rust
AccountSettingsParams { 
    account: Pubkey 
};
```

Function:

```rust
InfoClient.account_settings();
async fn account_settings(
    &self,
    account: Pubkey,
) -> Result<DefaultResponse<AccountSettingsResponse>, ExchangeError>
```

Response:

```rust
AccountSettingsResponse = Vec<AccountSettingsModel>;
AccountSettingsModel {
    symbol: String,
    isolated: bool,
    leverage: Decimal,
    created_at: u64,
    updated_at: u64,
}
```

