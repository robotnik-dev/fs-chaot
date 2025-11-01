FROM ghcr.io/prefix-dev/pixi:0.40.0 AS builder

WORKDIR /app

COPY . .


RUN pixi install --locked

ENV SSL_CERT_FILE=/app/.pixi/envs/default/ssl/cacert.pem \
    CURL_CA_BUNDLE=/app/.pixi/envs/default/ssl/cacert.pem \
    CARGO_HTTP_CAINFO=/app/.pixi/envs/default/ssl/cacert.pem

RUN pixi run build

FROM ubuntu:24.04

COPY --from=builder /app/target/dx/fs-chaot/release/web/ /usr/local/app

RUN mkdir -p /db

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

ENTRYPOINT [ "/usr/local/app/fs-chaot" ]

