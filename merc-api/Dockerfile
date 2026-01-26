# Build stage
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY merc-error ./merc-error
COPY merc-events ./merc-events
COPY merc-storage ./merc-storage
COPY merc-api ./merc-api

# Build release binary
RUN cargo build --release --package merc-api

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/merc-api /app/merc-api

EXPOSE 8080

CMD ["/app/merc-api"]
