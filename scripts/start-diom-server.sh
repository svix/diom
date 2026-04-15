#!/usr/bin/env bash
set -euo pipefail

# Starts a Diom server from a binary for integration testing.
#
# Usage: ./start-diom-server.sh [<binary-path>]
#
# Arguments:
#   binary-path   Path to the diom-server binary (default: target/release/diom-server)
#
# Environment variables:
#   DIOM_ADMIN_TOKEN  Admin token (default: admin_abcdefghijlmnopqrstuvwxyz012345)
#   DIOM_PORT         Port to listen on (default: 8624)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$SCRIPT_DIR/.."

BINARY="${1:-$ROOT_DIR/target/release/diom-server}"
DIOM_ADMIN_TOKEN="${DIOM_ADMIN_TOKEN:-admin_abcdefghijlmnopqrstuvwxyz012345}"
PORT="${DIOM_PORT:-8624}"
PID_FILE="/tmp/diom-server.pid"

if [[ ! -f "$BINARY" ]]; then
    echo "Error: binary not found at $BINARY"
    echo "Build it with: cargo build --release --package diom-server"
    exit 1
fi

chmod +x "$BINARY"

echo "==> Starting Diom server ($BINARY) on port $PORT..."

DIOM_LISTEN_ADDRESS="0.0.0.0:$PORT" \
    DIOM_ENVIRONMENT=dev \
    DIOM_CLUSTER_AUTO_INITIALIZE=true \
    DIOM_ADMIN_TOKEN="$DIOM_ADMIN_TOKEN" \
    "$BINARY" &

echo $! >"$PID_FILE"

echo "==> Waiting for server to be ready..."
for i in $(seq 1 60); do
    if curl -sf "http://localhost:$PORT/api/v1.health.ping" >/dev/null 2>&1; then
        echo "Server is ready (pid $(cat "$PID_FILE"))"
        exit 0
    fi
done

echo "Server failed to start"
