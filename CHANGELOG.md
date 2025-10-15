# Changelog

## [Unreleased]
- Optimized WebSocket client architecture.

## [3.0.0] - 2025-10-15
### Changed
- Renamed subscription methods in `enum models/ws/subscriptions.rs` to match updated documentation.
  The issue was that older method names were still referenced in the docs, causing confusion.
- Changed the type of the `margin` field in the `PositionModel`  
  from `Option<Decimal>` to `Decimal`.

### Removed
- Removed the `Balances` subscription and all related components.

### Added
- Added the `maintenance margin required in cm` field (`cm` → `cross_mmr`)  
  to the both account info responses (REST, WS).

## [2.0.0] - 2025-10-09
### Removed
- Removed the `event_type` ("e") field from the WebSocket Trades subscription response.  
  Only `fullfill_taker` events are now streamed.  
  `fullfill_maker` events have been removed, since in every match the traded size of the maker and taker sides is exactly equal.

## [1.0.0] - 2025-10-08
### Removed
- Removed the `counter_party` field in `TradeHistoryModel` (`info/responses/account.rs`) and `TradeModel` (`ws/responses.rs`).