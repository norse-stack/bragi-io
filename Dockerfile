# Stage 1: Build the Rust CLI binary.
# Pin to bookworm (glibc 2.36) to match the Liberica runtime base below
# (Debian 12 / glibc 2.36). An unpinned `rust:slim` pulls trixie (glibc 2.39+),
# whose binary then fails at runtime: `GLIBC_2.39 not found`.
FROM rust:1.93-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy workspace manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY crates/core/ crates/core/
COPY crates/cli/ crates/cli/

RUN cargo build --release -p bragi-io

# Stage 2: Runtime — Liberica JRE for font-metric parity with local dev JVM
# (Eclipse Temurin computes different glyph widths → missing spaces in extracted text)
FROM bellsoft/liberica-openjre-debian:21

# Install Python 3.11 and runtime deps
RUN apt-get update && apt-get install -y \
    python3.11 \
    python3.11-venv \
    python3-pip \
    ca-certificates \
    curl \
    fontconfig \
    fonts-dejavu-core \
    # URW Base35: metric-compatible with PostScript standard fonts (Times-Roman → Nimbus Roman, etc.)
    # Liberation: metric-compatible with Times New Roman, Arial, Courier New
    # These are critical for PDFBox word-boundary detection on PDFs with non-embedded fonts
    fonts-urw-base35 \
    fonts-liberation \
    && rm -rf /var/lib/apt/lists/* \
    && fc-cache -f

WORKDIR /app

# Copy CLI binary from builder (package is `bragi-io`, bin is `bragi`; kept at the
# internal path /app/bin/blazegraph-cli, which BLAZEGRAPH_CLI_PATH points at).
COPY --from=builder /build/target/release/bragi /app/bin/blazegraph-cli

# Copy Tika JAR and default processing config
COPY crates/core/deps/tika/jni-jars/blazing-tika-jni.jar /app/bin/blazing-tika-jni.jar
COPY crates/cli/configs/processing/config.yaml /app/bin/config.yaml

# Copy server code and install Python deps using python3.11
COPY server/ /app/server/
RUN python3.11 -m pip install --no-cache-dir --break-system-packages -r /app/server/requirements.txt

# Environment
ENV BLAZEGRAPH_CLI_PATH=/app/bin/blazegraph-cli
ENV BLAZEGRAPH_JAR_PATH=/app/bin/blazing-tika-jni.jar
ENV BLAZEGRAPH_CONFIG_PATH=/app/bin/config.yaml
ENV JAVA_HOME=/usr/lib/jvm/jre

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Clear PDFBox font cache on startup so it re-scans /usr/share/fonts/
# This ensures font package updates are always picked up (see CR-04)
CMD ["sh", "-c", "rm -f /root/.pdfbox.cache && python3.11 -m uvicorn server.main:app --host 0.0.0.0 --port 8080"]
