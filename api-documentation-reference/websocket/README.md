# Websocket

Mainnet websocket URL: [wss://ws.pacifica.fi/ws](wss://ws.pacifica.fi/ws)&#x20;

Testnet websocket URL: [wss://test-ws.pacifica.fi/ws](wss://test-ws.pacifica.fi/ws)

The API service provides a universal endpoint for websocket streams. The subscribed data will be streamed in the corresponding channel after the connection is established.

#### Subscription Message <a href="#subscription-message" id="subscription-message"></a>

```
{
    "method": "subscribe",
    "params": { ... }
}
```

#### Unsubscription Message <a href="#unsubscription-message" id="unsubscription-message"></a>

```
{
    "method": "unsubscribe",
    "params": { ... }
}
```

#### Heartbeat and Timeout <a href="#heartbeat-and-timeout" id="heartbeat-and-timeout"></a>

A webscoket connection will be closed if no message is sent for the past 60 seconds, or the connection has been alive for 24 hours.

To keep the connection alive without messages in 60 seconds, we can send a heartbeat message

```
{
    "method": "ping"
}
```

and alive connection will respond with

```
{
    "channel": "pong"
}
```
