# REST API

Mainnet Pacifica Rest URL: [**https://api.pacifica.fi/api**](https://api.pacifica.fi/api)

Testnet Pacifica Rest URL: [**https://test-api.pacifica.fi/api**](https://test-api.pacifica.fi/api)

Next, the structures of `params/payload` and `response` will be provided, as well as the dependencies and the method that needs to be called. The same **Request Structure and Default Response** is used everywhere, except for **SubAccount** and **BatchOrder**.

```rust
DefaultResponse<P> { // Where P is specific endpoint response like CreateOrderResponse
    pub success: Option<bool>,
    pub data: Option<P>,
    pub error: Option<String>,
    pub code: Option<u16>,
}

// Batch Order Final Request located at  
// pacifica_rust_sdk::models::exchange::payload::batch_order
FinalRequest<P> { // Where P is specific Operation Payload, like CreateOrderPayload  
    pub headers: OperationFinalHeaders,
    pub payload: P,
}

// Thats a default Final Headers structures. 
enum OperationFinalHeaders {
    Default(DefaultFinalHeaders),
    SubAccountCreate(SubAccountFinalHeaders),
}

DefaultFinalHeaders {
    pub account: Pubkey,
    pub agent_wallet: Option<Pubkey>,
    pub signature: PacificSignature,
    pub timestamp: u64, // need be equal with signature timestamp
    pub expiry_window: Option<u32>, // need be equal with signature expiry_window
}
```
