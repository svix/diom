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

RESULTS_DIR=$(mktemp -d)
cleanup() {
    echo "Cleaning up..."
    docker stop "$CONTAINER_NAME" 2>/dev/null || true
    docker rm "$CONTAINER_NAME" 2>/dev/null || true
    rm -rf "$RESULTS_DIR"
}
trap cleanup EXIT

# --- Parse functions per SDK ---

parse_rust() {
    local output="$1"
    # Rust output: "test test_name ... ok" or "test test_name ... FAILED"
    while IFS= read -r line; do
        if [[ "$line" =~ ^test\ (.+)\ \.\.\.\ (.+)$ ]]; then
            local name="${BASH_REMATCH[1]}"
            local result="${BASH_REMATCH[2]}"
            if [[ "$result" == "ok" ]]; then
                echo "PASS $name"
            else
                echo "FAIL $name"
            fi
        fi
    done <<< "$output"
}

parse_python() {
    local output="$1"
    # pytest -v output: "tests/test_integration.py::test_name PASSED" or "FAILED"
    while IFS= read -r line; do
        if [[ "$line" =~ ::([a-zA-Z_0-9]+)\ (PASSED|FAILED) ]]; then
            local name="${BASH_REMATCH[1]}"
            local result="${BASH_REMATCH[2]}"
            if [[ "$result" == "PASSED" ]]; then
                echo "PASS $name"
            else
                echo "FAIL $name"
            fi
        fi
    done <<< "$output"
}

parse_javascript() {
    local output="$1"
    # Node test output: "✔ test_name (duration)" or "✖ test_name (duration)"
    while IFS= read -r line; do
        if [[ "$line" =~ ^\ +[✔✓]\ (.+)\ \( ]]; then
            echo "PASS ${BASH_REMATCH[1]}"
        elif [[ "$line" =~ ^\ +[✖✗]\ (.+)\ \( ]]; then
            echo "FAIL ${BASH_REMATCH[1]}"
        fi
    done <<< "$output"
}

parse_go() {
    local output="$1"
    # Go output: "--- PASS: TestName (duration)" or "--- FAIL: TestName (duration)"
    while IFS= read -r line; do
        if [[ "$line" =~ ^---\ (PASS|FAIL):\ ([a-zA-Z_0-9]+) ]]; then
            local result="${BASH_REMATCH[1]}"
            local name="${BASH_REMATCH[2]}"
            if [[ "$result" == "PASS" ]]; then
                echo "PASS $name"
            else
                echo "FAIL $name"
            fi
        fi
    done <<< "$output"
}

parse_java() {
    local output="$1"
    # Gradle output for JUnit: look for test result lines
    # "IntegrationTest > testName() PASSED" or "FAILED"
    while IFS= read -r line; do
        if [[ "$line" =~ ([a-zA-Z_0-9]+)\(\)\ (PASSED|FAILED) ]]; then
            local name="${BASH_REMATCH[1]}"
            local result="${BASH_REMATCH[2]}"
            if [[ "$result" == "PASSED" ]]; then
                echo "PASS $name"
            else
                echo "FAIL $name"
            fi
        fi
    done <<< "$output"
}

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

    output_file="$RESULTS_DIR/${sdk}.out"
    sdk_exit=0

    case "$sdk" in
        rust)
            (cd "$ROOT_DIR" && cargo test --package coyote-client -- --ignored 2>&1) | tee "$output_file" || sdk_exit=$?
            ;;
        python)
            (cd "$ROOT_DIR/z-clients/python" && COYOTE_INTEGRATION=1 uv run --group dev pytest tests/ -v 2>&1) | tee "$output_file" || sdk_exit=$?
            ;;
        javascript)
            (cd "$ROOT_DIR/z-clients/javascript" && npm ci && npm run build && npm run bundle && node --test tests/integration.test.mjs 2>&1) | tee "$output_file" || sdk_exit=$?
            ;;
        go)
            (cd "$ROOT_DIR/z-clients/go" && go test -tags=integration -v -count=1 ./... 2>&1) | tee "$output_file" || sdk_exit=$?
            ;;
        java)
            (cd "$ROOT_DIR/z-clients/java" && ./gradlew integrationTest 2>&1) | tee "$output_file" || sdk_exit=$?
            ;;
    esac

    # Parse results
    output=$(<"$output_file")
    results=$(parse_"$sdk" "$output")

    if [[ -z "$results" ]]; then
        if [[ $sdk_exit -ne 0 ]]; then
            echo "ERROR" > "$RESULTS_DIR/${sdk}.results"
            FAILED_SDKS+=("$sdk")
        else
            echo "NO_TESTS" > "$RESULTS_DIR/${sdk}.results"
        fi
    else
        echo "$results" > "$RESULTS_DIR/${sdk}.results"
        if echo "$results" | grep -q "^FAIL "; then
            FAILED_SDKS+=("$sdk")
        fi
    fi
done

# --- Print summary ---
echo ""
echo ""
echo "============================================================"
echo "                    TEST SUMMARY"
echo "============================================================"
echo ""

any_failure=false

for sdk in "${SDKS[@]}"; do
    results_file="$RESULTS_DIR/${sdk}.results"
    if [[ ! -f "$results_file" ]]; then
        continue
    fi

    content=$(<"$results_file")

    if [[ "$content" == "ERROR" ]]; then
        echo "  $sdk: ERROR (build or setup failed, no tests ran)"
        any_failure=true
        echo ""
        continue
    fi

    if [[ "$content" == "NO_TESTS" ]]; then
        echo "  $sdk: OK (no individual test results parsed)"
        echo ""
        continue
    fi

    pass_count=$(echo "$content" | grep -c "^PASS " || true)
    fail_count=$(echo "$content" | grep -c "^FAIL " || true)
    total=$((pass_count + fail_count))

    if [[ $fail_count -eq 0 ]]; then
        echo "  $sdk: ALL PASSED ($pass_count/$total)"
    else
        echo "  $sdk: $pass_count/$total passed, $fail_count FAILED"
        any_failure=true
        echo "$content" | grep "^FAIL " | while read -r _ name; do
            echo "    - FAIL: $name"
        done
    fi

    echo "$content" | grep "^PASS " | while read -r _ name; do
        echo "    - PASS: $name"
    done

    echo ""
done

echo "============================================================"
if [[ ${#FAILED_SDKS[@]} -eq 0 ]]; then
    echo "  Result: ALL SDK TESTS PASSED"
else
    echo "  Result: FAILURES in: ${FAILED_SDKS[*]}"
fi
echo "============================================================"

if [[ "$any_failure" == true ]]; then
    exit 1
fi
