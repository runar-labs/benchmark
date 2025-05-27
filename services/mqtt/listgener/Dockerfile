# Dockerfile for ListGener observer service
FROM rust:latest as builder
WORKDIR /app
COPY . .
WORKDIR /app/benchmark/services/mqtt/listgener
RUN cargo build --release -p listgener

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/listgener ./listgener
CMD ["./listgener"]
