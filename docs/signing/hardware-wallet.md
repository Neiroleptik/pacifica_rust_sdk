---
description: >-
  There is actually a method for signing using a hardware wallet in the Rust
  SDK, but it is not used and has not been tested.
---

# Hardware Wallet

```rust
// src/common/signing.rs
// required tests
pub fn sign_with_hardware_wallet<T: Serialize, U: Serialize>(
    header: &T,
    payload: &U,
    hardware_wallet_path: &str,
) -> Result<(Value, String), ExchangeError> {
    let (message_value, message_bytes) = prepare_message(header, payload)?;
    let message_str = String::from_utf8(message_bytes)
        .map_err(|e| ExchangeError::LedgerSigningFailed(format!("Invalid UTF-8: {}", e)))?;
    let output = Command::new("solana")
        .arg("sign-offchain-message")
        .arg("-k")
        .arg(hardware_wallet_path)
        .arg(&message_str)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ExchangeError::LedgerSigningFailed(stderr.to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let signature_line = stdout.lines().last().ok_or(ExchangeError::NoSignature)?;
    Ok((message_value, signature_line.to_string()))
}
```

Pacifica supports hardware wallet signature authentication via Ed25519 [off-chain message signing](https://github.com/solana-labs/solana/blob/master/docs/src/proposals/off-chain-message-signing.md). To use hardware wallet, after constructing the [message bytes](https://docs.pacifica.fi/api-documentation/api/signing/implementation#id-7.-convert-to-bytes-and-generate-signature), prepend it with the `\xffsolana offchain` header together with message length, version information, etc.

Then, in the [#id-8.-build-final-request](implementation.md#id-8.-build-final-request "mention"), use `hardware`  type of PacificSignature :

```rust
let signature: PacificSignature::Hardware(signature);
```

For more details, refer to [this example](https://github.com/pacifica-fi/python-sdk/blob/2b5e629eb15d86c1a229df5d1847f5000f113ec9/rest/transfer_subaccount_fund_hardware.py#L27-L47) in the official Pacifica Python SDK.

[\
](https://docs.pacifica.fi/api-documentation/api/signing/error-handling)
