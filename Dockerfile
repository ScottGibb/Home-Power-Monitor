# Stage 1: Build the application
FROM rust:bookworm AS builder

RUN USER=root cargo new --bin app
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Production runtime
FROM alpine:latest AS prod
COPY --from=builder /app/target/release/home_power_monitor /usr/local/bin/home_power_monitor
CMD ["home_power_monitor"]

# Stage 3: Development runtime
FROM alpine:latest AS dev
COPY --from=builder /app/target/release/home_power_monitor /usr/local/bin/home_power_monitor
CMD ["home_power_monitor"]
