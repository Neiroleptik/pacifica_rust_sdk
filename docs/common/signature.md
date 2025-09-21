# Signature

```rust
enum PacificSignature {
    Simple(String),
    Hardware(HardwareWalletSignature),
    Raw(RawSignature),
}


RawSignature {
    type_field: "raw", // private
    signature: String,
}


HardwareWalletSignature {
    type_field: "hardware", // private
    signature: String,
}

```

```rust
DefaultSignatureHeaders {
    timestamp: u64,
    type_field: String,
    expiry_window: Option<u32>,
}
```
