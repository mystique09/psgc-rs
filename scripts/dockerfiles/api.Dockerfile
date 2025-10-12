FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/psgc-rs

# Runtime environment variables
ARG HOST
ARG PORT
ARG RUST_LOG
ARG RUST_BACKTRACE
ARG MODE

ENV HOST=${HOST:-0.0.0.0}
ENV PORT=${PORT:-3000}
ENV RUST_LOG=${RUST_LOG:-info,psgc_infrastructure=info,psgc_application=info,psgc_api=info,tokio=trace,runtime=trace,rbatis=info,actix_web=info}
ENV RUST_BACKTRACE=${RUST_BACKTRACE:-1}
ENV MODE=${MODE:-production}

# Copy the pre-built binary from the artifact
COPY ./api ./api

# Make the binary executable
RUN chmod +x ./api

CMD ["./api"]