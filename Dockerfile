FROM rust:1.74 AS builder
WORKDIR /usr/src/tamako
COPY . .
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres
RUN cargo build --release && mv ./target/release/tamako ./tamako

FROM debian:bookworm-slim
# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev pkg-config ca-certificates && apt-get clean && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/tamako/tamako /app/
COPY --from=builder /usr/src/tamako/migrations/ /app/migrations/
COPY --from=builder /usr/src/tamako/assets/ /app/assets/
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
CMD [ "/app/tamako" ]