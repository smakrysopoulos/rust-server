FROM rust:slim-bullseye as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rust-server .

ENV RUST_LOG=debug
ENV MONGO_URI="mongodb://admin:password@db:27017"
CMD ["./rust-server"]