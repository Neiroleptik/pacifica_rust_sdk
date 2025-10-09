# Changelog

## [Unreleased]
- Optimize WebSocket client architecture.

## [2.0.0] - 2025-10-09
### Removed
- Removed the `event_type` ("e") field from the WebSocket Trades subscription response.  
  Only `fullfill_taker` events are now streamed.  
  `fullfill_maker` events have been removed, since in every match the traded size of the maker and taker sides is exactly equal.

## [1.0.0] - 2025-10-08
### Removed
- Removed the `counter_party` field in `TradeHistoryModel` (`info/responses/account.rs`) and `TradeModel` (`ws/responses.rs`).