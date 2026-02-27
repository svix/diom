clippy:
    #!/bin/bash
    # TODO: set this
    PACKAGES="$(cargo workspaces list | grep -v '^\(coyote-cli\|coyote-client\)$' | sed -e 's/^/--package=/' | tr '\n' ' ')"
    set -x
    cargo +nightly clippy --fix --allow-dirty --all-features --all-targets $PACKAGES

# Format rust code
fmt:
    cargo +nightly fmt
    cargo sort --workspace -o package,lib,bin,features,dependencies,dev-dependencies,lints,workspace

lint: clippy fmt

# Test the backend
test *args='':
    cargo nextest run {{ args }} # no --workspace, rely on workspace default-members

# Test the SDKs + CLI
test-sdks:
    cargo nextest run --package=coyote-client --package=coyote-cli

# Run code generation for all SDKs + CLI
codegen:
    cargo codegen

# Install dependencies for using this justfile
setup:
    rustup toolchain install nightly
    rustup toolchain install stable
    cargo install --locked cargo-sort cargo-workspaces cargo-nextest

# Run all the test commands
test-all: test test-sdks

[working-directory('benchmarks')]
bench:
    cargo bench
