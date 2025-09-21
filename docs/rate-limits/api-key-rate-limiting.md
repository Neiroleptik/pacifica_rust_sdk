# API-Key rate limiting

Pacifica also offers API-Key rate limiting on APIs that allows for more flexible limits.\
For more information around API-key limits, please reach out to us in the [Discord API channel](https://discord.com/channels/1325864651816435822/1378723526957334548).\
\
API-Keys are generated via REST API. The Rust SDK provides [examples](../../src/bin/api_key.rs) for how API-Key can be generated, listed and revoked.\
\
Each account can have up to five API keys.

**Examples:**

<pre class="language-rust"><code class="lang-rust">use pacifica_rust_sdk::models::exchange::{
    payload::api_key::{
        CreateApiKeyPayload, 
        ListApiKeysPayload, 
        RevokeApiKeyPayload};
    response::api_key::{
        CreateApiKeyResponse,
        ListApiKeysResponse,
        RevokeApiKeyPayload};
use pacifica_rust_sdk::common::types::DefaultResponse;
use pacifica_rust_sdk::exchange::exchange_client::ExchangeClient;

// all specific Responses located in DefaultResponse and same structures (WebSocket, etc.)
DefaultResponse&#x3C;P> { // P = specific response struct
    success: Option&#x3C;bool>,
    data: Option&#x3C;P>, // Our response data is here, need take him and unwrap(), or .ref()/deref() if available.
    error: Option&#x3C;String>,
    code: Option&#x3C;u16>,
}

let expiry_window: Option&#x3C;u32> = None;

<strong>ExchangeClient.create_api_key {
</strong><strong>    CreateApiKeyPayload {},
</strong><strong>    expiry_window
</strong>    }; => DefaultResponse&#x3C;CreateApiKeyResponse { api_key: String }>

ExchangeClient.get_list_api_keys {
    ListApiKeysPayload {},
    expiry_window
    };  =>  DefaultResponse&#x3C;ListApiKeysResponse { active_api_keys: Vec&#x3C;String>, api_key_limit: u8 }>

ExchangeClient.revoke_api_key{
    RevokeApiKeyPayload { api_key: String },
    expiry_window
}; => DefaultResponse&#x3C;RevokeApiKeyPayload{}>
</code></pre>

Note:\
API keys are generated with a prefix for fast lookup\
Format: `"{8_char_prefix}_{base58_encoded_uuid}"`

#### Using a Pacifica API-key

Pacifica's API-Keys are used to enhance websocket rate-limiting. The default rate for an API-Key follows the same restrictions as IP-based rate limits.\
\
Pacifica API-Keys are used in the connection header to specify API-Key rate limiting. Using the Python SDK as an example,

* for Websockets, add `extra_headers={"PF-API-KEY": "your_rate_limit_key"}`into `websockets.connect`
* for REST APIs, add `"PF-API-KEY": "your_rate_limit_key"` into `headers` with `{"Content-Type": "application/json"}`
