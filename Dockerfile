FROM rust:1.77.0-bookworm as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:12.5-slim
WORKDIR /app
RUN apt-get update && \
	apt-get install -y \
		zip \
		ca-certificates \
		openssl && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/xylo /app/xylo
ENTRYPOINT ["/app/xylo", "--config", "/etc/xylo/config.yaml"]
