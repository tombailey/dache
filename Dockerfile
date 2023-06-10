FROM ghcr.io/instrumentisto/rust:nightly-slim-2023-06-07 as builder

WORKDIR /app

COPY ./src ./src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY LICENSE LICENSE

RUN cargo install --path .
RUN cargo build --release



FROM debian:buster-slim

WORKDIR /app
COPY --from=builder /app/target/release/dache /app/dache

CMD ["./dache"]
