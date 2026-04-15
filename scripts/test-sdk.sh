#!/usr/bin/env bash
set -euo pipefail

# Runs integration tests for a specific SDK (or all SDKs).
#
# Usage: ./test-sdk.sh <sdk> [<sdk> ...]
#        ./test-sdk.sh all
#
# Supported SDKs: rust, python, javascript, go, java
#
# Environment variables:
#   DIOM_TOKEN       Auth token for the running server (has a default)
#   DIOM_SERVER_URL  Server URL (default: http://localhost:8624)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$SCRIPT_DIR/.."

export DIOM_TOKEN="${DIOM_TOKEN:-admin_abcdefghijlmnopqrstuvwxyz012345}"
export DIOM_SERVER_URL="${DIOM_SERVER_URL:-http://localhost:8624}"

export DIOM_TOKEN="$DIOM_TOKEN"
export DIOM_SERVER_URL="$DIOM_SERVER_URL"

ALL_SDKS=(rust python javascript go java)

run_rust() {
    echo "==> Running Rust SDK integration tests"
    (cd "$ROOT_DIR" && cargo test --package diom -- --ignored)
}

run_python() {
    echo "==> Running Python SDK integration tests"
    (cd "$ROOT_DIR/z-clients/python" && DIOM_INTEGRATION=1 uv run --group dev pytest tests/ -v)
}

run_javascript() {
    echo "==> Running JavaScript SDK integration tests"
    (cd "$ROOT_DIR/z-clients/javascript" && npm ci && npm run build && npm run bundle && node --test tests/integration.test.mjs)
}

run_go() {
    echo "==> Running Go SDK integration tests"
    (cd "$ROOT_DIR/z-clients/go" && go test -tags=integration -v -count=1 ./...)
}

run_java() {
    echo "==> Running Java SDK integration tests"
    (cd "$ROOT_DIR/z-clients/java" && ./gradlew integrationTest)
}

# Parse arguments
SDKS=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        rust | python | javascript | go | java)
            SDKS+=("$1")
            shift
            ;;
        all)
            SDKS=("${ALL_SDKS[@]}")
            shift
            ;;
        *)
            echo "Usage: $0 <rust|python|javascript|go|java|all> ..."
            exit 1
            ;;
    esac
done

if [[ ${#SDKS[@]} -eq 0 ]]; then
    echo "Usage: $0 <rust|python|javascript|go|java|all> ..."
    exit 1
fi

FAILED_SDKS=()

for sdk in "${SDKS[@]}"; do
    echo ""
    echo "=========================================="
    echo "  $sdk"
    echo "=========================================="

    if "run_$sdk"; then
        echo "==> $sdk: PASSED"
    else
        echo "==> $sdk: FAILED"
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
