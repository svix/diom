# syntax=docker/dockerfile:1
#
# NOTE: build from the operator/ directory with `docker build .`

# Using https://github.com/LukeMathWalker/cargo-chef for better layer caching

# Base image for planner and build - keep in sync with root Dockerfile
FROM docker.io/rust:1.97.1-slim-trixie AS chef
RUN cargo install --locked cargo-chef@0.1.77
WORKDIR /app

# Build plan environment
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build environment
FROM chef AS build

ARG __BUST_DOCKER_BUILD_CACHE=2026-06-10
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
RUN cargo chef cook --release -p diom-operator --recipe-path recipe.json

# Build the operator
COPY . .

ARG CARGO_LOG
ARG GITHUB_SHA
ARG RELEASE_VERSION
RUN cargo build --release -p diom-operator --bin diom-operator --frozen

# Production
FROM docker.io/debian:trixie-20260713-slim AS prod

RUN <<EOF
    #!/bin/bash
    set -euxo pipefail
    useradd appuser
    mkdir -p /home/appuser
    chown -R appuser: /home/appuser
EOF

ARG __BUST_DOCKER_BUILD_CACHE=2026-06-10
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

COPY --chown=root:root --chmod=755 --from=build /app/target/release/diom-operator /usr/local/bin/diom-operator

LABEL org.opencontainers.image.authors="support@svix.com" \
      org.opencontainers.image.url="https://diom.svix.com" \
      org.opencontainers.image.documentation="https://diom.svix.com/docs" \
      org.opencontainers.image.description="The Diom backend components platform, kubernetes operator component" \
      org.opencontainers.image.title="diom operator" \
      org.opencontainers.image.vendor="Svix" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.base.name="docker.io/debian:trixie"

ENTRYPOINT ["/usr/local/bin/diom-operator"]
