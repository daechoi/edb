FROM rust:1.79.0 as build

RUN apt update && apt install lld clang -y protobuf-compiler

#RUN apt-get update \
#    && apt-get install -y musl-tools protobuf-compiler \
#    && rustup target add x86_64-unknown-linux-musl

#ENV RUSTFLAGS=-Clinker=musl-gcc
WORKDIR /usr/src/edb

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src/bin  && mkdir -p src/client \
    && echo "fn main() {}" >src/bin/edb.rs \
    && echo "fn main() {}" >src/client/esql.rs \
    && echo "fn main() {}" >build.rs

RUN cargo fetch 

RUN cargo build --release \
    && rm -rf build.rs src target/x86_64-unknown-linux-gcc/release/edb*

COPY . .
RUN cargo build \
   && cargo install --debug --path . 

# Runtime image
FROM debian:stable-slim as runtime

RUN apt-get update -y \ 
  && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/local/cargo/bin/edb /usr/local/bin/edb
COPY --from=build /usr/src/edb/config/Config.yaml /etc/Config.yaml
CMD ["edb", "--config", "/etc/Config"]


#RUN cargo build --release
#CMD ["tail", "-f", "/dev/null"]
#CMD ["./target/release/edb"]
