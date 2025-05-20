# syntax=docker/dockerfile:1

# Parts of this file was generated using `docker init`.
# With tips from https://kerkour.com/rust-small-docker-image#from-alpine
# See also https://docs.docker.com/engine/reference/builder/

################################################################################
# Create a stage for building the front-end.

ARG DENO_VERSION=2.0.3
ARG NODE_VERSION=22
FROM docker.io/library/node:${NODE_VERSION}-bullseye-slim AS builder
ARG DENO_VERSION
WORKDIR /app

# Copy necessary build files (avoiding bind mounts because Node wants extra permissions for them).
COPY functions functions
COPY public public
COPY src src
COPY eslint.config.mjs .editorconfig .npmrc .prettierignore .prettierrc astro.config.ts deno.jsonc deno.lock package-lock.json package.json tsconfig.json .

# Build the website.
RUN <<EOF
set -eu

# Install Deno.
apt update && apt upgrade -y
apt install -y curl unzip
curl -fsSL https://deno.land/install.sh | sh
export PATH=$PATH:/root/.deno/bin

# Build.
deno install --frozen
npm ci
./node_modules/.bin/astro telemetry disable
deno task build
EOF

################################################################################
# Create a stage for building the application.

FROM docker.io/library/rust:1.86.0-alpine AS rust-builder

# Prepare static linker and install OpenSSL dependency
RUN apk add musl-dev pkgconf openssl-dev openssl-libs-static
RUN update-ca-certificates

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
ENV USER=appuser
ENV UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"
WORKDIR /app

# Copy necessary build files
COPY --from=builder /app/dist dist
COPY ./functions ./functions
COPY ./public ./public
COPY ./src ./src
COPY build.rs .
COPY Cargo.lock .
COPY Cargo.toml .

# Build the application.
# FIXME: Somehow cache the dependencies' build; these take ages to compile now!
RUN cargo build --release --locked

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application.
FROM docker.io/library/alpine AS final

# Copy user from builder
COPY --from=rust-builder /etc/passwd /etc/passwd
COPY --from=rust-builder /etc/group /etc/group
WORKDIR /app

# Copy the executable from the "build" stage.
COPY --from=rust-builder /app/target/release/portfolio ./

# Use the unprivileged user created previously
USER appuser:appuser

# The port that the application listens on.
EXPOSE 1965
EXPOSE 8787

ENTRYPOINT ["/app/portfolio"]
