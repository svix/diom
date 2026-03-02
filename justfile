set quiet := true

HERE := justfile_directory()

lint:
    cargo +nightly clippy --workspace --fix --allow-dirty --all-features --all-targets
    cargo +nightly fmt
    cargo sort --workspace -o package,lib,bin,features,dependencies,dev-dependencies,lints,workspace

# Test the backend
test *args='':
    cargo nextest run {{ args }} # no --workspace, rely on workspace default-members

# Test the SDKs + CLI
test-sdks:
    cargo nextest run --package=diom-client --package=diom-cli

[no-exit-message]
test-dc *args='':
    docker compose -f {{ HERE / "testing-docker-compose.yml" }} {{ args }}

# Rebuild a service in `docker compose`, leaving its volumes intact
[no-exit-message]
test-dc-rebuild service:
    docker compose -f {{ HERE / "testing-docker-compose.yml" }} rm -fs {{ service }}
    docker compose -f {{ HERE / "testing-docker-compose.yml" }} up --build -d {{ service }}

codegen:
    cargo codegen

# Run all the test commands
test-all: test test-sdks

[working-directory('benchmarks')]
bench:
    cargo bench
