# Build stage
FROM rust:latest as builder
WORKDIR /usr/src/hxckr-core
COPY . .
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/hxckr-core/target/release/hxckr-core /usr/local/bin/hxckr-core
CMD ["hxckr-core"]