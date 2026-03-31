#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

COYOTE_TOKEN="admin_abcdefghijlmnopqrstuvwxyz012345"
CONTAINER_NAME="coyote-test"
IMAGE_NAME="coyote-server"
PORT=8050

# Parse arguments
SDKS=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        rust|python|javascript|go|java|all)
            SDKS+=("$1")
            shift
            ;;
        *)
            echo "Usage: $0 [rust|python|javascript|go|java|all] ..."
            echo "  If no SDK is specified, all SDKs are tested."
            exit 1
            ;;
    esac
done

if [[ ${#SDKS[@]} -eq 0 ]] || [[ " ${SDKS[*]} " == *" all "* ]]; then
    SDKS=(rust python javascript go java)
fi

cleanup() {
    echo "Cleaning up..."
    docker stop "$CONTAINER_NAME" 2>/dev/null || true
    docker rm "$CONTAINER_NAME" 2>/dev/null || true
}
trap cleanup EXIT

echo "==> Building Docker image..."
docker build --target prod -t "$IMAGE_NAME" "$ROOT_DIR"

echo "==> Starting Coyote server..."
docker rm -f "$CONTAINER_NAME" 2>/dev/null || true
docker run -d --name "$CONTAINER_NAME" \
    --network host \
    -e COYOTE_LISTEN_ADDRESS="0.0.0.0:$PORT" \
    -e COYOTE_ENVIRONMENT=dev \
    -e COYOTE_CLUSTER_AUTO_INITIALIZE=true \
    -e "COYOTE_ADMIN_TOKEN=$COYOTE_TOKEN" \
    "$IMAGE_NAME"

echo "==> Waiting for server to be ready..."
for i in $(seq 1 60); do
    if curl -sf "http://localhost:$PORT/api/v1.health.ping" > /dev/null 2>&1; then
        echo "Server is ready"
        break
    fi
    if [[ $i -eq 60 ]]; then
        echo "Server failed to start"
        docker logs "$CONTAINER_NAME"
        exit 1
    fi
    sleep 1
done

FAILED_SDKS=()

for sdk in "${SDKS[@]}"; do
    echo ""
    echo "=========================================="
    echo "==> Running $sdk SDK integration tests"
    echo "=========================================="

    sdk_exit=0

    case "$sdk" in
        rust)
            (cd "$ROOT_DIR" && cargo test --package coyote-client -- --ignored 2>&1) || sdk_exit=$?
            ;;
        python)
            (cd "$ROOT_DIR/z-clients/python" && COYOTE_INTEGRATION=1 uv run --group dev pytest tests/ -v 2>&1) || sdk_exit=$?
            ;;
        javascript)
            (cd "$ROOT_DIR/z-clients/javascript" && npm ci && npm run build && npm run bundle && node --test tests/integration.test.mjs 2>&1) || sdk_exit=$?
            ;;
        go)
            (cd "$ROOT_DIR/z-clients/go" && go test -tags=integration -v -count=1 ./... 2>&1) || sdk_exit=$?
            ;;
        java)
            (cd "$ROOT_DIR/z-clients/java" && ./gradlew integrationTest 2>&1) || sdk_exit=$?
            ;;
    esac

    if [[ $sdk_exit -ne 0 ]]; then
        FAILED_SDKS+=("$sdk")
    fi
done

echo ""
echo "============================================================"
if [[ ${#FAILED_SDKS[@]} -eq 0 ]]; then
    echo "  Result: ALL SDK TESTS PASSED"
else
    echo "  Result: FAILURES in: ${FAILED_SDKS[*]}"
fi
echo "============================================================"

if [[ ${#FAILED_SDKS[@]} -gt 0 ]]; then
    exit 1
fi
