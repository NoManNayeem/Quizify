# Use the official Rust image as a base for building
FROM rust:1.83.0 as builder

# Set the working directory
WORKDIR /app

# Install musl-tools for static linking
RUN apt-get update && apt-get install -y --no-install-recommends musl-tools

# Add the musl target
RUN rustup target add x86_64-unknown-linux-musl

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Copy the static directory
COPY static ./static

# Build the application with musl for static linking
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Use a minimal runtime image (alpine)
FROM alpine:latest

# Set the working directory
WORKDIR /app

# Install minimal dependencies
RUN apk add --no-cache ca-certificates

# Copy the statically linked binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/quizify-backend .

# Copy the static directory for serving static files
COPY --from=builder /app/static ./static

# Expose the port used by the application
EXPOSE 8080

# Start the application
CMD ["./quizify-backend"]
