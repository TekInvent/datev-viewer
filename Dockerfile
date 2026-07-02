# Development container for Teki DATEV Viewer
# Contains: Rust toolchain + Node.js (for Vite/React)
# Tauri desktop build is intentionally excluded — that runs on the macOS host.

FROM rust:1.88-bookworm

# --- Tauri 2 Linux system prerequisites + Node.js (LTS) ---
RUN apt-get update && apt-get install -y --no-install-recommends \
    # pkg-config and build tools
    pkg-config \
    build-essential \
    file \
    wget \
    # Tauri required libraries
    libdbus-1-dev \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libxdo-dev \
    libsoup-3.0-dev \
    # Node.js setup
    curl ca-certificates \
    && curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y nodejs \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# --- Rust toolchain components ---
RUN rustup component add clippy rustfmt

WORKDIR /workspace

# Cache Cargo registry between runs
ENV CARGO_HOME=/cargo-cache
ENV PATH="${CARGO_HOME}/bin:${PATH}"

CMD ["bash"]
