---
description: use pacifica_rust_sdk::common::tick_lot::TickLot;
---

# Tick and lot size

Both `'price'` and `'amount'` fields in order related API operations are subject to rounding, needing to be multiples tick and lot size.\
\
Generally `tick_size` is determined by the rightmost decimal place of a symbol's current price, and prices generally have five significant figures, with the exception of assets with more than six integer places, where sig.figs = #of integer places.\
\
For example:\
\
If `'price' = 123.45`, expect `'tick_size' = 0.01`\
If `'price' = 123456`, expect `'tick_size' = 1`\
\
Generally, `lot_size*tick_size = 0.0001 or 0.00001`, based on the market.\
\
For the exact implemented `tick_size` and `lot_size` of each market, call the [market info](https://docs.pacifica.fi/api-documentation/api/rest-api/markets/get-market-info) endpoint to verify.

### Rounding

Pacifica accepts requests containing `'price'` and `'amount'` fields only when they are multiples of `tick_size` and `lot_size` respectively. Any requests with incorrectly rounded `'price'` and `'amount'` fields will return `'"Internal server error","code":500'`

For example:\
\
BTC has `"tick_size": "1"`, `"lot_size": "0.00001"`\
\
A request where `"amount": "0.000005"` will return `Status 500: Internal server error`\
A request where `"price": "100_000.5"` will return `Status 500: Internal server error`

A request where `"amount": "0.00002"` will be accepted\
A request where `"price": "100_001"` will be accepted

### SDK Implementation

There is in [Rust SDK module](../src/common/tick_lot.rs) `tick_lot`:\
`path:` `src/common/tick_lot.rs` \
`use pacifica_rust_sdk::common::tick_lot.rs`\
\


`InfoClient` **creates and contains** `TickLot` with \
`market_cache`: `Vec<String, MarketInfo>` parameters.\
`InfoClient.tick_lot;`

This saves you from having to look up and match `tick_size` and `lot_size`:

```rust
TickLot.normalize_price(
symbol: str/String, 
price: Decimal
) => Decimal

TickLot.normalize_amount(
symbol: str/String, 
amount: Decimal
) => Decimal
```

`ExchangeClient` **creates and contains** `InfoClient`.\
`ExchangeClient.info_client;`
