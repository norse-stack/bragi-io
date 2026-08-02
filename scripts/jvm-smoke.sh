#!/usr/bin/env bash
# JVM smoke gate (T1.6 C3a) — a real PDF through the real JNI/Tika path, asserting
# the freshly-produced bgraph_sha256 and node count against the committed golden.
#
# This is the lane `make test` deliberately omits. `make test` and `golden-test`
# are JVM-free: the golden freeze replays a cached tier and panics if Tika is ever
# invoked. So the pdf->xhtml JNI hop — the one T1.5b R3 silently broke by renaming
# the `com/blazegraph/TikaMain` class string (a lookup no compiler checks) — has
# had no gate at all. This target is that gate.
#
# It is a SEPARATE lane, not folded into `make test`: keeping the core suite
# JVM-free is a feature (it is why golden-test can replay with no JVM). This adds
# the missing lane rather than compromising the hermetic one.
set -euo pipefail
cd "$(dirname "$0")/.."   # -> public/bragi (repo root when projected)

CLI=target/release/bragi
GOLDEN=crates/core/test_fixtures/golden/1.0.0/attention
PDF="$GOLDEN/attention.pdf"
CONFIG="$GOLDEN/config.yaml"
GOLDEN_MD="$GOLDEN/document.bgraph.md"
JRE="${JRE_PATH:-${JAVA_HOME:-$HOME/.sdkman/candidates/java/current}}"
JAR="${JAR_PATH:-crates/core/deps/tika/jni-jars/blazing-tika-jni.jar}"
EXPECT_NODES=179

test -x "$CLI"          || { echo "❌ CLI not built — run: make build-cli"; exit 1; }
test -f "$JAR"          || { echo "❌ Tika JAR missing: $JAR"; exit 1; }
test -x "$JRE/bin/java" || { echo "❌ no JVM at JRE_PATH=$JRE (expected bin/java)"; exit 1; }

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

echo "🔬 JVM smoke — fresh Tika parse via $("$JRE/bin/java" -version 2>&1 | head -1)"
# --fresh-from c0 forces the full pipeline (Tika/JVM -> c1 -> c2 -> graph); a
# TEMP cache-dir so we never overwrite the committed golden cache tiers. -c binds
# config_hash to the golden config.
#
# Deliberately WITHOUT --include-style-info: the committed golden is frozen with
# style=null, and this gate asserts reproduction of THAT blessed artifact. A fresh
# parse WITH the flag is byte-identical to the golden except each node's style
# field (and config_hash, which folds the flag in) — so style is the only variable
# and matching the golden means omitting it. `make golden-generate` DOES pass the
# flag; the committed golden predates it, an inconsistency C8's regeneration
# resolves (see the T1.6 flow, C3a). The class-lookup gate does not depend on it.
PREPROCESSOR_JRE_PATH="$JRE" PREPROCESSOR_JAR_PATH="$JAR" JAVA_HOME="$JRE" \
  "$CLI" parse -i "$PDF" -f bgraph-md \
    -c "$CONFIG" --fresh-from c0 --cache-dir "$TMP/cache" -o "$TMP/out.md" \
    2>&1 | tee "$TMP/parse.log"

sha() { grep -oE '"bgraph_sha256":"[0-9a-f]{64}"' "$1" | head -1 | grep -oE '[0-9a-f]{64}'; }
GOT_SHA=$(sha "$TMP/out.md" || true)
WANT_SHA=$(sha "$GOLDEN_MD" || true)
GOT_NODES=$(grep -oE 'Graph: [0-9]+ nodes' "$TMP/parse.log" | grep -oE '[0-9]+' | head -1 || true)

echo
echo "   nodes:         got=${GOT_NODES:-?}  want=$EXPECT_NODES"
echo "   bgraph_sha256: got=${GOT_SHA:-?}"
echo "                  want=${WANT_SHA:-?}"

fail=0
[ "${GOT_NODES:-}" = "$EXPECT_NODES" ] || { echo "❌ node count mismatch"; fail=1; }
{ [ -n "${GOT_SHA:-}" ] && [ "$GOT_SHA" = "$WANT_SHA" ]; } || { echo "❌ bgraph_sha256 mismatch — the JNI path did not reproduce the golden"; fail=1; }
[ "$fail" = 0 ] || { echo "❌ JVM smoke FAILED"; exit 1; }
echo "✅ JVM smoke passed — the real JNI/Tika path reproduces the golden (${EXPECT_NODES} nodes, sha ${GOT_SHA:0:12}…)"
