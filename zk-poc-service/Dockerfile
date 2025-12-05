# syntax=docker/dockerfile:1.7

FROM rustlang/rust:nightly-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
 && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY common ./common
COPY community-backend ./community-backend
COPY zk-poc-service ./zk-poc-service

RUN cargo build -p zk-poc-service --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/zk-poc-service /usr/local/bin/zk-poc-service

EXPOSE 3002

CMD ["zk-poc-service","--listener","0.0.0.0:3002"]

