# Bragi CI image — the toolchain container the pipeline runs inside.
#
# DISTINCT from the product image (../Dockerfile): the product image *ships* the
# API + CLI; this image *builds and tests* them. It carries the whole toolchain —
# Rust, the JVM, uv, system Python, and the parser's fonts — so `make test`,
# `make golden-test`, and the JVM smoke path all run in one place, identically on
# a laptop and on GitHub.
#
# It deliberately COPYs no source: the image is toolchain-only, so the cargo
# registry/git caches and target/ live OUTSIDE it (named volumes locally,
# actions/cache on GitHub) and survive across runs. Baking source in would defeat
# the cache the whole phase exists to prove.
#
# JVM parity is not incidental. This is based on the SAME Liberica OpenJRE 21 as
# the product runtime, on purpose: font-metric parity across the JVM was hard-won
# (Temurin computes different glyph widths -> missing spaces in extracted text),
# so the CI image must not reintroduce that variable. It inherits the exact base.
FROM bellsoft/liberica-openjre-debian:21

# Rust version — keep in sync with rust-toolchain.toml (the single source of
# truth for the cargo/rustc pin). Overridable so the 1.97 -> 1.93 fallback is a
# one-line probe: docker build --build-arg RUST_VERSION=1.93 ...
ARG RUST_VERSION=1.97

# System toolchain:
#   build-essential/pkg-config/libssl-dev — the cargo build's C linker + openssl
#   python3/venv/pip                       — the mono's boundary + projection
#                                            scripts call bare `python3`
#   git                                    — cargo git deps + the version/PRODUCED_BY
#                                            `git rev-parse` the golden targets run
#   curl/ca-certificates                   — the rustup + uv installers
#   fonts + fontconfig                     — load-bearing for PDFBox word-boundary
#                                            detection; the identical trio to product
RUN apt-get update && apt-get install -y --no-install-recommends \
      build-essential \
      pkg-config \
      libssl-dev \
      python3 \
      python3-venv \
      python3-pip \
      git \
      curl \
      ca-certificates \
      fontconfig \
      fonts-dejavu-core \
      fonts-urw-base35 \
      fonts-liberation \
    && rm -rf /var/lib/apt/lists/* \
    && fc-cache -f

# Rust via rustup, pinned. CARGO_HOME/RUSTUP_HOME sit at fixed system paths so the
# registry + git caches are addressable as volumes (local) / actions/cache (CI).
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --no-modify-path --profile minimal --default-toolchain "${RUST_VERSION}" \
    && rustc --version && cargo --version

# uv for the Python SDK suite (the shipped SDK is uv-native — build + test).
RUN curl -LsSf https://astral.sh/uv/install.sh | env UV_INSTALL_DIR=/usr/local/bin sh \
    && uv --version

# JAVA_HOME identical to the product image so JVM invocation resolves the same.
ENV JAVA_HOME=/usr/lib/jvm/jre

# Fail the build here if any leg of the toolchain is missing or JAVA_HOME is wrong.
RUN echo "=== bragi-ci toolchain ===" \
    && rustc --version \
    && cargo --version \
    && uv --version \
    && python3 --version \
    && test -x "${JAVA_HOME}/bin/java" \
    && "${JAVA_HOME}/bin/java" -version
