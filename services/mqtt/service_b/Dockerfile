# Dockerfile for Service B
FROM rust:latest as builder
WORKDIR /app
COPY . .
WORKDIR /app/benchmark/services/mqtt/service_b
RUN cargo build --release -p service_b

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/service_b ./service_b
CMD ["./service_b"]
