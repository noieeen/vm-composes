FROM rust:1.86 as builder

WORKDIR /usr/src/auth-service
COPY . .

ENV SQLX_OFFLINE=true
RUN cargo build --release
# RUN cargo install --path .

# RUN apt-get update && apt-get install -y libpq-dev pkg-config

# FROM debian:bullseye-slim
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libpq-dev libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/auth-service/target/release/auth-service /usr/local/bin/auth-service

# ENV RUST_LOG=info
# EXPOSE 3000

CMD ["auth-service"]