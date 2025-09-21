# WS



```rust
// WebSocket structs doesn't implement validation fn,
WebSocketParams<P> {
    source: String,
    params: P,
}

WebSocketRequest<P> {
    id: Uuid,
    // Where String is action_type/method
    params: HashMap<String, FinalRequest<P>>,
}

DefaultWebSocketMsg<R> {
    channel: String,
    data: R,
}

WebSocketOperationResponse<R> {
    code: u16,
    data: Option<R>,
    err: Option<String>,
    id: Option<Uuid>,
    timestamp: u64, // t
    type_field: Option<String>, // type
}


WebSocketSubscription<P> {
    method: WsMethod,
    params: WebSocketParams<P>,
}

type WebSocketUnsubscription<P> = WebSocketSubscription<P>;
```
