#!/usr/bin/env bash
set -euo pipefail
# Simple dev helper to run node and a static file server for the dashboard.
# Requires: cargo, python3

# Run Rust node
( cd node && cargo run ) &
NODE_PID=$!

# Serve dashboard
( cd dashboard && python3 -m http.server 8080 ) &
HTTP_PID=$!

cleanup() {
  echo "Shutting down..."
  kill $NODE_PID || true
  kill $HTTP_PID || true
}
trap cleanup EXIT

wait