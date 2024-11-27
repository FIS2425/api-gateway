FROM rust:slim-bookworm AS chef
RUN apt-get update && apt-get install -y \
    musl-dev libssl-dev pkg-config \
    && cargo install cargo-chef \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /api-gateway

FROM chef AS planner
COPY . .
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /api-gateway/recipe.json recipe.json
RUN cargo chef cook
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install openssl libssl-dev libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /api-gateway/target/release/hypergate /app/hypergate
COPY --from=builder /api-gateway/static/openapi.yaml /app/static/openapi.yaml
COPY --from=builder /api-gateway/static/openapi.html /app/static/openapi.html
COPY --from=builder /api-gateway/config.yaml /app/config.yaml
CMD ["/app/hypergate", "serve", "--conf", "/app/config.yaml", "--specs", "/app/static/openapi.yaml", "--html", "/app/static/openapi.html"]
