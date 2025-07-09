# Stage 1: Build the Rust application
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Install any system dependencies required by your Rust crates (e.g., openssl, opus)
# For `audiopus_sys`, you'll likely need `libopus-dev` or similar packages depending on the base OS.
# For Debian/Ubuntu based images:
RUN apt-get update && apt-get install -y \
    libopus-dev \
    pkg-config \
    build-essential \
    libssl-dev \
    # Adding ffmpeg here for the build phase, good practice
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-gnu

# Build dependencies (dummy build)
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release --target x86_64-unknown-linux-gnu
RUN rm -rf src/*.rs # Remove dummy source file

COPY src ./src

RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-gnu

# Stage 2: Create the final, smaller runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install python3, pip, ffmpeg, and ca-certificates
# Make sure ffmpeg is installed with its full capabilities.
# In Debian, 'ffmpeg' typically brings most common codecs.
# If you run into issues with very obscure formats, you might need to
# consider a different base image or compiling ffmpeg yourself (more complex).
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 \
    python3-pip \
    ffmpeg \
    ca-certificates \
    # Clean up apt caches to keep the image size down
    && rm -rf /var/lib/apt/lists/*

# Install yt-dlp using pip, with the --break-system-packages flag
RUN pip3 install yt-dlp --break-system-packages

# Create logs directory with write permissions
RUN mkdir -p /app/logs && chmod 777 /app/logs

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/rust_bot ./rust_bot

CMD ["./rust_bot"]