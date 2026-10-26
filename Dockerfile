# syntax=docker/dockerfile:1
#
# NOTE: this should be built from the top level directory of the reop with with `docker build .`

# Using https://github.com/LukeMathWalker/cargo-chef for better layer caching

# Base image for planner and build - keep in sync with .github/workflows/server-ci.yml
FROM cgr.dev/chainguard/wolfi-base@sha256:0feab531471d2edfcca55f18883441cb7df598e724fa2dedf9985273f532eed7 AS chef
SHELL ["/bin/sh", "-eux", "-c"]
RUN --mount=target=/var/cache/apk,type=cache,sharing=locked --mount=target=/tmp,type=cache,sharing=locked <<EOF
    apk add rust~1.98.1 \
        build-base~1 \
        openssl-dev~4 \
        zlib-dev~1.3 \
        cmake~4 \
        shadow~4 \
        git~2
EOF

SHELL ["/bin/sh", "-eux", "-c"]
RUN <<EOF
    cargo install --locked cargo-chef@0.1.77
    cargo install --locked cargo-sbom@0.10.0
EOF

WORKDIR /app

# Build plan environment
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build environment
FROM chef AS build-base

SHELL ["/bin/sh", "-eux", "-c"]

COPY --from=planner /app/recipe.json recipe.json

FROM build-base AS build-server

RUN cargo chef cook --release --package diom-server --features diom-backend/openapi --recipe-path recipe.json

# Build the server
COPY . .

ARG CARGO_LOG
ARG GITHUB_SHA
ARG RELEASE_VERSION
ARG WRITE_EXECUTABLE_METADATA="1"
RUN <<EOF
    cargo build --release --package diom-server --bin diom-server --features diom-backend/openapi --frozen
    cargo sbom --cargo-package diom-server > /app/diom-server.spdx
EOF

FROM build-base AS build-cli

# Build dependencies - this is the caching Docker layer
RUN cargo chef cook --release --package diom-cli --recipe-path recipe.json

# Build the CLI
COPY . .

ARG CARGO_LOG
ARG GITHUB_SHA
ARG RELEASE_VERSION
ARG WRITE_EXECUTABLE_METADATA="1"
RUN <<EOF
    cargo build --release --package diom-cli --bin diom --frozen
    cargo sbom --cargo-package diom-cli > /app/diom-cli.spdx
EOF

# shared base image with dependencies
FROM cgr.dev/chainguard/wolfi-base@sha256:0feab531471d2edfcca55f18883441cb7df598e724fa2dedf9985273f532eed7 AS base
SHELL ["/bin/sh", "-eux", "-c"]

RUN --mount=target=/var/cache/apk,type=cache,sharing=locked --mount=target=/tmp,type=cache,sharing=locked <<EOF
    apk update
    apk add \
        ca-certificates~20260909-r3 \
        openssl~4
    update-ca-certificates
EOF

RUN <<EOF
    mkdir -p /app
    addgroup -S -g 1000 appgroup
    adduser -G appgroup -S -u 1000 appuser
    chown -R appuser:appgroup /app
    mkdir -p /home/appuser
    chown -R appuser:appgroup /home/appuser
    mkdir -p /usr/local/share
EOF

# CLI Production
FROM base AS cli-prod

USER 1000:1000
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
      org.opencontainers.image.base.name="cgr.dev/chainguard/wolfi-base"

ENTRYPOINT ["/usr/local/bin/diom"]

# Production
FROM cli-prod AS prod

# this is not a terminal USER stanza for this image
# hadolint ignore=DL3066
USER root

RUN <<EOF
    mkdir -p /storage
    chown -R appuser: /storage
EOF

ENV DIOM_PERSISTENT_DB_PATH="/storage/db"
# Should point to ephemeral storage in production
ENV DIOM_EPHEMERAL_DB_PATH="/storage/db-ephemeral"
ENV DIOM_CLUSTER_LOG_PATH="/storage/logs"
ENV DIOM_CLUSTER_SNAPSHOT_PATH="/storage/snapshots"

USER 1000:1000
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
      org.opencontainers.image.base.name="cgr.dev/chainguard/wolfi-base"

ENTRYPOINT ["/usr/local/bin/diom-server"]
