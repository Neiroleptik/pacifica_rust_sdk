---
description: >-
  Refer to official Pacifica docs to more clear implementation. In general this
  Rust SDK is low-level client.
---

# Implementation

A brief explanation of the structure of operation creation as recreated in the SDK.

1. Create Final Request Structure [Rust SDK Implementation](../../src/common/utils.rs) \
   `async fn prepare_final_request {...}`       &#x20;
   1. Signing [Rust SDK Implementation](../../src/common/signing.rs)

## 1. Setup and Initialization:

```rust
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use rust_decimal:Decimal;
use uuid::Uuid;
use pacifica_rust_sdk::common::{
    types::{DefaultSignatureHeaders, DefaultResponse, 
            OrderSide, Tif},
    utils::get_timestamp_ms};
use pacifica_rust_sdk::exchange::operations::Operation;
use pacifica_rust_sdk::models::exchange::payload::CreateOrderPayload;


let PRIVATE_KEY = "your_private_key_here";
let keypair = Keypair.from_base58_str(PRIVATE_KEY);
let public_key = keypair.pubkey();
```

## 2. Choose Endpoint and Define Operation Type

For this example, we use the order creation endpoint. Refer to [operation-types.md](operation-types.md "mention") for a list of all types and corresponding API endpoints.

```rust
let API_URL = "https://api.pacifica.fi/api/v1/orders/create";
let operation_type = Operation::CreateOrder.name();
let operation_data = CreateOrderPayload {
    "symbol": "BTC".to_string(),
    "price": Decimal::from_str("100000"),
    "amount": Decimal::from_str("0.1"),
    "side": OrderSide::Ask,
    "tif": Tif::GTC,
    "reduce_only": False,
    "client_order_id": Some(Uuid.new_v4()),
};
```

## 3. Create Signature Header

Note that all times specified are denoted in milliseconds.\
\
The `"expiry_window"` field is optional, and defaults to 30\_000 (30 seconds) if not specified in the header.

```rust
// src/common/utils.rs
let expiry_window: u32 = 5000;
let sign_headers = DefaultSignatureHeaders {
        timestamp: get_timestamp_ms(),
        expiry_window: Some(expiry_window),
        type_field: Operation,
    };
```

## 4. Combine Header and Payload

<pre class="language-rust"><code class="lang-rust"><strong>// src/common/signing.rs
</strong><strong>pub fn prepare_message&#x3C;T: Serialize, U: Serialize>(
</strong>    header: &#x26;T,
    payload: &#x26;U,
) -> Result&#x3C;(Value, Vec&#x3C;u8>), ExchangeError> {
    let data = serde_json::to_value(header)?;
    data.is_object();
    match data {
        Value::Object(mut map) => {
            map.insert("data".to_string(), serde_json::to_value(payload)?);

            for key in ["type", "timestamp", "expiry_window"] {
                if !map.contains_key(key) {
                    map.insert(key.to_string(), Value::Null);
                }
            }
            let data = Value::Object(map);
            let sorted_data = sort_json_keys(&#x26;data);
            let message_bytes = serde_json::to_vec(&#x26;sorted_data)?;
            Ok((sorted_data, message_bytes))
        }
        _ => Err(ExchangeError::Custom(
            "header must serialize to a JSON object".to_string(),
        )),
    }
}
</code></pre>

#### In the case of our example, firstly this creates `data`:

```python
 {
     "timestamp": 1748970123456,
     "expiry_window": None,
     "type": "create_order",
     "data": {
         "symbol": "BTC",
         "price": "100000",
         "amount": "0.1",
         "side": "bid",
         "tif": "GTC",
         "reduce_only": False,
         "client_order_id": "12345678-1234-1234-1234-123456789abc"
     }
 }
```

Note that data must be in same level as other headers.

## Recursively Sort JSON Keys

<pre class="language-rust"><code class="lang-rust"><strong>// src/common/utils.rs
</strong><strong>pub fn sort_json_keys(value: &#x26;Value) -> Value {
</strong><strong>    match value {
</strong>        Value::Object(map) => {
            let mut sorted = Map::new();
            let mut keys: Vec&#x3C;String> = map.keys().cloned().collect();
            keys.sort();
            for k in keys {
                if let Some(v) = map.get(&#x26;k) {
                    sorted.insert(k, sort_json_keys(v));
                }
            }
            Value::Object(sorted)
        }
        Value::Array(arr) => {
            let new_arr = arr.iter().map(sort_json_keys).collect::&#x3C;Vec&#x3C;Value>>();
            Value::Array(new_arr)
        }
        _ => value.clone(),
    }
}
</code></pre>

In the case of our example, this creates `sorted_data`:

```python
{
     "data": {
         "amount": "0.1",
         "client_order_id": "12345678-1234-1234-1234-123456789abc",
         "price": "100000",
         "reduce_only": false,
         "side": "bid",
         "symbol": "BTC",
         "tif": "GTC"
     },
     "expiry_window": 5000,
     "timestamp": 1748970123456,
     "type": "create_order"
 }
```

Note that the recursive sorting alphabetically sorts \*all\* levels

#### 6. Create Compact JSON

## Compact JSON string with no whitespace and standardized seperators

```rust
// src/common/signing.rs
use serde_json;
use bs58;

serde_json::to_string(p)
```

In the case of our example, this creates:

```
{"data":{"amount":"0.1","client_order_id":"12345678-1234-1234-1234-123456789abc","price":"100000","reduce_only":false,"side":"bid","symbol":"BTC","tif":"GTC"},"expiry_window":5000,"timestamp":1748970123456,"type":"create_order"}
```

This ensures that all logically identical messages will always produce \*identical\* signatures

## 7. Convert to Bytes and Generate Signature

Messages are converted to UTF-8 bytes for signing. The signature generated is then converted to Base58 string for transmission.

```rust
// /src/common/signing.rs
# Convert to UTF-8 bytes
let message_bytes = serde_json::to_vec(&sorted_data)?

# Sign message bytes using your private key
let signature: Signature = keypair.sign_message(&message_bytes);

# Convert signature to Base58 string
let signature_base58 = bs58::encode(signature.as_ref()).into_string();
# Expect an output similar to:
# "5j1Vy9UqYUF2jKD9r2Lv5AoMWHJuW5a1mqVzEhC9SJL5GqbPkGEQKpW3UZmKXr4UWrHMJ5xHQFMJkZWE8J5VyA"
```

## 8. Build Final Request

Build the header with generated authentication info and combine with operation data

```rust
// src/common/utils.rs
let (_message, signature) = sign_message(&sign_headers, &sign_payload, keypair)?;
let final_headers = OperationFinalHeaders::Default(DefaultFinalHeaders {
    account: *main_pubkey,
    agent_wallet: *agent_pubkey,
    signature: PacificSignature::Simple(signature), // Simple, Hardware, Raw
    expiry_window: sign_headers.expiry_window,
    timestamp: sign_headers.timestamp,
});

let final_request: FinalRequest<P> = FinalRequest {
    // Final request flatten fields.
    headers: final_headers,
    payload: sign_payload,
};
```

In the case of our example, the final request looks like:

```python
 FinalRequest {
     "account": "6ETnufiec2CxVWTS4u5Wiq33Zh5Y3Qm6Pkdpi375fuxP",
     "agent_wallet": null,
     "signature": "5j1Vy9UqYUF2jKD9r2Lv5AoMWHJuW5a1mqVzEhC9SJL5GqbPkGEQKpW3UZmKXr4UWrHMJ",
     "timestamp": 1748970123456,
     "expiry_window": 5000,
     "symbol": "BTC",
     "price": "100000",
     "amount": "0.1",
     "side": "bid",
     "tif": "GTC",
     "reduce_only": false,
     "client_order_id": "12345678-1234-1234-1234-123456789abc"
 }
```

Response, as this example:

```rust
DefaultResponse {
    code: Option<u16>, 
    success: Option<bool>, 
    data: Option<CreateOrderResponse { order_id: u64 }>, 
    error: Option<String>
}
```

