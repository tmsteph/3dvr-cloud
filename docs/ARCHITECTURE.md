# Architecture (draft)

## Components
- **Dashboard (browser)**: lightweight control plane.
- **Node (daemon)**: resource monitor + local API + peer book.
- **Discovery**: Gun.js-based room (move to libp2p later).
- **Protocols**: JSON over HTTP for local control; signed messages for peer traffic later.

## Data Flows
- Dashboard → Node: read `/resources`, write `/peer/connect`
- Node ↔ Discovery: presence heartbeat with public key and port hints
- Future: Node ↔ Node: chunk replication, verification, accounting

## Security & Privacy
- Local-only control API by default (bind 127.0.0.1)
- Public keys identify nodes (no PII by default)
- Signed envelopes for peer messages (future)