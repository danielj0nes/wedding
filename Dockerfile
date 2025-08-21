# Multi-stage Dockerfile for Rust 1.89
FROM rust:1.89 AS builder
WORKDIR /usr/src/wedding

# Copy backend crate
COPY backend/wedding/Cargo.toml backend/wedding/Cargo.lock* ./
COPY backend/wedding/ ./

# Copy built frontend assets into the builder so we can include them in the final image
# (assumes your frontend build artifacts are in frontend/dist)
COPY frontend/dist /usr/src/wedding/frontend_dist

# Build the release binary
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim
WORKDIR /app

# Install ca-certificates (common requirement)
RUN apt-get update \
	&& apt-get install -y --no-install-recommends ca-certificates \
	&& rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/wedding/target/release/wedding /usr/local/bin/wedding


# Copy frontend static files into both /frontend/dist (for the server's relative path)
# and /app/static (previous location) so either path will work
COPY --from=builder /usr/src/wedding/frontend_dist /app/static

RUN chmod +x /usr/local/bin/wedding \
	&& useradd -m appuser \
	&& chown -R appuser:appuser /usr/local/bin/wedding /app/static

USER appuser

# Tell the app where static files live via env var
ENV WEDDING_FRONTEND_DIR=/app/static

EXPOSE 8080

# Execute the wedding binary when the container starts
CMD ["/usr/local/bin/wedding"]
