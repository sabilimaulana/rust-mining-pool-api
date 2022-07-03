FROM rust:latest as builder

RUN cargo new --bin rust-mining-pool-api
WORKDIR ./rust-mining-pool-api
COPY . .
RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder /rust-mining-pool-api/target/release/rust-mining-pool-api ./rust-mining-pool-api
RUN apt update && apt install libpq-dev -y
CMD ["./rust-mining-pool-api"]