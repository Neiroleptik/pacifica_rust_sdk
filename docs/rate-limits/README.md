# Rate Limits

Pacifica implements IP and API-Key based rate limiting.

**For REST API:**

1. every IP address begins with 100 credits
2. any http method used will decrement the credit by one (for now all requests have the same weighting)
3. if user makes request while credit < 0, then return HTTP429
4. credits reset back to 100 at the end of each 60 second interval

**For Websocket API:**

1. every IP address is limited to 100 concurrent WS connections.
2. If an IP tries to exceed 100 connections, returns HTTP 429
3. each connection is limited to 20 subscriptions per channel

**Pacifica API Key:**

For Websocket API, Pacifica also provides additional rate-limiting options via API-keys.

**Bytes Size:**

Rest / Websocket request's bytes size is limited to 4kb: enough for \~10 operations with batch order.
