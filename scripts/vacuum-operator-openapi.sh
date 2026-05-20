#!/usr/bin/env sh
set -eu

root=$(cd "$(dirname "$0")/.." && pwd)

jq '{
  "openapi": "3.1.0",
  "info": {"title": "DiomCluster CRD", "description": "DiomCluster CRD schema", "version": "0.0.0"},
  "paths": {},
  "components": {
    "schemas": (.spec.versions | map({(.name): .schema.openAPIV3Schema}) | add // {})
  }
}' "$root/infra/helm-diom/charts/crds/crds/diomclusters.json" |
    docker run --rm -i \
        -v "$root":/work:ro \
        dshanley/vacuum lint \
        /dev/stdin \
        --fail-severity error \
        --ruleset infra/operator/.vacuum.yaml \
        --min-score 0 \
        "$@"
