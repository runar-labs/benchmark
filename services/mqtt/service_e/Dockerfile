# Dockerfile for Service E
FROM rust:latest as builder
WORKDIR /app
COPY . .
WORKDIR /app/benchmark/services/mqtt/service_e
RUN cargo build --release -p service_e

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/service_e ./service_e
CMD ["./service_e"]
