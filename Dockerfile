# syntax=docker/dockerfile:1

# --- Stage 1: Build binary ---
FROM rust:1-slim-bookworm AS builder

WORKDIR /app

# Copy dependency manifests first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source files to compile dependencies only
RUN mkdir -p src && \
    echo "pub mod dbs;" > src/lib.rs && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy real source code
COPY src ./src

# Touch source files to ensure cargo re-compiles the binary
RUN touch src/main.rs src/lib.rs && \
    cargo build --release

# --- Stage 2: Minimal runtime image ---
FROM debian:bookworm-slim AS runner

WORKDIR /app

# Install certificates for secure connections and clean cache
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Run as non-root user
RUN useradd -m -u 1000 appuser
USER appuser

# Copy binary from builder
COPY --from=builder /app/target/release/proj_1 /app/proj_1

ENV APP_PORT=3000
EXPOSE 3000

CMD ["/app/proj_1"]

