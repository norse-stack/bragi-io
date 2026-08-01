#!/usr/bin/env bash
#
# Assert the CODE/RELEASE version is coherent across every crate + the Python
# SDK. Non-zero exit on drift. Use as a pre-publish / CI gate — this is the
# guard against the axes silently diverging again (the CR-87 failure mode,
# here for the code axis).
#
# Only the code axis is checked. The schema/format axis (BGRAPH_FORMAT_VERSION)
# is independent by design and intentionally NOT compared here.
#
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# First `version = "..."` line = the [package]/[project] version.
first_version() { awk -F'"' '/^version[[:space:]]*=/ { print $2; exit }' "$1"; }

core=$(first_version "$ROOT/crates/core/Cargo.toml")
cli=$(first_version "$ROOT/crates/cli/Cargo.toml")
py=$(first_version "$ROOT/py/sdk/pyproject.toml")

echo "  bragi-io-core (Cargo.toml) : $core"
echo "  blazegraph-io  cli (Cargo.toml) : $cli"
echo "  blazegraph-io  sdk (pyproject)  : $py"

if [[ "$core" == "$cli" && "$cli" == "$py" ]]; then
  echo "✅ code/release version coherent: $core"
else
  echo "❌ version drift — the three must match. Fix with: make bump-version V=X.Y.Z" >&2
  exit 1
fi
