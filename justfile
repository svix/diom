set quiet := true

HERE := justfile_directory()

# run all lints
[group('lint')]
lint: clippy machete fmt sort vacuum-openapi audit

# run clippy in --fix mode
[group('lint')]
clippy:
    # keep this beta to keep it in sync with CI
    cargo +beta clippy --workspace --fix --allow-dirty --all-features --all-targets

# run cargo-machete in --fix mode
[group('lint')]
machete:
    cargo machete --fix

# run cargo-fmt in --fix mode
[group('lint')]
fmt:
    # this has to be nightly to get import sorting working correctly
    cargo +nightly fmt

# run cargo sort
[group('lint')]
sort:
    cargo sort --no-format --workspace -o package,lib,bin,features,dependencies,dev-dependencies,lints,workspace

# run `vacuum` to check openapi
[group('lint')]
vacuum-openapi:
    # keep this in sync with lint-openapi.yml
    docker run --rm \
            -v .:/work:ro \
            dshanley/vacuum lint \
            openapi.json \
            --fail-severity error \
            --ruleset .vacuum.yaml \
            --min-score 0

# run security lints
[group('lint')]
audit:
    cargo deny check -c {{ HERE / "deny.toml" }}

# Test the backend
test *args='':
    cargo nextest run {{ args }} # no --workspace, rely on workspace default-members

# Test the SDKs + CLI
test-sdks:
    cargo nextest run --package=diom-client --package=diom-cli

# Invoke `docker-compose`
[no-exit-message]
test-dc *args='':
    docker compose -f {{ HERE / "testing-docker-compose.yml" }} {{ args }}

# Rebuild a service in `docker compose`, leaving its volumes intact
[no-exit-message]
test-dc-rebuild service:
    docker compose -f {{ HERE / "testing-docker-compose.yml" }} rm -fs {{ service }}
    docker compose -f {{ HERE / "testing-docker-compose.yml" }} up --build -d {{ service }}

# Generate all of the client libraries
codegen: default-config
    cargo codegen

# Dump out config.defaults.toml and ENVIRONMENT_VARIABLES.md
default-config:
    env -u DIOM_ADMIN_TOKEN cargo run -- write-config config.defaults.toml
    cargo run -- describe-environment-variables ENVIRONMENT_VARIABLES.md

# Run all the test commands
test-all: test test-sdks

# Run benchmarks
[working-directory('benchmarks')]
bench:
    cargo bench

# Regenerate the CRD JSON and write it into the Helm chart
[working-directory('infra/operator')]
generate-crd:
    cargo run -- --print-crd-json > {{ HERE / "infra/helm-diom/charts/crds/crds/diomclusters.json" }}
