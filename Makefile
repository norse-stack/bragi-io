# Blazegraph submodule Makefile.
#
# The parent-repo Makefile carries the full dev workflow (build-jar, run-*,
# sb-eval, the stage-fixture generators, …). This file holds only the
# self-contained targets that belong WITH the submodule so they travel on the
# submodule branch and are runnable from a bare submodule checkout.
#
# Currently: `golden-generate` — rebuild the Block D golden freeze family.

# ---------------------------------------------------------------------------
# Toolchain resolution (mirrors the parent Makefile's defaults).
# ---------------------------------------------------------------------------
JRE_PATH ?= $(or $(JAVA_HOME),$(HOME)/.sdkman/candidates/java/current)
JAR_PATH ?= blazegraph-core/deps/tika/jni-jars/blazing-tika-jni.jar

CLI_BIN     := target/release/blazegraph-io

# ---------------------------------------------------------------------------
# Golden freeze family (Block D — the cold-tier reconstruction anchor).
# ---------------------------------------------------------------------------
GOLDEN_DIR    := blazegraph-core/test_fixtures/golden/1.0.0/attention
GOLDEN_CACHE  := blazegraph-core/test_fixtures/snapshots
GOLDEN_PDF    := $(GOLDEN_DIR)/attention.pdf
GOLDEN_CONFIG := $(GOLDEN_DIR)/config.yaml
GOLDEN_MD     := $(GOLDEN_DIR)/document.bgraph.md
GOLDEN_SHA    := $(GOLDEN_DIR)/PRODUCED_BY

.PHONY: build-cli golden-generate golden-generate-docs golden-generate-all golden-test hooks bump-version version-check

# ---------------------------------------------------------------------------
# Version — the CODE/release axis (crate::VERSION / cargo-publish + PyPI
# number), kept in lockstep across core + CLI + Python SDK. DELIBERATELY
# separate from the schema/format axis (BGRAPH_FORMAT_VERSION): code evolves on
# 0.x, the customer-facing schema stays 1.x. See
# P2/core/architecture/15-version-model.md.
# ---------------------------------------------------------------------------
bump-version: ## Bundle-bump the code/release version everywhere: make bump-version V=0.5.0
	@test -n "$(V)" || { echo "usage: make bump-version V=X.Y.Z"; exit 2; }
	@scripts/bump-version.sh $(V)

version-check: ## Assert the code/release version is coherent across all crates + SDK
	@scripts/version-check.sh

hooks: ## Enable the repo's secret-scanning git hooks (see .githooks/README.md)
	git config core.hooksPath .githooks
	@echo "✅ Hooks enabled (core.hooksPath=.githooks)."
	@command -v gitleaks >/dev/null 2>&1 || echo "⚠  gitleaks not found — install it: brew install gitleaks"

build-cli: ## Build the JNI CLI (release) — needed to run a fresh Tika parse
	cargo build --release -p blazegraph-io

## golden-generate: rebuild the golden freeze family from the PDF with a CLEAN,
## FRESH Tika parse (needs the JVM). `--fresh-from c0` forces Tika to run and
## rebuilds C1 -> C2 from scratch — the family is NEVER seeded from a stale
## cache. Emits the style-bearing bgraph.md from that same run and records the
## producing codebase sha. The JVM-free `golden_freeze_tests` then replay from
## the fresh C2, so their bytes match by construction.
golden-generate: build-cli
	@echo "🧊 Regenerating the Block D golden freeze family (fresh Tika parse)..."
	@if [ ! -f "$(JAR_PATH)" ]; then \
		echo "❌ Tika JAR not found at $(JAR_PATH) — build it from the parent repo: make build-jar"; \
		exit 1; \
	fi
	@# Clean the committed cache tiers so we can never freeze stale bytes.
	rm -rf $(GOLDEN_CACHE)/c1-xhtml $(GOLDEN_CACHE)/c2-preprocessor \
	       $(GOLDEN_CACHE)/c3-graph $(GOLDEN_CACHE)/c0-pdf \
	       $(GOLDEN_CACHE)/debug $(GOLDEN_CACHE)/stat
	@# Fresh, full-pipeline parse: Tika (C0->C1) + preprocessor (->C2) + build
	@# + emit. `--include-style-info` puts `style` on the wire so the frozen md
	@# self-verifies (graph_sha256 covers node style_info). `-c $(GOLDEN_CONFIG)`
	@# binds the emitted config_hash to the committed golden config.
	PREPROCESSOR_JRE_PATH=$(JRE_PATH) PREPROCESSOR_JAR_PATH=$(JAR_PATH) \
	JAVA_HOME=$(JRE_PATH) \
	./$(CLI_BIN) parse \
		-i $(GOLDEN_PDF) \
		-f bgraph-md \
		--include-style-info \
		-c $(GOLDEN_CONFIG) \
		-o $(GOLDEN_MD) \
		--cache-dir $(GOLDEN_CACHE) \
		--fresh-from c0
	@# Record the producing codebase sha (the codebase_sha binding — a sidecar,
	@# never a serialized artifact field).
	git rev-parse HEAD > $(GOLDEN_SHA)
	@echo "✅ Golden family regenerated:"
	@echo "   md:          $(GOLDEN_MD)"
	@echo "   PRODUCED_BY: $$(cat $(GOLDEN_SHA))"
	@echo "   C1/C2 cache: $(GOLDEN_CACHE)/{c1-xhtml,c2-preprocessor}/"
	@echo ""
	@echo "Next: verify the JVM-free replay reproduces it byte-for-byte:"
	@echo "   make golden-test   (or: cargo test -p blazegraph-io-core --test golden_freeze_tests)"

## golden-generate-docs: regenerate the JVM-free channel goldens (docx + md)
## from their committed source docs. The docx and markdown channels are
## pure-Rust (no Tika/JVM). These goldens are the single blessed fixtures the
## downstream (urd-adapters / urd-cli) reads directly. Verification tests for
## them are tracked in CR-91 (DRAFT).
GOLDEN_1_0_0 := blazegraph-core/test_fixtures/golden/1.0.0
golden-generate-docs: build-cli
	@echo "📄 Regenerating the docx + md channel goldens (JVM-free)..."
	./$(CLI_BIN) parse -i $(GOLDEN_1_0_0)/demo-docx/source.docx -f bgraph-md -o $(GOLDEN_1_0_0)/demo-docx/document.bgraph.md
	./$(CLI_BIN) parse -i $(GOLDEN_1_0_0)/demo-md/source.md -f bgraph-md -o $(GOLDEN_1_0_0)/demo-md/document.bgraph.md
	@echo "✅ docx + md goldens regenerated under $(GOLDEN_1_0_0)/{demo-docx,demo-md}/"

golden-generate-all: golden-generate golden-generate-docs ## Re-bless the entire 1.0.0 golden family (PDF + docx + md)

golden-test: ## Run the JVM-free golden freeze + roundtrip tests
	cargo test -p blazegraph-io-core --test golden_freeze_tests
