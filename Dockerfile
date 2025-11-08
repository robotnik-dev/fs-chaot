FROM ghcr.io/prefix-dev/pixi:latest AS builder

WORKDIR /app

ENV SSL_CERT_FILE=/app/.pixi/envs/prod/ssl/cacert.pem \
    CURL_CA_BUNDLE=/app/.pixi/envs/prod/ssl/cacert.pem \
    CARGO_HTTP_CAINFO=/app/.pixi/envs/prod/ssl/cacert.pem

# Copy only dependency files first to cache pixi install layer
COPY pixi.toml pixi.lock ./
RUN pixi install --locked -e prod

# Copy only Cargo files to cache dependency compilation
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/app/target \
    mkdir -p src && echo "fn main() {}" > src/main.rs && \
    pixi run -e prod cargo build --release && \
    rm -rf src

# Now copy the actual source code
COPY . .

# Build the actual application with BuildKit cache mounts
RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/app/target \
    pixi run -e prod build && \
    mkdir -p /tmp/output && \
    cp -r target/dx/fs-chaot/release/web /tmp/output/

FROM ubuntu:24.04 AS production

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /tmp/output/web/ /usr/local/app

# Create db directory and set permissions
RUN mkdir -p /usr/local/app/db && chmod 777 /usr/local/app/db

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

WORKDIR /usr/local/app

ENTRYPOINT ["/usr/local/app/fs-chaot"]
