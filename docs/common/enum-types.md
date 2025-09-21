# Enum Types

### Payload Use:

```rust
AggLevel {
    L1 = 1,
    L2 = 2,
    L5 = 5,
    L10 = 10,
    L100 = 100,
    L1000 = 1000,
}
```

```rust
Interval {
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    TwoHours,
    FourHours,
    EightHours,
    TwelveHours,
    OneDay,
}
```

```rust
EquityHistoryInterval {
    OneDay,
    SevenDays,
    Month, // 30d
    All,
}
```

```rust
OrderSide {
    Bid,
    Ask,
}
```

```rust
Tif {
    GTC,
    IOC,
    ALO,
}
```

```rust
WsMethod {
    Subscribe,
    Unsubscribe,
}
```

### Responses Use:

```rust
OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    TakeProfitLimit,
    StopLossLimit,
    TakeProfitMarket,
    StopLossMarket,
}
```

```rust
OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}
```

```rust
OrderReason {
    Cancelled,
    Rejected,
    Cancel,
    ForceCancel,
    Expired,
    PostOnlyRejected,
    SelfTradePrevented,
}

```

```rust
OrderEventType {
    Make,
    Take,
    FulfillMaker,
    FulfillTaker,
    FulfillMarket,
    FulfillLimit,
    Adjust,
    StopCreated,
    StopParentOrderFilled,
    StopTriggered,
    StopUpgrade,
    Cancel,
    ForceCancel,
    Expired,
    PostOnlyRejected,
    SelfTradePrevented,
}
```

```rust
TradeSide {
    OpenLong,
    OpenShort,
    CloseLong,
    CloseShort,
}
```

```rust
TradeCause {
    Normal,
    MarketLiquidation,
    BackstopLiquidation,
    Settlement,
}
```

#### Account

```rust
AccountEventType {
    Deposit,
    DepositRelease,
    Withdraw,
    Trade,
    MarketLiquidation,
    BackstopLiquidation,
    AdlLiquidation,
    SubaccountTransfer,
    Funding,
    Payout,
}
```
