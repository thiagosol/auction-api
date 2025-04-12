# 1️⃣ Build Stage
FROM rust:1.84 AS builder

WORKDIR /app

RUN apt update && apt install -y musl-tools

COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

# 2️⃣ Runtime Stage
FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/auction-api-rust /app/auction-api-rust

RUN chmod +x /app/auction-api-rust

ARG DB_POSTGRES_USER
ENV DB_POSTGRES_USER=${DB_POSTGRES_USER}

ARG DB_POSTGRES_PASS
ENV DB_POSTGRES_PASS=${DB_POSTGRES_PASS}

ARG SERVER_IP
ENV SERVER_IP=${SERVER_IP}

EXPOSE 8080

CMD ["/app/auction-api-rust"]
