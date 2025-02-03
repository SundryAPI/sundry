FROM rust:1.83-bullseye AS builder

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
ARG PUBLIC_DEV_MODE
ENV PUBLIC_DEV_MODE=$PUBLIC_DEV_MODE
ARG PUBLIC_GITHUB_APP_NAME
ENV PUBLIC_GITHUB_APP_NAME=$PUBLIC_GITHUB_APP_NAME
ARG PUBLIC_GITHUB_APP_CLIENT_ID=$PUBLIC_GITHUB_APP_CLIENT_ID
ARG PUBLIC_SLACK_CLIENT_ID
ENV PUBLIC_SLACK_CLIENT_ID=$PUBLIC_SLACK_CLIENT_ID
ARG PUBLIC_SLACK_SCOPES
ENV PUBLIC_SLACK_SCOPES=$PUBLIC_SLACK_SCOPES
ARG PUBLIC_SLACK_USER_SCOPES
ENV PUBLIC_SLACK_USER_SCOPES=$PUBLIC_SLACK_USER_SCOPES

# system dependencies
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends \
    openssl \
    ca-certificates \
    pkg-config \
    postgresql-client \
    clang \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Install sass
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get update && apt update -y \
  && apt-get install -y --no-install-recommends nodejs \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
RUN npm install -g sass

# cargo-binstall for faster Rust dependencies
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && cp cargo-binstall /usr/local/cargo/bin

# Mold linker for efficiency
RUN curl -L --output /tmp/mold.tar.gz https://github.com/rui314/mold/releases/download/v2.35.0/mold-2.35.0-x86_64-linux.tar.gz \
    && cd /tmp \
    && tar xzf mold.tar.gz \
    && cp mold-*/bin/mold /usr/bin/ \
    && cp mold-*/lib/mold/mold-wrapper.so /usr/lib/ \
    && rm -rf /tmp/mold*

# Leptos for devX
RUN cargo binstall cargo-leptos -y --version 0.2.24

# WASM support for Leptos
RUN rustup target add wasm32-unknown-unknown

# The locked version of wasm-bindgen
RUN cargo binstall wasm-bindgen-cli --version 0.2.99

# Build
WORKDIR /usr/src/app
COPY . .
RUN cargo leptos build -p website --release


FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install certificates and other runtime dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/website /app/
COPY --from=builder /usr/src/app/target/site /app/site
COPY --from=builder /usr/src/app/Cargo.toml /app/

ARG LEPTOS_SITE_HOST
ARG LEPTOS_SITE_PORT
ENV LEPTOS_SITE_ADDR="${LEPTOS_SITE_HOST}:${LEPTOS_SITE_PORT}"
ENV LEPTOS_SITE_ROOT="site"

EXPOSE $LEPTOS_SITE_PORT

# Run the server
CMD ["/app/website"]
