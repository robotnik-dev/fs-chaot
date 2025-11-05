FROM ghcr.io/prefix-dev/pixi:latest AS builder

WORKDIR /app

COPY . .
# use `--locked` to ensure the lockfile is up to date with pixi.toml
RUN pixi install --locked -e prod

ENV SSL_CERT_FILE=/app/.pixi/envs/prod/ssl/cacert.pem \
    CURL_CA_BUNDLE=/app/.pixi/envs/prod/ssl/cacert.pem \
    CARGO_HTTP_CAINFO=/app/.pixi/envs/prod/ssl/cacert.pem
    # CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=/app/.pixi/envs/prod/bin/x86_64-conda-linux-gnu-cc

RUN pixi run -e prod build

FROM ubuntu:24.04 AS production

COPY --from=builder /app/target/dx/fs-chaot/release/web/ /usr/local/app

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

ENTRYPOINT ["/usr/local/app/fs-chaot"]
