# people-cloud

> A user-powered hosting mesh where anyone can **pay or get paid** for compute, storage, and bandwidth — with a free tier routed over best‑effort peers.

**Status:** 🚧 Pre‑alpha scaffold (prototype starting point)

---

## Why

Centralized clouds can deplatform, overcharge, or fail single‑handedly. `people-cloud` flips that: your devices become part of a **decentralized hosting mesh**. Earn for uptime and bandwidth; spend when you deploy your own sites or apps.

---

## Architecture (v0)

- **Dashboard (browser)** — Local web UI to see node status and connect to peers.
- **Node (Rust daemon)** — Background service that:
  - Measures available CPU/RAM/disk/bandwidth
  - Exposes a local HTTP API (`/health`, `/resources`, `/peer/connect`)
  - Maintains peer hints (for later P2P via WebRTC/libp2p/Gun.js)
- **Protocols** — Early specs for identity, resource metering, payouts, and QoS tiers.

> v0 focuses on *local-first* status + resource reporting and a simple Gun.js room for discovery.
  Future versions add real P2P replication, content pinning, and verifiable metering.

---

## Quickstart

### 1) Run the Rust node

Requirements: Rust (stable), cargo

```bash
cd node
cargo run
```

The node starts an HTTP server on `http://127.0.0.1:8787` with:
- `GET /health` → `{"ok":true,"uptime_seconds":...}`
- `GET /resources` → system resource snapshot
- `POST /peer/connect` → add a known peer hint

### 2) Open the Dashboard

Just open `dashboard/index.html` in your browser (or serve it):

```bash
# Simple Python static server
cd dashboard
python3 -m http.server 8080
# Now visit http://localhost:8080
```

The Dashboard reads local node APIs and joins a Gun.js "room" for discovery (CDN script tag).

---

## Roadmap (short)

- [ ] v0.1 — Local node + dashboard; manual peer hints; resource snapshots
- [ ] v0.2 — Basic content pinning & replication queue (mock); QoS tiers (free/paid)
- [ ] v0.3 — Reputation scoring; signed heartbeats; basic credits accounting
- [ ] v0.4 — WebRTC/libp2p transport; NAT traversal; bandwidth metering
- [ ] v0.5 — Payments bridge (Stripe/Lightning); earn/spend; configurable policies

See `docs/ROADMAP.md` for details.

---

## License

MIT — see `LICENSE`.