# Protocols (early draft)

## Identity
- Ed25519 keypair per node
- Public key advertised in discovery room

## Heartbeat
```json
{
  "type": "heartbeat",
  "pubkey": "<base64>",
  "uptime_s": 123,
  "timestamp": 1680000000,
  "resources": {
    "cpu_load": 0.24,
    "mem_free_mb": 1024,
    "disk_free_gb": 120.5,
    "net_up_kbps": 5000,
    "net_down_kbps": 20000
  }
}
```
Signed body: `sign(privateKey, sha256(body))`

## Resource Snapshot (Local API)
`GET /resources` → snapshot (not signed; local only)

## Peer Hint
`POST /peer/connect` with JSON:
```json
{ "addr": "wss://example.org/peer", "pubkey": "<base64-optional>" }
```

## QoS Tiers (draft)
- **free**: opportunistic, best effort
- **standard**: verified uptime & redundancy
- **premium**: low-latency, multi-region, SLA-like targets