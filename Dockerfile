FROM rust:1.88-slim

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && echo "" > src/lib.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source
COPY . .

# Touch to ensure rebuild
RUN touch src/main.rs src/lib.rs

# Build
RUN cargo build --release

EXPOSE 3000
CMD ["./target/release/api2spec-fixture-rocket"]
