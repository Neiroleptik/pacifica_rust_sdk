# Account

* Get account info: This endpoint allows users to get all high-level account info such as balance, fee level, equity, etc.
* Get account settings: This endpoint allows users to get account margin and leverage settings (if they are not at default values)
* Update leverage: This endpoint allows users to change their account leverage for a specific trading pair. For open positions, users can only increase the leverage setting.
* Update margin mode: This endpoint allows users to switch between isolated and cross margin modes for a specific trading pair. For open positions, users cannot change the margin mode.
* Get positions: This endpoint allows users to get current positions.
* Get trade history
* Get funding history: This endpoint allows users to get funding history.
* Get account equity history: This endpoint allows users to get account equity history (account equity over time).
* Get account balance history: This endpoint allows users to get account balance history. Returns all balance effects that affects the queried account.
* Request withdrawal: This endpoint allows users to request withdrawal.



Binary for GET methods [Rust SDK examples](../../../src/bin/info.rs)
