FROM rust:1.83-bullseye AS builder

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

# Install system dependencies
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

# Mold linker for efficiency
RUN curl -L --output /tmp/mold.tar.gz https://github.com/rui314/mold/releases/download/v2.35.0/mold-2.35.0-x86_64-linux.tar.gz \
    && cd /tmp \
    && tar xzf mold.tar.gz \
    && cp mold-*/bin/mold /usr/bin/ \
    && cp mold-*/lib/mold/mold-wrapper.so /usr/lib/ \
    && rm -rf /tmp/mold*

# Build the project
WORKDIR /usr/src/app
COPY . .
RUN cargo build -p jobs --release

# Runtime
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

COPY --from=builder /usr/src/app/target/release/jobs /app/

CMD ["/app/jobs"]
