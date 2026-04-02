#!/usr/bin/env bash
set -euo pipefail

# Stops the Coyote server started by start-coyote-server.sh.

PID_FILE="/tmp/coyote-server.pid"

if [[ ! -f "$PID_FILE" ]]; then
    echo "No PID file found at $PID_FILE — nothing to stop."
    exit 0
fi

PID=$(cat "$PID_FILE")
echo "==> Stopping server process: $PID"
kill "$PID" 2>/dev/null || true
rm -f "$PID_FILE"
echo "Server stopped."
