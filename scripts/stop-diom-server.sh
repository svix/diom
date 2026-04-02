#!/usr/bin/env bash
set -euo pipefail

# Stops the Diom server started by start-diom-server.sh.

PID_FILE="/tmp/diom-server.pid"

if [[ ! -f "$PID_FILE" ]]; then
    echo "No PID file found at $PID_FILE — nothing to stop."
    exit 0
fi

PID=$(cat "$PID_FILE")
echo "==> Stopping server process: $PID"
kill "$PID" 2>/dev/null || true
rm -f "$PID_FILE"
echo "Server stopped."
