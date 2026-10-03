#!/usr/bin/env bash
#
# Assert the CODE/RELEASE version is coherent across every crate, the Python
# SDK, the self-hosted server and the example outputs quoted in the docs. Non-zero exit on drift. Use as a pre-publish / CI gate — this is the
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
# The self-hosted server's /health version: the FastAPI(version="...") keyword.
server=$(awk -F'"' '/^[[:space:]]+version="/ { print $2; exit }' "$ROOT/py/server/main.py")
# Every version quoted in a doc example that is not the core version, as file:line.
doc_drift=$(grep -rnoE '"bragi_version": "[^"]*"|"status": "healthy", "version": "[^"]*"' \
              "$ROOT/README.md" "$ROOT/docs" --include='*.md' \
            | grep -vF "\"$core\"" || true)

echo "  bragi-io-core (Cargo.toml) : $core"
echo "  bragi-io  cli (Cargo.toml) : $cli"
echo "  bragi-io  sdk (pyproject)  : $py"
echo "  self-hosted server (/health): $server"

if [[ -n "$doc_drift" ]]; then
  echo "  doc examples quoting another version:"
  echo "$doc_drift" | sed "s|^$ROOT/|    |"
fi

if [[ "$core" == "$cli" && "$cli" == "$py" && "$py" == "$server" && -z "$doc_drift" ]]; then
  echo "✅ code/release version coherent: $core"
else
  echo "❌ version drift — every site must match. Fix with: make bump-version V=X.Y.Z" >&2
  exit 1
fi
