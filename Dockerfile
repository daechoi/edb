FROM rust:1.79

RUN apt-get update \
     && apt-get install -y musl-tools protobuf-compiler \
     && rustup target add

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/edb"]
