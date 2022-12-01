# Build stage
FROM rust:latest as builder
WORKDIR /app
ADD . /app
RUN cargo build --release

# Prod stage
FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/oh_telemetry_collector_server_rust /
COPY configuration/* /app/configuration/*
COPY log4rs.yml /app/configuration/*
CMD ["./oh_telemetry_collector_server_rust"]