# syntax=docker/dockerfile:1

# Parts of this file was generated using `docker init`.
# With tips from https://kerkour.com/rust-small-docker-image#from-alpine
# See also https://docs.docker.com/engine/reference/builder/

################################################################################
# Create a stage for building the front-end.

ARG DENO_VERSION=2.0.3
ARG NODE_VERSION=22
FROM docker.io/library/node:${NODE_VERSION}-bullseye-slim as builder
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

FROM docker.io/library/rust:1.85.1-slim as rust-builder

# Prepare static linker for minimal final
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
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
COPY Cargo.lock .
COPY Cargo.toml .

# Build the application.
RUN cargo build --target x86_64-unknown-linux-musl --release --locked

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application.
FROM docker.io/library/alpine as final

# Copy user from builder
COPY --from=rust-builder /etc/passwd /etc/passwd
COPY --from=rust-builder /etc/group /etc/group
WORKDIR /app

# Copy the executable from the "build" stage.
COPY --from=rust-builder /app/target/x86_64-unknown-linux-musl/release/portfolio ./

# Use the unprivileged user created previously
USER appuser:appuser

# The port that the application listens on.
EXPOSE 8787

ENTRYPOINT ["/app/portfolio"]
