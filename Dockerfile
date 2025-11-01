FROM ghcr.io/prefix-dev/pixi:latest AS build

# copy source code, pixi.toml and pixi.lock to the container
WORKDIR /app
COPY . .
# install dependencies to `/app/.pixi/envs/prod`
# use `--locked` to ensure the lockfile is up to date with pixi.toml
RUN pixi install --locked -e prod

ENV SSL_CERT_FILE=/app/.pixi/envs/prod/ssl/cacert.pem \
    CURL_CA_BUNDLE=/app/.pixi/envs/prod/ssl/cacert.pem \
    CARGO_HTTP_CAINFO=/app/.pixi/envs/prod/ssl/cacert.pem
    # CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=/app/.pixi/envs/prod/bin/x86_64-conda-linux-gnu-cc

RUN pixi run -e prod build
# create the shell-hook bash script to activate the environment
RUN pixi shell-hook -e prod -s bash > /shell-hook
RUN echo "#!/bin/bash" > /app/entrypoint.sh
RUN cat /shell-hook >> /app/entrypoint.sh
# extend the shell-hook script to run the command passed to the container
RUN echo 'exec "$@"' >> /app/entrypoint.sh

FROM ubuntu:24.04 AS production
WORKDIR /app
# only copy the production environment into prod container
# please note that the "prefix" (path) needs to stay the same as in the build container
COPY --from=build /app/.pixi/envs/prod /app/.pixi/envs/prod
COPY --from=build --chmod=0755 /app/entrypoint.sh /app/entrypoint.sh
COPY --from=build /app/target/dx/fs-chaot/release/web/ /usr/local/app
# copy your project code into the container as well
# COPY ./fs-chaot /app/fs-chaot

RUN mkdir -p /db

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

ENTRYPOINT [ "/app/entrypoint.sh" ]
# run your app inside the pixi environment
CMD ["/usr/local/app/fs-chaot"]
