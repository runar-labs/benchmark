# Dockerfile for Service A
FROM rust:latest as builder
WORKDIR /app
COPY . .
WORKDIR /app/benchmark/services/mqtt/service_a
RUN cargo build --release -p service_a

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/service_a ./service_a
CMD ["./service_a"]
