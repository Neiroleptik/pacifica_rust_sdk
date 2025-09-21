# Subaccount fund transfer

**Minimal subaccount transfer is 10$**

**Bin:**

[Rust SDK example](../../../src/bin/subaccount_transfer.rs)

**Dependencies/Paths:**

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
    };
use pacifica_rust_sdk::models::{exchange::
    payload::subaccount::SubaccountTransferPayload,
    response::subaccount::SubaccountTransferResponse
};
use solana_sdk::signature::Keypair;
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};

```

**Method:**

```rust
Operation::SubaccountTransfer.name()
```

**Endpoint:**

```rust
Operation::SubaccountTransfer.endpoint();
```

**Sign Payload**:

```rust
SubaccountTransferPayload { 
    to_account: Pubkey, 
    amount: Decimal 
};
```

**Final Headers:**

```rust
OperationFinalHeaders::Default(
    DefaultFinalHeaders {
        account: Pubkey,
        agent_wallet: Option<Pubkey>,
        signature: PacificSignature,
        timestamp: u64,
        expiry_window: Option<u32>,
    }
);
```

**Function:**

```rust
ExchangeClient.subaccount_transfer();
async fn subaccount_transfer(
        &self,
        sign_payload: SubaccountTransferPayload,
        expiry_window: Option<u32>,
) -> Result<DefaultResponse
   <SubaccountTransferResponse>, ExchangeError>
```

**Response:**

```rust
SubaccountTransferResponse {
    success: bool,
    error: Option<String>,
};
```
