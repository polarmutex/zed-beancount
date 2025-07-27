# Dockerfile for building Zed Beancount extension
FROM rust:1.88

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Add WASM target
RUN rustup target add wasm32-wasip1

# Create working directory
WORKDIR /app

# Copy source code
COPY . .

# Build the extension
RUN cargo build --release --target wasm32-wasip1

# Copy the built WASM file to a known location
RUN cp target/wasm32-wasip1/release/*.wasm extension.wasm