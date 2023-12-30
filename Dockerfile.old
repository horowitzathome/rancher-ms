# Dockerfile

# Use a Rust base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy the Rust project files
COPY . .

# Build the Rust program
RUN cargo build --release

# Create a new image with only the compiled binary
FROM debian:buster-slim

WORKDIR /app

# Copy the binary from the builder image
COPY --from=builder /app/target/release/rancher-ms .

# Set the entry point
CMD ["./rancher-ms"]