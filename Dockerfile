FROM rust:1.82 as builder
WORKDIR /api-gateway
COPY . .
RUN cargo install --path . --root /usr/local/cargo

FROM debian:bookworm-slim
RUN apt-get update && apt-get install openssl libssl-dev libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/api-gateway /usr/local/bin/api-gateway
ENTRYPOINT ["api-gateway"]
