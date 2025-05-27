# Dockerfile for Service D
FROM rust:latest as builder
WORKDIR /app
COPY . .
WORKDIR /app/benchmark/services/mqtt/service_d
RUN cargo build --release -p service_d

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/service_d ./service_d
CMD ["./service_d"]
