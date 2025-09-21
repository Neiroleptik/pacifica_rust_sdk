# Error Codes

## Self Errors Enum

Check [Rust SDK errors.rs](../src/common/errors.rs)

## WebSocket and Rest errors

Check [Rust SDK types.ts](../src/common/types.rs)

#### WebSocket Exchange

The use of **WS** methods in _`ExchangeClient`_ for an **operation request** comes with an issue:

If you receive a error , then the  `id` **field** is not passed by the server, and the `recv` function cannot wait for a response if it is simply left in `.await()`.

**Reason**: Calling the operation request function via WS creates a tunnell with a shared connection, identified by `request_id`. The common loop receiver function cannot recognize the association of the server’s message. \
With a proper implementation, this should not become a problem.

#### WebSocket Info

If the subscription parameters contain invalid data (even if the method is correct, but the params format/data is invalid — for example:

```json
{
  "method": "subscribe",
  "params": {
    "method": "trades",
    "symbol": "LBTC"
  } 
}

```

, the subscription will not return an error.

Instead, you must wait for the confirmation message:

```json
// As an example: 
// But you don't receive this message, because symbol invalid

{
  "channel": "subscribe",
  "data": {
    "method": "trades",
    "symbol": "LBTC"
   }
}
```

If this message is not received within a reasonable response time (a few seconds), the subscription should be considered unsuccessful.

#### Rest

Nothing to add.

### **Referrence:**

```rust
// Where R is specific request response struct :
type info_sub_response<R> = DefaultWebSocketMsg<DefaultResponse<R>>;
type exchange_ws_response<R> = DefaultWebSocketMsg<WebSocketOperationResponse<R>>;
type rest_response = DefaultResponse<R>

let ws_info_error_field = info_sub_response.data.error.uwrap();
let ws_operation_err_field = info_sub_response.data.err.uwrap();
let rest_error_field = rest_response.error.unwrap()

// src/common/types
DefaultResponse<P> {
    pub success: Option<bool>,
    pub data: Option<P>,
    pub error: Option<String>,
    pub code: Option<u16>,
}

DefaultWebSocketMsg<R> {
    pub channel: String,
    pub data: R,
}

WebSocketOperationResponse<R> {
    pub code: u16,
    pub data: Option<R>,
    pub err: Option<String>,
    pub id: Option<Uuid>,
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}
```



HTTP Status Codes

| 400 | Bad Request                             |
| --- | --------------------------------------- |
| 401 | Unauthorized                            |
| 403 | Forbidden                               |
| 404 | Not Found                               |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error                   |
| 504 | Gateway Timeout                         |

#### Error Codes and Messages

**Authentication & Authorization (400, 403)**

| Code | Error Message            | Description                                                       |
| ---- | ------------------------ | ----------------------------------------------------------------- |
| 400  | "Invalid signature"      | Signature format incorrect                                        |
| 400  | "Invalid message"        | Message format or content invalid                                 |
| 400  | "Invalid public key"     | Public key invalid                                                |
| 400  | "Verification failed"    | Signature verification process failed                             |
| 403  | "Signer not whitelisted" | Account not authorized to use the exchange (requires access code) |

**Account Errors**

| Code | Error Message                       | Description                                     |
| ---- | ----------------------------------- | ----------------------------------------------- |
| 1    | "Account not found: {address}"      | User account does not exist                     |
| 2    | "Account already exists: {address}" | Account creation failed - already exists        |
| 40   | "Not a main account: {address}"     | Operation requires main account, not subaccount |

**Balance & Trading Errors**

| Code | Error Message                                                                 | Description                                |
| ---- | ----------------------------------------------------------------------------- | ------------------------------------------ |
| 5    | "Insufficient balance for {address}: {required} (account value: {available})" | Not enough available balance for operation |
| 9    | "Over withdrawal: balance {balance} amount {amount}"                          | Withdrawal exceeds available balance       |
| 29   | "Withdraw amount too low: {amount}"                                           | Withdrawal below minimum threshold         |
| 31   | "Daily withdraw limit exceeded for {address}: {amount} > {limit}"             | Daily withdrawal limit reached             |

**Order Management Errors**

| Code | Error Message                                               | Description                                  |
| ---- | ----------------------------------------------------------- | -------------------------------------------- |
| 6    | "Order not found for {address}: {order\_id}"                | Order ID does not exist                      |
| 7    | "Order amount too low for {address}: {amount} < {minimum}"  | Order size below minimum                     |
| 8    | "Order amount too high for {address}: {amount} > {maximum}" | Order size above maximum                     |
| 10   | "Open order limit reached for {address}: {limit}"           | Too many open orders                         |
| 20   | "Invalid order type"                                        | Unsupported order type                       |
| 36   | "Duplicate client order id: {client\_order\_id}"            | Client order ID already used                 |
| 37   | "Unused client order id: {client\_order\_id}"               | Client order ID not found (for cancellation) |

**Trading & Position Errors**

| Code | Error Message                                                                                      | Description                                   |
| ---- | -------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| 3    | "Book not found: {symbol}"                                                                         | Trading pair does not exist                   |
| 4    | "Invalid tick level: {tick}"                                                                       | Invalid tick size                             |
| 11   | "Cannot decrease leverage with open position.: {leverage}"                                         | Cannot reduce leverage with open positions    |
| 12   | "Cannot update margin with open position for {address}: isolated {isolated}"                       | Cannot change margin mode with open positions |
| 13   | "Position not found for {address}: {symbol}"                                                       | No position exists for symbol                 |
| 28   | "Immediate liquidation for {symbol}: size {amount} tick level {tick} safe tick level {safe\_tick}" | Order would cause immediate liquidation       |
| 30   | "Price too far from mark: {price} {mark\_price}"                                                   | Order price too far from market price         |

**Stop Orders**

| Code | Error Message                         | Description                    |
| ---- | ------------------------------------- | ------------------------------ |
| 16   | "Invalid stop tick: {tick}"           | Stop price tick level invalid  |
| 17   | "Invalid stop order side"             | Stop order direction invalid   |
| 18   | "Invalid stop order amount: {amount}" | Stop order size invalid        |
| 19   | "Invalid stop order reduce only"      | Invalid reduce-only stop order |

**Reduce-Only Orders**

| Code | Error Message                                                                                  | Description                            |
| ---- | ---------------------------------------------------------------------------------------------- | -------------------------------------- |
| 21   | "Invalid reduce-only order side: must be opposite to position side"                            | Reduce-only order must oppose position |
| 22   | "Invalid reduce-only order amount: {order\_amount} exceeds position amount {position\_amount}" | Reduce-only size exceeds position      |
| 23   | "No position found for reduce-only order: {address} {symbol}"                                  | No position to reduce                  |

**Transfer & Subaccount Errors**

| Code | Error Message                                            | Description                           |
| ---- | -------------------------------------------------------- | ------------------------------------- |
| 33   | "Invalid transfer relationship: {from} -> {to}"          | Accounts not related for transfers    |
| 34   | "Subaccount withdrawal not allowed: {address}"           | Subaccounts cannot withdraw directly  |
| 35   | "Subaccounts cannot create other subaccounts: {address}" | Subaccounts cannot create subaccounts |

**Rate Limiting**

| Code | HTTP Status | Error Message         | Description                     |
| ---- | ----------- | --------------------- | ------------------------------- |
| 429  | 429         | "Rate limit exceeded" | API request rate limit exceeded |

**Request Timeout**

| Code | HTTP Status | Error Message       | Description                    |
| ---- | ----------- | ------------------- | ------------------------------ |
| 504  | 504         | "Request timed out" | Request exceeded timeout limit |

#### WebSocket Error Codes

WebSocket connections use 401 instead fo 400 for invalid signature:

| Code | Error Message       | Description           |
| ---- | ------------------- | --------------------- |
| 401  | "Invalid signature" | Authentication failed |
