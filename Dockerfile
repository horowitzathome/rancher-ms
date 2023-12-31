# Use a Rust base image with musl libc
FROM ekidd/rust-musl-builder as builder

# Set the working directory
WORKDIR /app

# Copy the Rust project files
COPY . .

# Build the Rust program
RUN cargo build --release

# Create a new image with only the compiled binary
FROM scratch

WORKDIR /app

# Copy the binary from the builder image
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rancher-ms .

# Set the entry point
CMD ["./rancher-ms"]
