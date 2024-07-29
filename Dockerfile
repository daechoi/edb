FROM rust:1.79.0 AS build

RUN apt update && apt install -y protobuf-compiler

WORKDIR /usr/src/edb

COPY Cargo.toml Cargo.lock build.rs . 
COPY ./src ./src
COPY ./proto ./proto

RUN cargo install --path .  --locked

COPY ./config ./config

# Runtime image
FROM debian:stable-slim AS runtime

RUN apt-get update -y \ 
  && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/local/cargo/bin/edb /usr/local/bin/edb
COPY --from=build /usr/src/edb/config/Config.yaml /etc/Config.yaml
CMD ["edb", "--config", "/etc/Config"]

