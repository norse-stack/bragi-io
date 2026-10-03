#!/usr/bin/env bash
#
# Bundle-bump the Bragi CODE/RELEASE version across every crate + the
# Python SDK in one shot. This is the "which build?" axis (crate::VERSION, the
# cargo-publish / PyPI number), bumped whenever emitted output changes.
#
# It is DELIBERATELY SEPARATE from the schema/format axis
# (BGRAPH_FORMAT_VERSION, currently 1.0.0). Code evolves on its own line
# (bugfixes, formatting) without ever touching the consumer-facing schema
# shape. See architecture doc 15 (version model).
#
# Usage:  scripts/bump-version.sh <X.Y.Z>
#         make bump-version V=<X.Y.Z>
#
# Sites stamped (kept in lockstep):
#   - crates/core/Cargo.toml        [package] version  (bragi-io-core)
#   - crates/cli/Cargo.toml         [package] version  (bragi-io CLI)
#   - py/sdk/pyproject.toml  [project] version  (bragi-io SDK)
# The Python __init__.__version__ reads from installed metadata — no 4th site.
#
set -euo pipefail

VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
  echo "usage: $(basename "$0") <X.Y.Z>" >&2
  exit 2
fi
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+([-+][0-9A-Za-z.-]+)?$ ]]; then
  echo "error: '$VERSION' is not a valid X.Y.Z version" >&2
  exit 2
fi

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Stamp VERSION into only the FIRST `version = "..."` line of a manifest — the
# [package]/[project] version. Dependency versions come later and are never the
# first such line, so they are untouched.
stamp() {
  local file="$1"
  if [[ ! -f "$file" ]]; then
    echo "error: missing $file" >&2
    exit 1
  fi
  awk -v v="$VERSION" '
    !done && /^version[[:space:]]*=[[:space:]]*"/ {
      sub(/"[^"]*"/, "\"" v "\"")
      done = 1
    }
    { print }
  ' "$file" > "$file.tmp" && mv "$file.tmp" "$file"
  echo "  stamped $(basename "$(dirname "$file")")/$(basename "$file")"
}

# Keep an intra-workspace path-dependency's version REQUIREMENT in lockstep
# with the crate it points at. These crates publish together at one version, so
# the CLI's requirement on core must move with core — otherwise `cargo publish`
# fails to resolve. (`cargo` uses the path for local builds and the version for
# publish; both must agree.)
stamp_dep() {
  local file="$1" dep="$2"
  [[ -f "$file" ]] || { echo "error: missing $file" >&2; exit 1; }
  awk -v v="$VERSION" -v pat="^$2[[:space:]]*=[[:space:]]*\\{" '
    $0 ~ pat { sub(/version[[:space:]]*=[[:space:]]*"[^"]*"/, "version = \"" v "\"") }
    { print }
  ' "$file" > "$file.tmp" && mv "$file.tmp" "$file"
  echo "  stamped dep $dep in $(basename "$(dirname "$file")")/$(basename "$file")"
}

echo "Bumping code/release version → $VERSION"
stamp "$ROOT/crates/core/Cargo.toml"
stamp "$ROOT/crates/cli/Cargo.toml"
stamp "$ROOT/py/sdk/pyproject.toml"
stamp_dep "$ROOT/crates/cli/Cargo.toml" "bragi-io-core"
echo "Done. Run 'make version-check' to confirm coherence."
