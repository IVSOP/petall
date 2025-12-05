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
COPY .sqlx ./.sqlx

ENV SQLX_OFFLINE=true
RUN cargo build -p petall --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/petall /usr/local/bin/petall

ENV RUST_LOG=info
EXPOSE 8080

CMD ["petall","run","--bind-ip","0.0.0.0","--bind-port","8080"]
