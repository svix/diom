#!/usr/bin/env bash
set -euo pipefail

# Starts a Coyote server from a binary for integration testing.
#
# Usage: ./start-coyote-server.sh [<binary-path>]
#
# Arguments:
#   binary-path   Path to the coyote-server binary (default: target/release/coyote-server)
#
# Environment variables:
#   COYOTE_ADMIN_TOKEN  Admin token (default: admin_abcdefghijlmnopqrstuvwxyz012345)
#   COYOTE_PORT         Port to listen on (default: 8050)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$SCRIPT_DIR/.."

BINARY="${1:-$ROOT_DIR/target/release/coyote-server}"
COYOTE_ADMIN_TOKEN="${COYOTE_ADMIN_TOKEN:-admin_abcdefghijlmnopqrstuvwxyz012345}"
PORT="${COYOTE_PORT:-8050}"
PID_FILE="/tmp/coyote-server.pid"

if [[ ! -f "$BINARY" ]]; then
    echo "Error: binary not found at $BINARY"
    echo "Build it with: cargo build --release --package coyote-server"
    exit 1
fi

chmod +x "$BINARY"

echo "==> Starting Coyote server ($BINARY) on port $PORT..."

COYOTE_LISTEN_ADDRESS="0.0.0.0:$PORT" \
COYOTE_ENVIRONMENT=dev \
COYOTE_CLUSTER_AUTO_INITIALIZE=true \
COYOTE_ADMIN_TOKEN="$COYOTE_ADMIN_TOKEN" \
    "$BINARY" &

echo $! > "$PID_FILE"

echo "==> Waiting for server to be ready..."
for i in $(seq 1 60); do
    if curl -sf "http://localhost:$PORT/api/v1.health.ping" > /dev/null 2>&1; then
        echo "Server is ready (pid $(cat "$PID_FILE"))"
        exit 0
    fi
done

echo "Server failed to start"
