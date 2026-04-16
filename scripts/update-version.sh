#!/usr/bin/env bash
# Propagates the version from .version to every file that tracks it.
set -euo pipefail

REPO_ROOT="$(dirname "$0")/.."
VERSION="$(cat "$REPO_ROOT/.version")"

# Cargo workspace version
sed -i "/\[workspace\.package\]/,/^\[/ s/^version *= *\".*\"/version = \"$VERSION\"/" \
    "$REPO_ROOT/Cargo.toml"

# JavaScript — replace only the first top-level "version" field
sed -i "0,/\"version\": *\"[^\"]*\"/ s//\"version\": \"$VERSION\"/" \
    "$REPO_ROOT/z-clients/javascript/package.json"

# Python
sed -i "s/^version *= *\".*\"/version = \"$VERSION\"/" \
    "$REPO_ROOT/z-clients/python/pyproject.toml"

# Java — replace only the first <version> tag (the project's own)
sed -i "0,/<version>.*<\/version>/ s//<version>$VERSION<\/version>/" \
    "$REPO_ROOT/z-clients/java/pom.xml"

echo "Updated all versions to: $VERSION"

# Verify by running the sync checker
"$(dirname "$0")/check-version-sync.sh"
