# Multi-stage Dockerfile for Rust 1.89
FROM rust:1.89 AS builder

# Set the working directory inside the container
WORKDIR /usr/src/wedding

# Copy the full project and build directly to ensure the built binary matches the repo
COPY . .
RUN cargo build --release


# Final runtime image: Debian slim for compact size
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage. The crate name (from Cargo.toml) is `wedding`.
COPY --from=builder /usr/src/wedding/target/release/wedding /usr/local/bin/wedding

RUN chmod +x /usr/local/bin/wedding \
	&& useradd -m appuser && chown appuser:appuser /usr/local/bin/wedding

USER appuser

# When the container runs, execute the wedding binary
CMD ["/usr/local/bin/wedding"]
