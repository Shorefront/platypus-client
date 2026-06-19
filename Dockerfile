FROM rust:1.92.0-trixie as builder
WORKDIR /usr/src/app

# Multi-stage Dockerfile for building a Leptos (Rust) application
RUN apt-get update && apt-get install -y --no-install-recommends \
	build-essential \
	pkg-config \
	libssl-dev \
	ca-certificates \
	&& rm -rf /var/lib/apt/lists/*

# Copy manifests first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./
# If workspace with multiple crates, adjust as needed
RUN mkdir src && echo "fn main() { println!(\"if you see this, the build used the cached stub\"); }" > src/main.rs

# Build a dummy release to cache dependencies
RUN cargo build --release || true

# Copy the real source
COPY . .

# Build the release binary (assumes binary target in Cargo.toml)
RUN cargo build --release

# Runtime stage
FROM debian:trixie-slim
WORKDIR /app

# Copy CA certs
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage. Adjust binary name if different.
COPY --from=builder /usr/src/app/target/release/platypus-client /app/platypus-client

ENV RUST_LOG=info
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
ENV PORT=3000

EXPOSE 3000

CMD ["/app/platypus-client"]
