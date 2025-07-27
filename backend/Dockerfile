FROM rust:1-slim AS base
RUN apt update -qq && apt install -y -qq --no-install-recommends musl-tools
RUN rustup target add x86_64-unknown-linux-musl

FROM base AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest AS runner
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server .
EXPOSE 8080
VOLUME ["/app/static"]
ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=8080
ENV STORAGE_PATH=/app/static
ENV DATABASE_HOST=127.0.0.1
ENV DATABASE_PORT=5432
ENV DATABASE_USER=postgres
ENV DATABASE_PASSWORD=
ENV DATABASE_NAME=postgres
CMD ["./server"]
