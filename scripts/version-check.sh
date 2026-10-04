#!/usr/bin/env bash
#
# Assert the CODE/RELEASE version is coherent across every crate, the Python
# SDK, the self-hosted server and the example outputs quoted in the docs. Non-zero exit on drift. Use as a pre-publish / CI gate — this is the
# guard against the axes silently diverging again (the CR-87 failure mode,
# here for the code axis).
#
# The schema/format axis (BGRAPH_FORMAT_VERSION) is independent of the code
# axis by design, and the two are never compared with each other. The schema
# number is checked only against itself: every current-schema value quoted in a
# doc example (`"schema_version": "…"`, the SDK's `schema v…>` repr, a
# `schema_version  # "…"` comment) must equal the constant. Prose that names an
# older schema ("added in schema 1.1.0") does not match these shapes and is left
# alone.
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

# The schema axis: the constant, and every doc example quoting another value.
schema=$(awk -F'"' '/^pub const BGRAPH_FORMAT_VERSION/ { print $2; exit }' \
           "$ROOT/crates/core/src/preprocessors/md/mod.rs")
schema_drift=$(grep -rnoE '"schema_version": ?"[^"]*"|schema v[0-9][^>]*>|schema_version[[:space:]]+# "[^"]*"' \
                 "$ROOT/README.md" "$ROOT/docs" "$ROOT/py/sdk/README.md" --include='*.md' \
               | grep -vE "(\"|v)${schema//./\\.}(\"|>)" || true)

echo "  bragi-io-core (Cargo.toml) : $core"
echo "  bragi-io  cli (Cargo.toml) : $cli"
echo "  bragi-io  sdk (pyproject)  : $py"
echo "  self-hosted server (/health): $server"

echo "  schema (BGRAPH_FORMAT_VERSION): $schema"

if [[ -n "$doc_drift" ]]; then
  echo "  doc examples quoting another version:"
  echo "$doc_drift" | sed "s|^$ROOT/|    |"
fi
if [[ -z "$schema" ]]; then
  echo "  could not read BGRAPH_FORMAT_VERSION"
elif [[ -n "$schema_drift" ]]; then
  echo "  doc examples quoting another schema:"
  echo "$schema_drift" | sed "s|^$ROOT/|    |"
fi

status=0
if [[ "$core" == "$cli" && "$cli" == "$py" && "$py" == "$server" && -z "$doc_drift" ]]; then
  echo "✅ code/release version coherent: $core"
else
  echo "❌ version drift — every site must match. Fix with: make bump-version V=X.Y.Z" >&2
  status=1
fi
if [[ -n "$schema" && -z "$schema_drift" ]]; then
  echo "✅ schema examples current: $schema"
else
  echo "❌ schema drift — doc examples must quote the current schema ($schema); fix them by hand" >&2
  status=1
fi
exit "$status"
