#!/usr/bin/env bash
set -euo pipefail

BINARY="${1:-/tmp/coyote-server}"

chmod +x "$BINARY"

COYOTE_LISTEN_ADDRESS=0.0.0.0:8050 \
COYOTE_ENVIRONMENT=dev \
COYOTE_CLUSTER_AUTO_INITIALIZE=true \
COYOTE_ADMIN_TOKEN="${COYOTE_ADMIN_TOKEN:?}" \
  "$BINARY" &

for _i in $(seq 1 60); do
  if curl -sf http://localhost:8050/api/v1.health.ping > /dev/null 2>&1; then
    echo "Server is ready"
    exit 0
  fi
  sleep 1
done

echo "Server failed to start"
exit 1
