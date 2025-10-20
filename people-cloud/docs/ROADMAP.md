# Roadmap

### v0.1 — Local-first prototype
- Rust node with `/health` and `/resources`
- Dashboard that queries local node
- Gun.js room for presence/discovery (no data relay yet)
- Manual peer-hint management

### v0.2 — Content basis
- Local content registry and pin queue
- Mock replication (simulated copies, local-only)
- QoS policy draft (free/paid tiers)

### v0.3 — Trust & credits
- Signed heartbeats + replay protection
- Reputation score (uptime, latency, fulfilled pins)
- Local credits ledger (earn/spend mock)

### v0.4 — Real transport
- Add WebRTC/libp2p for peer data paths
- NAT traversal; TURN/relay fallbacks
- Bandwidth/CPU metering

### v0.5 — Payments
- Stripe/Lightning bridge for credits
- Payout policies + minimum thresholds
- Compliance and audit trail stubs

### v1.0 — MVP mesh
- Multi-region peers
- Pinning, replication, verification live
- Public “free tier” rails + premium lanes