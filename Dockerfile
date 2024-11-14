# syntax=docker/dockerfile:1

# Parts of this file was generated using `docker init`.
# See also https://docs.docker.com/engine/reference/builder/

################################################################################
# Create a stage for building the application.

ARG DENO_VERSION=2.0.3
ARG NODE_VERSION=22
FROM docker.io/library/node:${NODE_VERSION}-bullseye-slim as builder
ARG DENO_VERSION
WORKDIR /app

# Copy necessary build files (avoiding bind mounts because Node wants extra permissions for them).
COPY functions functions
COPY public public
COPY src src
COPY .eslintrc .npmrc .prettierignore .prettierrc astro.config.ts deno.jsonc deno.lock package-lock.json package.json tsconfig.json .

# Build the application.
RUN <<EOF
set -eu

# Install Deno.
apt update && apt upgrade -y
apt install -y curl unzip
curl -fsSL https://deno.land/install.sh | sh
export PATH=$PATH:/root/.deno/bin

# Build the application.
deno install --frozen
npm ci
./node_modules/.bin/astro telemetry disable
deno task build
EOF

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application.
FROM docker.io/denoland/deno:alpine-${DENO_VERSION} as final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser
WORKDIR /app

# Copy the executables and web files from the "build" stage.
COPY --from=builder /app/dist dist
COPY --from=builder /app/node_modules node_modules
COPY --from=builder /app/vendor vendor
COPY --from=builder /app/functions functions
COPY --from=builder /app/deno.jsonc /app/deno.lock /app/package-lock.json /app/package.json /app/tsconfig.json .

# The port that the application listens on.
EXPOSE 8787

CMD ["deno", "task", "start"]
