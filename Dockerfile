# syntax=docker/dockerfile:1
#
# NOTE: this should be built from the top level directory of the reop with with `docker build .`

# Using https://github.com/LukeMathWalker/cargo-chef for better layer caching

# Base image for planner and build - keep in sync with .github/workflows/server-ci.yml
FROM docker.io/rust:1.93-slim-trixie AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Build plan environment
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build environment
FROM chef AS build

ENV __BUST_DOCKER_BUILD_CACHE=2026-01-30
RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked --mount=target=/var/cache/apt,type=cache,sharing=locked <<EOF
    #!/bin/bash
    set -euxo pipefail
    export DEBIAN_FRONTEND=noninteractive
    apt-get update -q
    apt-get install -y \
        mold \
        --no-install-recommends
EOF

# Set up mold as our linker
RUN <<EOF
    mkdir -p .cargo
    echo "" >>.cargo/config.toml
    echo "[target.'cfg(target_os = \"linux\"']" >>.cargo/config.toml
    echo 'rustflags = ["-C", "link-arg=-fuse-ld=mold"]' >>.cargo/config.toml
    cat .cargo/config.toml
EOF

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer
RUN cargo chef cook --release --package coyote-server --features coyote/openapi --recipe-path recipe.json

# Build the server
COPY . .

ARG CARGO_LOG
ARG GITHUB_SHA
ARG RELEASE_VERSION
RUN cargo build --release --package coyote-server --bin coyote-server --features coyote/openapi --frozen

# Production
FROM docker.io/debian:trixie-slim AS prod

RUN <<EOF
    #!/bin/bash
    set -euxo pipefail
    mkdir -p /app
    useradd appuser
    chown -R appuser: /app
    mkdir -p /home/appuser
    chown -R appuser: /home/appuser
    mkdir -p /storage
    chown -R appuser: /storage
EOF

ENV __BUST_DOCKER_BUILD_CACHE=2026-01-30
RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked --mount=target=/var/cache/apt,type=cache,sharing=locked <<EOF
    #!/bin/bash
    set -euxo pipefail
    export DEBIAN_FRONTEND=noninteractive
    apt-get update -q
    apt-get install -y \
        ca-certificates=20250419 \
        --no-install-recommends
    update-ca-certificates
EOF

USER appuser
WORKDIR /home/appuser
EXPOSE 8050

COPY --chown=root:root --chmod=755 --from=build /app/target/release/coyote-server /usr/local/bin/coyote-server

CMD ["/usr/local/bin/coyote-server"]
