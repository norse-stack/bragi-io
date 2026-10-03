# Bragi submodule Makefile.
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
JAR_PATH ?= crates/core/deps/tika/jni-jars/blazing-tika-jni.jar

CLI_BIN     := target/release/bragi

# ---------------------------------------------------------------------------
# Golden freeze family (Block D — the cold-tier reconstruction anchor).
# ---------------------------------------------------------------------------
GOLDEN_DIR    := crates/core/test_fixtures/golden/1.0.0/attention
GOLDEN_CACHE  := crates/core/test_fixtures/snapshots
GOLDEN_PDF    := $(GOLDEN_DIR)/attention.pdf
GOLDEN_CONFIG := $(GOLDEN_DIR)/config.yaml
GOLDEN_MD     := $(GOLDEN_DIR)/document.bgraph.md
GOLDEN_SHA    := $(GOLDEN_DIR)/PRODUCED_BY

# ---------------------------------------------------------------------------
# Canon parse — the ONE command behind every number in the public docs
# (public-docs-generator). A DEFAULT-config parse of the canon document
# (attention) to both serializations; the docs cite THESE numbers and show the
# bare command form. Env-wired to the vendored JRE+JAR so it runs without the
# first-use auto-download. Output is scratch under target/ (gitignored).
# ---------------------------------------------------------------------------
CANON_PDF := $(GOLDEN_DIR)/attention.pdf
CANON_OUT := target/canon

.PHONY: build-cli build-archive parse-canon serve golden-generate golden-generate-docs golden-generate-all golden-test jvm-smoke docs-check golden-bless test sync-python-fixture test-python build-python publish-python hooks bump-version version-check

# ---------------------------------------------------------------------------
# Version — the CODE/release axis (crate::VERSION / cargo-publish + PyPI
# number), kept in lockstep across core + CLI + Python SDK. DELIBERATELY
# separate from the schema/format axis (BGRAPH_FORMAT_VERSION): code evolves on
# 0.x, the consumer-facing schema stays 1.x. See
# architecture doc 15 (version model).
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
	cargo build --release -p bragi-io

parse-canon: build-cli ## Parse the canon doc (attention) → json + bgraph.md — the docs' number source
	@mkdir -p $(CANON_OUT)
	PREPROCESSOR_JRE_PATH=$(JRE_PATH) PREPROCESSOR_JAR_PATH=$(JAR_PATH) JAVA_HOME=$(JRE_PATH) \
	  ./$(CLI_BIN) parse -i $(CANON_PDF) -o $(CANON_OUT)/attention.json
	PREPROCESSOR_JRE_PATH=$(JRE_PATH) PREPROCESSOR_JAR_PATH=$(JAR_PATH) JAVA_HOME=$(JRE_PATH) \
	  ./$(CLI_BIN) parse -i $(CANON_PDF) -f bgraph-md -o $(CANON_OUT)/attention.bgraph.md
	@echo "✅ canon → $(CANON_OUT)/attention.json + attention.bgraph.md"
	@ls -la $(CANON_OUT)

## build-archive: build the release CLI for one target triple and package the
## EXACT archive release.yml ships — the `bragi` binary plus the vendored Tika
## JAR. Extracted from release.yml (T1.6 C6 prep) so the build+package logic lives
## once, in the Makefile, and is exercised locally at rung 2 on Linux — leaving
## rung 5 (release.yml on a real repo) to prove only the matrix + upload +
## release-creation WIRING, not the build. The Windows .zip leg stays in the
## workflow (pwsh/Compress-Archive); this covers the three tar.gz targets. §9.
##   make build-archive TARGET=aarch64-unknown-linux-gnu   ->  bragi-io-<triple>.tar.gz
build-archive: ## Build + package the release tar.gz for TARGET=<triple> (bragi + Tika JAR)
	@test -n "$(TARGET)" || { echo "usage: make build-archive TARGET=<rust-triple>"; exit 2; }
	@test -f "$(JAR_PATH)" || { echo "❌ Tika JAR not found at $(JAR_PATH)"; exit 1; }
	cargo build --release -p bragi-io --target $(TARGET)
	rm -rf staging && mkdir staging
	cp target/$(TARGET)/release/bragi staging/
	cp $(JAR_PATH) staging/
	cd staging && tar -czf ../bragi-io-$(TARGET).tar.gz bragi blazing-tika-jni.jar
	@echo "✅ bragi-io-$(TARGET).tar.gz"
	@tar -tzf bragi-io-$(TARGET).tar.gz

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
	@# self-verifies (bgraph_sha256 covers node style_info). `-c $(GOLDEN_CONFIG)`
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
	@echo "   make golden-test   (or: cargo test -p bragi-io-core --test golden_freeze_tests)"

## golden-generate-docs: regenerate the JVM-free channel goldens (docx + md)
## from their committed source docs. The docx and markdown channels are
## pure-Rust (no Tika/JVM). These goldens are the single blessed fixtures the
## downstream (urd-adapters / urd-cli) reads directly. Verification tests for
## them are tracked in CR-91 (DRAFT).
GOLDEN_1_0_0 := crates/core/test_fixtures/golden/1.0.0
golden-generate-docs: build-cli
	@echo "📄 Regenerating the docx + md + ocr channel goldens (JVM-free)..."
	./$(CLI_BIN) parse -i $(GOLDEN_1_0_0)/demo-docx/source.docx -f bgraph-md -o $(GOLDEN_1_0_0)/demo-docx/document.bgraph.md
	./$(CLI_BIN) parse -i $(GOLDEN_1_0_0)/demo-md/source.md -f bgraph-md -o $(GOLDEN_1_0_0)/demo-md/document.bgraph.md
	./$(CLI_BIN) parse -i $(GOLDEN_1_0_0)/demo-ocr/source.json -f bgraph-md -o $(GOLDEN_1_0_0)/demo-ocr/document.bgraph.md
	@echo "✅ docx + md + ocr goldens regenerated under $(GOLDEN_1_0_0)/{demo-docx,demo-md,demo-ocr}/"

golden-generate-all: golden-generate golden-generate-docs ## Re-bless the entire 1.0.0 golden family (PDF + docx + md)

golden-test: ## Run the JVM-free golden freeze + roundtrip tests
	cargo test -p bragi-io-core --test golden_freeze_tests

## jvm-smoke: the lane make test leaves out — a real PDF through the real JNI/Tika
## path, asserting the fresh bgraph_sha256 + node count against the committed
## golden. Needs a JVM (JRE_PATH/JAVA_HOME) and the vendored JAR. This is the
## gate the `com/blazegraph/TikaMain` class string never had; T1.5b R3 broke that
## string and four green gates missed it. See scripts/jvm-smoke.sh, T1.6 C3a.
jvm-smoke: build-cli ## JVM smoke gate — a real JNI/Tika parse must reproduce the golden (needs a JVM)
	@scripts/jvm-smoke.sh

## docs-check: assert the published docs say what the code does — env vars vs the
## Dockerfile, routes vs the FastAPI app, the SDK type table vs __all__, and that
## relative links resolve. `make test` never reads docs/; this is that lane.
docs-check: ## Docs-truth gates: env/routes/types/links vs source (CR-94 §G)
	@python3 scripts/docs-check.py

## serve: run the self-hosted FastAPI parse API the public docs describe, so a
## docs run can actually curl it. uv supplies fastapi+uvicorn (py/server has a
## requirements.txt, no venv); the CLI env is wired so the server's subprocess
## parse finds the vendored JRE+JAR. Matches the Dockerfile CMD (port 8080).
serve: build-cli ## Run the self-hosted parse API at http://localhost:8080 (GET /health, POST /v1/parse/pdf)
	@command -v uv >/dev/null 2>&1 || { echo "❌ uv not found — needed to run the server (see py/server/requirements.txt)"; exit 1; }
	BRAGI_CLI_PATH=$(abspath $(CLI_BIN)) BRAGI_JAR_PATH=$(abspath $(JAR_PATH)) \
	PREPROCESSOR_JRE_PATH=$(JRE_PATH) JAVA_HOME=$(JRE_PATH) \
	  uv run --with-requirements py/server/requirements.txt \
	    uvicorn main:app --app-dir py/server --host 0.0.0.0 --port 8080

## golden-bless: re-freeze the ENTIRE golden family in-place, JVM-free — every
## channel's `document.bgraph.md` AND `document.bgraph.json`, plus the attention
## `PRODUCED_BY` + materialized `config.yaml`. Replays the committed C2 cache
## (attention) and re-parses the light sources (demo-md/docx); no Tika. Use
## after an INTENDED output change (a version bump or a deliberate schema move),
## then `make golden-test` must be green and the `git diff` must be only what you
## expect. If the PIPELINE changed (not just the version), run a fresh Tika parse
## first with `make golden-generate-all`, then this. See operations/release.md §3.
golden-bless: ## Re-bless the golden family (md + json) in-place, JVM-free
	BLESS_GOLDEN=1 cargo test -p bragi-io-core --test golden_freeze_tests

## test: the domain's full sweep — the Rust core + CLI (including the golden
## freeze, the json wire and the schema-contract boundary proof) AND the Python
## SDK suite. JVM-free: the golden freeze replays the committed C2 cache.
##
## The SDK suite is IN this target on purpose. It used to be a separate
## `test-python` nobody's gate called, so the whole Python surface — the
## client, the local runner, the download path, the type layer — sat outside
## every green checkmark the repo printed. `make test` is what the mono's
## domain runner and CI both call; anything not reachable from here is not
## actually tested. Pair with `make test-downstream` (api + urd) in the mono.
test: ## Run the full test suite — core + CLI + the Python SDK
	cargo test -p bragi-io-core -p bragi-io
	@$(MAKE) --no-print-directory test-python

# --- Python SDK ----------------------------------------------------------
PY_DIR      := py/sdk
PY_FIXTURES := $(PY_DIR)/tests/fixtures

## sync-python-fixture: regenerate the Python SDK's 1.0.0 graph.json fixtures
## from the golden sources (the repo ignores generated *.json, so these are
## rebuilt on demand rather than committed — same policy as the golden family,
## which commits bgraph.md, not graph.json). The PDF fixture replays the
## committed C2 cache (JVM-free, byte-consistent with the golden); the md
## fixture is a pure-Rust Free-flow parse. Ground truth for the SDK type tests.
sync-python-fixture: build-cli ## Regenerate the Python SDK 1.0.0 graph.json fixtures
	@echo "🐍 Regenerating Python SDK fixtures (1.0.0 graph.json)..."
	@# The dir holds only generated (gitignored) json, so git cannot track it and
	@# it does not survive a clone or a directory move. Create it, don't assume it.
	@mkdir -p $(PY_FIXTURES)
	PREPROCESSOR_JRE_PATH=$(JRE_PATH) PREPROCESSOR_JAR_PATH=$(JAR_PATH) JAVA_HOME=$(JRE_PATH) \
	./$(CLI_BIN) parse -i $(GOLDEN_PDF) -f bgraph --include-style-info \
		-c $(GOLDEN_CONFIG) -o $(PY_FIXTURES)/attention_graph.json \
		--cache-dir $(GOLDEN_CACHE) --fresh-from c2
	./$(CLI_BIN) parse -i $(GOLDEN_1_0_0)/demo-md/source.md -f bgraph \
		-o $(PY_FIXTURES)/demo_md_graph.json
	./$(CLI_BIN) parse -i $(GOLDEN_1_0_0)/demo-ocr/source.json -f bgraph \
		-o $(PY_FIXTURES)/demo_ocr_graph.json
	@echo "✅ Python fixtures regenerated under $(PY_FIXTURES)/"

## The SDK's test venv, created on demand. The repo's python is uv-managed, and
## the venv is gitignored — so a fresh clone (and every CI runner) has no venv at
## all. Building it here is what lets `test` depend on the SDK suite without
## assuming someone ran uv by hand first.
$(PY_DIR)/.venv/bin/python:
	@command -v uv >/dev/null 2>&1 || { \
	  echo "❌ uv not found — the Python SDK suite needs it: https://docs.astral.sh/uv/"; \
	  exit 1; }
	@echo "🐍 Creating the SDK test venv at $(PY_DIR)/.venv ..."
	uv venv $(PY_DIR)/.venv
	uv pip install --python $(PY_DIR)/.venv/bin/python -e "$(PY_DIR)[dev]"

test-python: sync-python-fixture $(PY_DIR)/.venv/bin/python ## Regenerate fixtures + run the Python SDK test suite
	cd $(PY_DIR) && .venv/bin/python -m pytest -q

## build-python: build the SDK sdist + wheel into py/sdk/dist/ with uv.
## The repo's python is uv-managed — the .venv carries no pip — so `uv build` (an
## isolated build env) is the right tool, not `python -m build`. Cleans dist/ FIRST:
## the dir historically accumulated stale 0.1.x–0.2.x artifacts and a publish ships
## everything in dist/. Then `uvx twine check` pre-flights metadata + README render,
## so a bad long_description fails HERE — not after the version is burned on PyPI
## (you can't re-upload a version). This is the buildable half of release.md §7.
build-python: ## Build + validate the Python SDK sdist + wheel (into py/sdk/dist/)
	@command -v uv >/dev/null 2>&1 || { echo "❌ uv not found — the SDK build is uv-native (see release.md §7)"; exit 1; }
	@echo "🐍 Building the Python SDK (sdist + wheel) with uv..."
	rm -rf $(PY_DIR)/dist
	cd $(PY_DIR) && uv build
	@echo "🔎 Pre-flight (twine check — metadata + README render)..."
	uvx twine check $(PY_DIR)/dist/*
	@echo "✅ Built + checked:"; ls -1 $(PY_DIR)/dist

## publish-python: build + upload the SDK to PyPI. Gated on `version-check` (the
## code/release version must be coherent across crates + SDK before anything ships).
## The upload needs a PyPI token: `UV_PUBLISH_TOKEN=pypi-… make publish-python` (or
## ~/.pypirc). This is a publish lever — run it LAST. release.md §7.
publish-python: version-check build-python ## Build + upload the Python SDK to PyPI (needs a PyPI token)
	@echo "🚀 Uploading bragi-io to PyPI (uv publish)..."
	cd $(PY_DIR) && uv publish
	@echo "✅ Published. Verify: pip install bragi-io"
