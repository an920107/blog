FROM rust:1-slim AS base
RUN apt-get update && \
    apt-get install -y --no-install-recommends build-essential libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

FROM base AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -j2

FROM debian:trixie-slim AS runner
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/server .
EXPOSE 8080
VOLUME ["/app/static"]
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV HOST=0.0.0.0
ENV PORT=8080
ENV STORAGE_PATH=/app/static
ENV DATABASE_HOST=127.0.0.1
ENV DATABASE_PORT=5432
ENV DATABASE_USER=postgres
ENV DATABASE_NAME=postgres
ENV REDIS_URL=redis://127.0.0.1:6379
ENV QDRANT_URL=http://127.0.0.1:6334
CMD ["./server"]
