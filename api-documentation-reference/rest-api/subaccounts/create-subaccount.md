# Create subaccount

The subaccount creation process follows the below steps.

1. The main account must authorize the creation of a subaccount under its control
2. The subaccount must consent to being controlled by the main account
3. The API server must verify both signatures to prevent unauthorized subaccount creation

#### **Bin:**

[Rust SDK example](../../../src/bin/subaccount.rs)

**Dependencies/Paths:**

```rust
use pacifica_rust_sdk::exchange::{
    exchange_client::ExchangeClient,
    Operation,
    };
use pacifica_rust_sdk::models::{exchange::
    payload::{subaccount::{
        SubaccountInitiatePayload,
        SubaccountConfirmPayload,
        },
    },
    response::subaccount::SubaccountCreateResponse;
};
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, SubAccountFinalHeaders};
use solana_sdk::signature::Keypair;
use solana_sdk::pubkey::Pubkey;
```

**Method:**

```rust
Operation::SubaccountCreate::(SubaccountCreateAction::Initiate).name()
Operation::SubaccountCreate::(SubaccountCreateAction::Confirm).name()
```

**Endpoint:**

```rust
Operation::SubaccountCreate::(_).endpoint();
```

**Sign Payload**:

```rust
SubaccountInitiatePayload { account: Pubkey };
SubaccountConfirmPayload { signature: String };
```

**Final Headers:**

```rust
OperationFinalHeaders::SubAccountCreate(
    SubAccountFinalHeaders {
        main_account: Pubkey,
        subaccount: Pubkey,
        main_signature: PacificSignature,
        sub_signature: PacificSignature,
        timestamp: u64,
        expiry_window: Option<u32>,
    }
);
```

**Function:**

```rust
ExchangeClient.subaccount_create();
async fn subaccount_create(
    &self,
    subaccount: &Keypair,
    expiry_window: Option<u32>,
) -> Result<DefaultResponse<SubaccountCreateResponse>, 
            ExchangeError> 
```

**Response:**

```rust
SubaccountCreateResponse {}; // Empty => success is flag
```

* Status 200: Subaccount created successfully

```json
{
  "success": true,
  "data": null,
  "error": null,
  "code": null,
}
```

* Status 400: Bad request

```json
{
  "success": false,
  "data": null,
  "error": "Account already exists: CRTxBM...",
  "code": 2
}
```

* Status 500: Internal server error
