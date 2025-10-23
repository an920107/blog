FROM rust:1-alpine AS base
RUN apk add --no-cache build-base openssl-dev openssl-libs-static

FROM base AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest AS runner
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
ENV REIDS_URL=redis://127.0.0.1:6379
CMD ["./server"]
