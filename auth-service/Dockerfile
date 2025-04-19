FROM rustlang/rust:nightly as builder

WORKDIR /app
COPY . /app

RUN apt-get update && apt-get install -y libpq-dev pkg-config
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates libpq-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/auth-service .

ENV RUST_LOG=info
EXPOSE 3000

CMD ["./auth-service"]