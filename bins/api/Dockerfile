# Build stage
FROM rust:alpine AS builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY libs ./libs
COPY crates ./crates
COPY bins/api ./bins/api

# Build release binary
RUN cargo build --release --package api

# Runtime stage
FROM alpine:latest

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates libssl3

# Copy binary from builder
COPY --from=builder /app/target/release/api /app/api

EXPOSE 8080

CMD ["/app/api"]
