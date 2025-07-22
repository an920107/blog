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
ENV DATABASE_URL=postgres://postgres@localhost:5432/postgres
CMD ["./server"]
