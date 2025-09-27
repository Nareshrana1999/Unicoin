# Multi-stage Dockerfile for Unicoin
FROM rust:1.70-slim as rust-builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (cached layer)
RUN cargo build --release && rm -rf src

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Node.js stage for applications
FROM node:18-alpine as node-builder

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production

# Copy apps source
COPY apps ./apps

# Build applications
RUN npm run build:apps

# Final stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Create unicoin user
RUN useradd -r -s /bin/false unicoin

# Set working directory
WORKDIR /app

# Copy binary from rust-builder
COPY --from=rust-builder /app/target/release/unicoin-node /usr/local/bin/unicoin-node

# Copy apps from node-builder
COPY --from=node-builder /app/apps ./apps

# Create data directory
RUN mkdir -p /app/data && chown -R unicoin:unicoin /app

# Switch to unicoin user
USER unicoin

# Expose ports
EXPOSE 30303 8545 8546

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8545/health || exit 1

# Default command
CMD ["unicoin-node", "--config", "/app/config.toml"]
