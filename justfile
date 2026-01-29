lint:
    cargo +nightly fmt
    cargo sort --workspace -o package,lib,bin,features,dependencies,dev-dependencies,lints,workspace
    cargo +nightly clippy --workspace --fix --allow-dirty --all-features --all-targets

# Test the backend
test *args='':
    cargo nextest run {{ args }} # no --workspace, rely on workspace default-members

# Test the SDKs + CLI
test-sdks:
    cargo nextest run --package=coyote-client --package=coyote-cli

# Run all the test commands
test-all: test test-sdks
