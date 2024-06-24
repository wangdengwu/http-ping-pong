FROM rust as builder
WORKDIR /usr/src/http-pong
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN rustup target add wasm32-wasi
RUN cargo build --target wasm32-wasi --release

FROM  envoyproxy/envoy:v1.30-latest
COPY --from=builder /usr/src/http-pong/target/wasm32-wasi/release/pong.wasm /etc/envoy/pong.wasm
COPY envoy.yaml /etc/envoy/envoy.yaml