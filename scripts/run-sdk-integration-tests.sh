#!/usr/bin/env bash
set -euo pipefail

# Builds the server, starts it, runs SDK integration tests, then stops it.
#
# Usage: ./run-sdk-integration-tests.sh [options] [rust|python|javascript|go|java|all] ...
#        ./run-sdk-integration-tests.sh              # build + all SDKs
#        ./run-sdk-integration-tests.sh rust python   # build + specific SDKs
#
# Options:
#   --no-build          Skip building the server binary (use existing binary)
#   --binary <path>     Use a specific binary instead of building


SCRIPT_DIR="$(dirname "$0")"
ROOT_DIR="$SCRIPT_DIR/.."

BUILD=true
BINARY="$ROOT_DIR/target/release/diom-server"
SDK_ARGS=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --no-build)
            BUILD=false
            shift
            ;;
        --binary)
            BUILD=false
            BINARY="${2:?--binary requires a path argument}"
            shift 2
            ;;
        rust|python|javascript|go|java|all)
            SDK_ARGS+=("$1")
            shift
            ;;
        *)
            echo "Usage: $0 [--no-build | --binary <path>] [rust|python|javascript|go|java|all] ..."
            exit 1
            ;;
    esac
done

if [[ ${#SDK_ARGS[@]} -eq 0 ]]; then
    SDK_ARGS=(all)
fi

trap '"$SCRIPT_DIR/stop-diom-server.sh"' EXIT

if [[ "$BUILD" == true ]]; then
    echo "==> Building server binary..."
    (cd "$ROOT_DIR" && cargo build --release --package diom-server --bin diom-server)
fi

"$SCRIPT_DIR/start-diom-server.sh" "$BINARY"
"$SCRIPT_DIR/test-sdk.sh" "${SDK_ARGS[@]}"
