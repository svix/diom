# syntax=docker/dockerfile:1
#
# NOTE: this should be built from the top level directory of the reop with with `docker build .`

# Using https://github.com/LukeMathWalker/cargo-chef for better layer caching

# Base image for planner and build - keep in sync with .github/workflows/server-ci.yml
FROM docker.io/rust:1.96.1-slim-trixie AS chef
RUN cargo install --locked cargo-chef@0.1.77
RUN cargo install --locked cargo-sbom@0.10.0
WORKDIR /app

# Build plan environment
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build environment
FROM chef AS build-base

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

FROM build-base AS build-server

RUN cargo chef cook --release --package diom-server --features diom-backend/openapi --recipe-path recipe.json

# Build the server
COPY . .

ARG CARGO_LOG
ARG GITHUB_SHA
ARG RELEASE_VERSION
RUN cargo build --release --package diom-server --bin diom-server --features diom-backend/openapi --frozen
RUN cargo sbom --cargo-package diom-server > /app/diom-server.spdx

FROM build-base AS build-cli

# Build dependencies - this is the caching Docker layer
RUN cargo chef cook --release --package diom-cli --recipe-path recipe.json

# Build the CLI
COPY . .

ARG CARGO_LOG
ARG GITHUB_SHA
ARG RELEASE_VERSION
RUN cargo build --release --package diom-cli --bin diom --frozen
RUN cargo sbom --cargo-package diom-cli > /app/diom-cli.spdx

# shared base image with dependencies
FROM docker.io/debian:trixie-20260713-slim AS base

ARG __BUST_DOCKER_BUILD_CACHE=2026-07-13
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

RUN <<EOF
    #!/bin/bash
    set -euxo pipefail
    mkdir -p /app
    useradd appuser
    chown -R appuser: /app
    mkdir -p /home/appuser
    chown -R appuser: /home/appuser
    mkdir -p /usr/local/share
EOF

# CLI Production
FROM base AS cli-prod

USER appuser
WORKDIR /home/appuser

COPY --chown=root:root --chmod=755 --from=build-cli /app/target/release/diom /usr/local/bin/diom
COPY --chown=root:root --chmod=644 --from=build-cli /app/diom-cli.spdx /diom-cli.spdx


LABEL org.opencontainers.image.authors="support@svix.com" \
      org.opencontainers.image.url="https://diom.svix.com" \
      org.opencontainers.image.documentation="https://diom.svix.com/docs" \
      org.opencontainers.image.description="The Diom backend components platform, CLI component" \
      org.opencontainers.image.title="diom-cli" \
      org.opencontainers.image.vendor="Svix" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.base.name="docker.io/debian:trixie"

ENTRYPOINT ["/usr/local/bin/diom"]

# Production
FROM cli-prod AS prod

USER root

RUN <<EOF
    #!/bin/bash
    mkdir -p /storage
    chown -R appuser: /storage
EOF

ENV DIOM_PERSISTENT_DB_PATH="/storage/db"
# Should point to ephemeral storage in production
ENV DIOM_EPHEMERAL_DB_PATH="/storage/db-ephemeral"
ENV DIOM_CLUSTER_LOG_PATH="/storage/logs"
ENV DIOM_CLUSTER_SNAPSHOT_PATH="/storage/snapshots"

USER appuser
WORKDIR /home/appuser
EXPOSE 8624/tcp
EXPOSE 8625/tcp

COPY --chown=root:root --chmod=755 --from=build-server /app/target/release/diom-server /usr/local/bin/diom-server
COPY --chown=root:root --chmod=644 --from=build-server /app/diom-server.spdx /diom-server.spdx

LABEL org.opencontainers.image.authors="support@svix.com" \
      org.opencontainers.image.url="https://diom.svix.com" \
      org.opencontainers.image.documentation="https://diom.svix.com/docs" \
      org.opencontainers.image.description="The Diom backend components platform, server component" \
      org.opencontainers.image.title="diom-server" \
      org.opencontainers.image.vendor="Svix" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.base.name="docker.io/debian:trixie"

ENTRYPOINT ["/usr/local/bin/diom-server"]
