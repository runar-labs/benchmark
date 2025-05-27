# Dockerfile for Service C
FROM rust:latest as builder
WORKDIR /app
COPY . .
WORKDIR /app/benchmark/services/mqtt/service_c
RUN cargo build --release -p service_c

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/service_c ./service_c
CMD ["./service_c"]
