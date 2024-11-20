# Specify the base builder image
FROM rust:1.82 as builder

# Install required dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /app
RUN cargo new --bin app
WORKDIR /app/app

# Copy over manifests
COPY Cargo.lock Cargo.toml ./

# Build only the dependencies to cache them
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/app/target \
    cargo build --release && \
    rm src/*.rs

# Copy the source code
COPY src ./src
COPY config.yaml ./config.yaml

# Build the application with cached dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/app/target \
    cargo build --release

# Create the runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary and config
COPY --from=builder /app/app/target/release/api-gateway /usr/local/bin/
COPY --from=builder /app/app/config.yaml /etc/api-gateway/

# Set the binary as the entrypoint
ENTRYPOINT ["/usr/local/bin/api-gateway"]
