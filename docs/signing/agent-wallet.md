# Agent Wallet

The signature verification process in both API and Websocket allow user generated "agent wallets" to sign on behalf of the original account. This is similar to the API Keys used for most leading exchanges.&#x20;

This way, API users can trade programmably without using the private key of the original account in the program.

## Generate Agent Wallets <a href="#generate-agent-wallets" id="generate-agent-wallets"></a>

Agent wallets can be generated on the [frontend](https://app.pacifica.fi/apikey), or using this Rust SDK.

**Bin:**&#x20;

[Rust SDK example](../../src/bin/agent_wallet.rs)

**Dependencies/Paths:**&#x20;

<pre class="language-rust"><code class="lang-rust">use pacifica_rust_sdk::exchange::{ 
<strong>        exchange_client::ExchangeClient, Operation, }; 
</strong>use pacifica_rust_sdk::models::{
        exchange:: payload::agent_wallet::BindAgentWalletPayload, 
        response::agent_wallet::BindAgentWalletResponse }; 
use solana_sdk::signature::Keypair; 
use pacifica_rust_sdk::common::types::{DefaultResponse, OperationFinalHeaders, DefaultFinalHeaders};
</code></pre>

**Method:**&#x20;

```rust
Operation::BindAgentWallet.name();
```

**Endpoint:**&#x20;

```rust
Operation::BindAgentWallet.endpoint();
```

**Sign Payload:**&#x20;

```rust
BindAgentWalletPayload {
    agent_wallet: Pubkey,
};
```

**Final Headers:**&#x20;

```rust
OperationFinalHeaders::Default( 
    DefaultFinalHeaders { 
    account: Pubkey, 
    agent_wallet: Option<pubkey>, 
    signature: PacificSignature, 
    timestamp: u64, 
    expiry_window: Option<u32>, 
});
```

**Function:**&#x20;

```rust
ExchangeClient.bind_agent_wallet(); 
async fn bind_agent_wallet( 
&self, 
sign_payload: BindAgentWalletPayload, 
expiry_window: Option<u32>, 
) -> Result<DefaultResponse <BindAgentWalletResponse>, ExchangeError>
```

#### Response: <a href="#use-agent-wallets" id="use-agent-wallets"></a>

```rust
BindAgentWalletResponse {}
```

## Use Agent Wallets <a href="#use-agent-wallets" id="use-agent-wallets"></a>

For all POST requests, simply use agent wallet's private key for message signing and add

`agent_wallet: [AGENT_WALLET_PUBLIC_KEY]` to the request payload.&#x20;

`account: [MAIN_ACCOUNT_PUBLIC_KEY]` is still required and **not equal** to `agent_wallet`

As an example, this [Rust SDK program](../../src/bin/basic_exchange_with_agent.rs) create ExchangeClient with agent wallet.

```rust
// src/bin/basic_exchange_with_agent.rs
ExchangeClient::new(
        is_mainnet, 
        enable_ws,  
        api_key.clone(), 
        agent_keypair, // signer = Agent Keypair
        main_pubkey,   // main_pubkey = Your main accounts pubkey (for 'account' field)
        Some(agent_pubkey), // used in 'agent_wallet' field
)
.await
.map_err(|e| format!("failed to init exchange client with agent: {:?}", e))
.unwrap();
```
