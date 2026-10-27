# syntax=docker/dockerfile:1
#
# NOTE: build from the root directory with `docker build -f infra/operator/Dockerfile .`

# Using https://github.com/LukeMathWalker/cargo-chef for better layer caching

# Base image for planner and build - keep in sync with root Dockerfile
FROM cgr.dev/chainguard/wolfi-base@sha256:0feab531471d2edfcca55f18883441cb7df598e724fa2dedf9985273f532eed7 AS chef
SHELL ["/bin/sh", "-eux", "-c"]
RUN --mount=target=/var/cache/apk,type=cache,sharing=locked --mount=target=/tmp,type=cache,sharing=locked <<EOF
    apk add rust~1.98.1 \
        build-base~1 \
        openssl-dev~4 \
        zlib-dev~1.3 \
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
FROM chef AS build
SHELL ["/bin/sh", "-eux", "-c"]

ARG __BUST_DOCKER_BUILD_CACHE=2026-09-14

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer
RUN cargo chef cook --release -p diom-operator --recipe-path recipe.json

# Build the operator
COPY . .

ARG CARGO_LOG
ARG GITHUB_SHA
ARG RELEASE_VERSION
ARG WRITE_EXECUTABLE_METADATA="1"
RUN <<EOF
    cargo build --release -p diom-operator --bin diom-operator --frozen
    cargo sbom > /app/diom-operator.spdx
    strip target/release/diom-operator
EOF

# Production
FROM cgr.dev/chainguard/wolfi-base@sha256:0feab531471d2edfcca55f18883441cb7df598e724fa2dedf9985273f532eed7 AS prod
SHELL ["/bin/sh", "-eux", "-c"]

RUN --mount=target=/var/cache/apk,type=cache,sharing=locked --mount=target=/tmp,type=cache,sharing=locked <<EOF
    apk update
    apk add ca-certificates~20260909-r3
    update-ca-certificates
EOF

RUN <<EOF
    mkdir -p /app
    addgroup -S -g 1000 appgroup
    adduser -G appgroup -S -u 1000 appuser
    chown -R appuser:appgroup /app
    mkdir -p /home/appuser
    chown -R appuser:appgroup /home/appuser
EOF

USER 1000:1000
WORKDIR /home/appuser

COPY --chown=root:root --chmod=755 --from=build /app/target/release/diom-operator /usr/local/bin/diom-operator
COPY --chown=root:root --chmod=644 --from=build /app/diom-operator.spdx /diom-operator.spdx

LABEL org.opencontainers.image.authors="support@svix.com" \
      org.opencontainers.image.url="https://diom.svix.com" \
      org.opencontainers.image.documentation="https://diom.svix.com/docs" \
      org.opencontainers.image.description="The Diom backend components platform, kubernetes operator component" \
      org.opencontainers.image.title="diom operator" \
      org.opencontainers.image.vendor="Svix" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.source="https://github.com/svix/diom" \
      org.opencontainers.image.base.name="cgr.dev/chainguard/wolfi-base"

ENTRYPOINT ["/usr/local/bin/diom-operator"]
