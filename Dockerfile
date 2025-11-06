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
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && \
    pixi run -e prod cargo build --release && \
    rm -rf src target/dx

# Now copy the actual source code
COPY . .

# Build the actual application
RUN pixi run -e prod build

FROM ubuntu:24.04 AS production

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/dx/fs-chaot/release/web/ /usr/local/app

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

ENTRYPOINT ["/usr/local/app/fs-chaot"]
