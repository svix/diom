#!/usr/bin/env bash
# Checks that all version strings across the repo match the canonical .version file.
set -euo pipefail

REPO_ROOT="$(dirname "$0")/.."
EXPECTED="$(cat "$REPO_ROOT/.version")"

errors=0

check() {
    local file="$1"
    local actual="$2"
    if [ "$actual" != "$EXPECTED" ]; then
        echo "MISMATCH $file: got '$actual', expected '$EXPECTED'"
        errors=$((errors + 1))
    fi
}

# Cargo workspace version
cargo_version=$(sed -n '/\[workspace\.package\]/,/^\[/{ s/^version *= *"\(.*\)"/\1/p; }' "$REPO_ROOT/Cargo.toml")
check "Cargo.toml [workspace.package] version" "$cargo_version"

# JavaScript
js_version=$(sed -n 's/.*"version": *"\(.*\)".*/\1/p' "$REPO_ROOT/z-clients/javascript/package.json")
check "z-clients/javascript/package.json" "$js_version"

# Python
py_version=$(sed -n 's/^version *= *"\(.*\)"/\1/p' "$REPO_ROOT/z-clients/python/pyproject.toml")
check "z-clients/python/pyproject.toml" "$py_version"

# Java
java_version=$(sed -n 's/.*<version>\(.*\)<\/version>.*/\1/p' "$REPO_ROOT/z-clients/java/pom.xml" | head -1)
check "z-clients/java/pom.xml" "$java_version"

# Helm chart version
helm_version=$(sed -n 's/^version: *\(.*\)/\1/p' "$REPO_ROOT/infra/helm-diom/Chart.yaml")
check "infra/helm-diom/Chart.yaml version" "$helm_version"

# Helm appVersion
helm_app_version=$(sed -n 's/^appVersion: *"\{0,1\}\([^"]*\)"\{0,1\}/\1/p' "$REPO_ROOT/infra/helm-diom/Chart.yaml")
check "infra/helm-diom/Chart.yaml appVersion" "$helm_app_version"

if [ "$errors" -gt 0 ]; then
    echo ""
    echo "ERROR: $errors version mismatch(es) found. All versions must match .version ($EXPECTED)."
    exit 1
fi

echo "All versions match: $EXPECTED"
