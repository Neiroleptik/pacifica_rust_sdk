# REST

```rust
enum OperationFinalHeaders {
    Default(DefaultFinalHeaders),
    SubAccountCreate(SubAccountFinalHeaders),
}


FinalRequest<P> {
    headers: OperationFinalHeaders,
    payload: P,
}

DefaultFinalHeaders {
    account: Pubkey,
    agent_wallet: Option<Pubkey>,
    signature: PacificSignature,
    timestamp: u64, // need be equal with signature timestamp
    expiry_window: Option<u32>, // need be equal with signature expiry_window
}

SubAccountFinalHeaders {
    main_account: Pubkey,
    subaccount: Pubkey,
    main_signature: PacificSignature,
    sub_signature: PacificSignature,
    timestamp: u64,
    expiry_window: Option<u32>,
}

DefaultResponse<P> {
    success: Option<bool>,
    data: Option<P>,
    error: Option<String>,
    code: Option<u16>,
}
```
