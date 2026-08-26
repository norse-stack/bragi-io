//! Block D — the golden-freeze anchor (museum layer ①).
//!
//! Freezes one real document's emitted **bgraph.md** at `1.0.0` as a
//! *reconstruction anchor*: the durable artifact a future version-pinned
//! binary (layer ②, deferred) can use to prove it still reproduces this
//! edition.
//!
//! The freeze check is **JVM-free**. It replays the deterministic identity
//! path (`process_document_with_cache` → `rules_and_graph` →
//! `build_graph_deterministic`, a pure function of `PreprocessorOutput` +
//! config) from a **committed C2 preprocessor cache**. A C2 cache hit skips
//! Tika (extraction) *and* the XHTML parse entirely — the stub preprocessor
//! below panics if either is ever reached, so a JVM/Tika invocation fails
//! the test loudly rather than passing silently.
//!
//! Two tests:
//!   * **A — reproduction (freeze):** regenerate bgraph.md JVM-free from the
//!     committed C2 cache and assert it is **byte-identical** to the frozen
//!     `golden/1.0.0/attention/document.bgraph.md`. Under `BLESS_GOLDEN=1`,
//!     re-freeze (write the md + refresh `PRODUCED_BY`) instead of asserting.
//!   * **B — roundtrip:** `parse_markdown` the frozen md and assert the
//!     identity verdict is `Verified` (doc-level `bgraph_sha256`
//!     self-consistency).
//!
//! Family layout (all committed):
//!   test_fixtures/golden/1.0.0/attention/attention.pdf        — the source
//!   test_fixtures/golden/1.0.0/attention/config.yaml          — the config
//!   test_fixtures/golden/1.0.0/attention/document.bgraph.md   — the freeze
//!   test_fixtures/golden/1.0.0/attention/PRODUCED_BY          — codebase sha
//!   test_fixtures/snapshots/{c1-xhtml,c2-preprocessor}/<sha>  — the cache
//!
//! Design-flow authority:
//! `docs/P2/core/design-flows/2026-07-06-canonical-versioning-and-fixture-stability.md`
//! (Block D). Purely additive: touches no core types, bumps no version,
//! freezes only bgraph.md (bgraph.json is CR-88).

use bragi_io_core::config::ParsingConfig;
use bragi_io_core::graphs::serialization::canonical::bgraph_sha256;
use bragi_io_core::graphs::serialization::markdown::emit_markdown;
use bragi_io_core::graphs::serialization::version::FormatVersion;
use bragi_io_core::preprocessors::docx::parse_docx;
use bragi_io_core::preprocessors::md::{parse_markdown, ParseIdentity, ParseOptions};
use bragi_io_core::preprocessors::ocr::parse_ocr;
use bragi_io_core::preprocessors::Preprocessor;
use bragi_io_core::processor::DocumentProcessor;
use bragi_io_core::storage::{CacheDefaults, FileStorage, FreshFrom};
use bragi_io_core::types::{DocumentGraph, ParseProvenance, PreprocessorOutput, SortedDocumentGraph};
use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};

// =========================================================================
// JVM-free guard: a preprocessor stub that panics if Tika or the XHTML
// parse is ever reached. Reaching either means the committed C2 cache did
// NOT hit — which would make the freeze depend on a live JVM. We want that
// to be a loud test failure, not a silent slow pass.
// =========================================================================

struct NoJvmPreprocessor;

impl Preprocessor for NoJvmPreprocessor {
    fn parse_pdf_to_markup_language(&self, _pdf_bytes: &[u8]) -> anyhow::Result<String> {
        panic!(
            "Tika/JVM extraction was invoked — the golden freeze must replay from the \
             committed C2 preprocessor cache (a C2 hit skips extraction). A cache miss here \
             means the committed C2 tier is absent or its pdf_hash no longer matches \
             attention.pdf. Rebuild the family with `make golden-generate`."
        );
    }

    fn parse_markup_to_preprocessor_output(
        &self,
        _markup: &str,
    ) -> anyhow::Result<PreprocessorOutput> {
        panic!(
            "The XHTML → PreprocessorOutput parse was invoked — the golden freeze must replay \
             from the committed C2 cache, not re-parse C1 XHTML. A C2 hit skips this step."
        );
    }

    fn name(&self) -> &str {
        "no-jvm-golden-freeze-stub"
    }

    fn supports_file_type(&self, _path: &Path) -> bool {
        true
    }
}

// =========================================================================
// Paths.
// =========================================================================

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_fixtures/golden/1.0.0/attention")
}

/// The `--cache-dir` root. FileStorage lays its own `c1-xhtml/`,
/// `c2-preprocessor/`, `c3-graph/` tiers underneath. Marcus's steer:
/// reuse `snapshots/` as the local fixture cache dir (the stage-snapshot
/// `snapshots/<doc>/` contents live alongside, untouched).
fn cache_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_fixtures/snapshots")
}

fn golden_md_path() -> PathBuf {
    golden_dir().join("document.bgraph.md")
}

// =========================================================================
// The JVM-free replay.
// =========================================================================

/// Regenerate `attention`'s bgraph.md from the committed C2 cache, JVM-free.
///
/// `FreshFrom::C3` is the exact tier we want: it consults the C2 cache (a
/// hit → skips Tika *and* the XHTML parse) but does **not** read a C3 graph
/// cache, so the deterministic builder (`build_graph_deterministic`) runs on
/// every invocation. That is the reproduction path we are anchoring. (The
/// handoff prose says "FreshFrom::C2"; that variant means "reparse from C1
/// XHTML" — the opposite of a C2 hit. We honor the stated intent — "a C2 hit
/// skips Tika entirely" — with the variant that actually produces it.)
///
/// `CacheDefaults` with every write disabled keeps the run read-only: the
/// committed cache is never mutated and no stray tiers are written.
fn regenerate_attention() -> (DocumentGraph, ParseProvenance) {
    let golden = golden_dir();
    let pdf = golden.join("attention.pdf");
    let config_path = golden.join("config.yaml");

    let config = ParsingConfig::load_from_file(
        config_path
            .to_str()
            .expect("config.yaml path is valid UTF-8"),
    )
    .expect("golden config.yaml loads");

    let storage = FileStorage::new(cache_dir().to_str().expect("cache dir path is valid UTF-8"))
        .expect("FileStorage opens at the committed cache dir");

    let mut processor =
        DocumentProcessor::new_with_dependencies(Box::new(NoJvmPreprocessor), Box::new(storage))
            .expect("DocumentProcessor builds with the no-JVM stub preprocessor");

    // Read-only: consult caches, write nothing.
    let read_only = CacheDefaults {
        c0_pdf: false,
        c1_xhtml: false,
        c2_preprocessor: false,
        c3_graph: false,
        stat: false,
    };

    let (graph, provenance) = processor
        .process_document_with_cache(
            pdf.to_str().expect("pdf path is valid UTF-8"),
            &config,
            FreshFrom::C3,
            &read_only,
            false,
        )
        .expect("C2 replay builds the graph deterministically");

    // CR-86 / DT-12: the anchor is the **default null-style `1.0.0`
    // edition**. `style_info` is now an always-present, config-valued node
    // field, gated at *build* time: the golden `config.yaml` leaves
    // `include_style_info` off (the default), so the built graph carries
    // `style_info: None` on every node. `bgraph_sha256` covers that (`null`),
    // the emitter serializes it (`"style":null`), and a re-parse
    // reconstructs `None` → the recomputed hash matches → `Verified` on the
    // default path. No emit flag: the emitter serializes exactly what the
    // graph holds. (The provisional Block D workaround — freezing WITH
    // `--include-style-info` because the default emit didn't self-verify —
    // is exactly the bug CR-86 fixes; it is gone.)
    (graph, provenance)
}

/// The frozen md is `emit_markdown` over the same C2-replayed graph.
fn regenerate_bgraph_md() -> String {
    let (graph, provenance) = regenerate_attention();
    emit_markdown(&graph, &provenance)
}

/// `bragi-io` git HEAD sha — the codebase_sha binding recorded in the
/// `PRODUCED_BY` sidecar. Not part of any serialized artifact.
fn git_head_sha() -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("git rev-parse HEAD runs");
    assert!(out.status.success(), "git rev-parse HEAD failed");
    String::from_utf8(out.stdout)
        .expect("git sha is valid UTF-8")
        .trim()
        .to_string()
}

fn bless_enabled() -> bool {
    std::env::var_os("BLESS_GOLDEN").is_some()
}

/// Prepended to the frozen golden `config.yaml` when blessing, explaining why
/// it is a complete machine-materialized config rather than the annotated
/// source. (A YAML comment — ignored on load, so it does not affect
/// `config_hash`.)
const GOLDEN_CONFIG_HEADER: &str = "\
# GENERATED — the complete, materialized config for this golden edition.
# Every field is pinned explicitly so `config_hash` (hashed from the whole
# config and embedded in document.bgraph.md) is immune to code-default drift:
# moving a #[serde(default)] in code cannot silently re-key this frozen anchor.
# Rewritten on BLESS_GOLDEN=1; do not hand-edit. (CR-89.)
";

/// Freeze the **complete** resolved config next to the md at bless time: load
/// the current config, re-serialize it with every field explicit, and write it
/// back. Keeps the golden edition self-contained — its identity depends on the
/// committed file, not on any `#[serde(default)]` in code.
fn freeze_materialized_config() {
    let config_path = golden_dir().join("config.yaml");
    let config = ParsingConfig::load_from_file(
        config_path.to_str().expect("config.yaml path is valid UTF-8"),
    )
    .expect("golden config.yaml loads");
    let materialized = format!(
        "{GOLDEN_CONFIG_HEADER}{}",
        config.to_yaml().expect("config serializes to complete YAML")
    );
    std::fs::write(&config_path, materialized).expect("write materialized golden config.yaml");
}

// =========================================================================
// Test A — reproduction (freeze).
// =========================================================================

#[test]
fn golden_freeze_attention_reproduces_bgraph_md() {
    let regenerated = regenerate_bgraph_md();
    let path = golden_md_path();

    if bless_enabled() {
        std::fs::write(&path, &regenerated).expect("write frozen golden bgraph.md");
        freeze_materialized_config();
        let sha = git_head_sha();
        std::fs::write(golden_dir().join("PRODUCED_BY"), format!("{sha}\n"))
            .expect("write PRODUCED_BY sidecar");
        eprintln!(
            "✅ BLESS_GOLDEN: re-froze {} ({} bytes) + materialized config.yaml + PRODUCED_BY {}",
            path.display(),
            regenerated.len(),
            sha
        );
        return;
    }

    let frozen = std::fs::read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "Missing frozen golden bgraph.md at {}.\n\
             Generate it JVM-free from the committed C2 cache with:\n  \
             BLESS_GOLDEN=1 cargo test -p bragi-io-core --test golden_freeze_tests\n\
             or rebuild the whole family (needs the JVM) with `make golden-generate`.",
            path.display()
        )
    });

    if regenerated != frozen {
        let max = regenerated.len().min(frozen.len());
        let mut first_diff = max;
        for i in 0..max {
            if regenerated.as_bytes()[i] != frozen.as_bytes()[i] {
                first_diff = i;
                break;
            }
        }
        let start = first_diff.saturating_sub(80);
        let end_r = (first_diff + 120).min(regenerated.len());
        let end_f = (first_diff + 120).min(frozen.len());
        panic!(
            "Golden freeze mismatch: HEAD no longer reproduces attention's 1.0.0 bgraph.md.\n\
             First divergence at byte {first_diff} (regenerated={} bytes, frozen={} bytes).\n\
             --- frozen window ---\n{}\n\
             --- regenerated window ---\n{}\n\
             \n\
             If this change *legitimately* moves the output (e.g. a crate-version bump or an\n\
             intended pipeline change), re-freeze intentionally:\n  \
             BLESS_GOLDEN=1 cargo test -p bragi-io-core --test golden_freeze_tests\n\
             and commit the updated document.bgraph.md + PRODUCED_BY. Otherwise this is a\n\
             reproduction regression — investigate before blessing.",
            regenerated.len(),
            frozen.len(),
            &frozen[start..end_f],
            &regenerated[start..end_r],
        );
    }
}

// =========================================================================
// Test B — roundtrip (read-side agreement).
// =========================================================================

#[test]
fn golden_freeze_attention_roundtrips_verified() {
    let path = golden_md_path();

    // Under bless, Test A is concurrently rewriting `document.bgraph.md`.
    // Reading it here would race the writer — a torn read on a re-bless, since
    // `fs::write` isn't atomic, and the bootstrap-skip only fired when the file
    // was *absent*, not *stale*. Instead, roundtrip the freshly-regenerated
    // content in-memory: the same deterministic, read-only C2 replay Test A
    // blesses (safe to run concurrently, byte-identical output). This verifies
    // the *current* output self-verifies without touching the file being
    // written. Off-bless, we check the *committed* artifact self-verifies
    // (catches corruption / hand-edits of the frozen md).
    let md = if bless_enabled() {
        regenerate_bgraph_md()
    } else {
        std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!(
                "Missing frozen golden bgraph.md at {}: {e}.\n\
                 Generate it with `BLESS_GOLDEN=1 cargo test -p bragi-io-core \
                 --test golden_freeze_tests` or `make golden-generate`.",
                path.display()
            )
        })
    };

    let result = parse_markdown(&md, ParseOptions::default())
        .expect("golden bgraph.md parses cleanly");

    assert!(
        matches!(result.identity, ParseIdentity::Verified),
        "golden bgraph.md must self-verify (doc-level bgraph_sha256); got {:?}",
        result.identity
    );
}

// =========================================================================
// Light channels (DOCX + MD) — CR-91.
//
// The docx/md channels are pure-Rust (no Tika/JVM), so the freeze IS the full
// product path: parse `source.{docx,md}` through the same core lib the CLI/API
// call, emit, and assert byte-identity against the frozen `document.bgraph.md`.
// No C2 replay, no `NoJvmPreprocessor` stub, and no `PRODUCED_BY` anchor —
// there is nothing nondeterministic to pin (CR-91 §3).
//
// Filename note: `regenerate_light_bgraph_md` reproduces the CLI's current
// stamping — docx stamps the source basename, md leaves it empty. That
// asymmetry is what CR-92 deletes; until then it is frozen here as the current
// shape, and THIS guard is what makes that future re-bless safe.
// =========================================================================

#[derive(Clone, Copy, Debug)]
enum LightChannel {
    Md,
    Docx,
    Ocr,
}

impl LightChannel {
    fn dir_name(self) -> &'static str {
        match self {
            LightChannel::Md => "demo-md",
            LightChannel::Docx => "demo-docx",
            LightChannel::Ocr => "demo-ocr",
        }
    }

    fn source_name(self) -> &'static str {
        match self {
            LightChannel::Md => "source.md",
            LightChannel::Docx => "source.docx",
            LightChannel::Ocr => "source.json",
        }
    }
}

fn light_dir(ch: LightChannel) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_fixtures/golden/1.0.0")
        .join(ch.dir_name())
}

fn light_golden_md_path(ch: LightChannel) -> PathBuf {
    light_dir(ch).join("document.bgraph.md")
}

/// Parse the committed source through the core lib (JVM-free) — the same
/// parse the CLI/API run. Returns the graph + provenance so both the md and
/// json arms freeze from the identical in-memory graph. CR-92: the `source`
/// block is content-only (no filename), identical across the md/docx channels.
fn regenerate_light(ch: LightChannel) -> (DocumentGraph, ParseProvenance) {
    let source = light_dir(ch).join(ch.source_name());
    match ch {
        LightChannel::Md => {
            let content = std::fs::read_to_string(&source)
                .unwrap_or_else(|e| panic!("read {}: {e}", source.display()));
            let result =
                parse_markdown(&content, ParseOptions::default()).expect("demo-md source parses");
            (result.graph, result.provenance)
        }
        LightChannel::Docx => {
            let bytes =
                std::fs::read(&source).unwrap_or_else(|e| panic!("read {}: {e}", source.display()));
            let result =
                parse_docx(&bytes, ParseOptions::default()).expect("demo-docx source parses");
            (result.graph, result.provenance)
        }
        LightChannel::Ocr => {
            let bytes =
                std::fs::read(&source).unwrap_or_else(|e| panic!("read {}: {e}", source.display()));
            let result =
                parse_ocr(&bytes, ParseOptions::default()).expect("demo-ocr source parses");
            (result.graph, result.provenance)
        }
    }
}

/// The frozen md is `emit_markdown` over the freshly-parsed light graph.
fn regenerate_light_bgraph_md(ch: LightChannel) -> String {
    let (graph, provenance) = regenerate_light(ch);
    emit_markdown(&graph, &provenance)
}

/// Test A analog — reproduction (freeze): byte-identity against the frozen md,
/// or re-freeze under `BLESS_GOLDEN`.
fn check_light_reproduces(ch: LightChannel) {
    let regenerated = regenerate_light_bgraph_md(ch);
    let path = light_golden_md_path(ch);

    if bless_enabled() {
        std::fs::write(&path, &regenerated).expect("write frozen light golden bgraph.md");
        eprintln!(
            "✅ BLESS_GOLDEN: re-froze {} ({} bytes)",
            path.display(),
            regenerated.len()
        );
        return;
    }

    let frozen = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "Missing frozen golden bgraph.md at {}: {e}.\n\
             Generate it JVM-free with:\n  \
             BLESS_GOLDEN=1 cargo test -p bragi-io-core --test golden_freeze_tests",
            path.display()
        )
    });

    if regenerated != frozen {
        let max = regenerated.len().min(frozen.len());
        let first_diff = (0..max)
            .find(|&i| regenerated.as_bytes()[i] != frozen.as_bytes()[i])
            .unwrap_or(max);
        let start = first_diff.saturating_sub(80);
        let end_r = (first_diff + 120).min(regenerated.len());
        let end_f = (first_diff + 120).min(frozen.len());
        panic!(
            "Golden freeze mismatch ({}): HEAD no longer reproduces the frozen \
             document.bgraph.md.\n\
             First divergence at byte {first_diff} (regenerated={} bytes, frozen={} bytes).\n\
             --- frozen window ---\n{}\n--- regenerated window ---\n{}\n\
             \nIf this legitimately moves the output (a crate-version bump or intended \
             change), re-freeze:\n  \
             BLESS_GOLDEN=1 cargo test -p bragi-io-core --test golden_freeze_tests\n\
             and commit the updated document.bgraph.md.",
            ch.dir_name(),
            regenerated.len(),
            frozen.len(),
            &frozen[start..end_f],
            &regenerated[start..end_r],
        );
    }
}

/// Test B analog — the frozen md self-verifies (doc-level bgraph_sha256).
fn check_light_roundtrips(ch: LightChannel) {
    let path = light_golden_md_path(ch);
    // Same race-avoidance as attention's Test B: under bless, Test A is
    // concurrently rewriting the file, so roundtrip the freshly-regenerated
    // content in-memory rather than reading the file mid-write.
    let md = if bless_enabled() {
        regenerate_light_bgraph_md(ch)
    } else {
        std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Missing frozen golden bgraph.md at {}: {e}.", path.display()))
    };

    let result =
        parse_markdown(&md, ParseOptions::default()).expect("frozen light golden parses cleanly");

    assert!(
        matches!(result.identity, ParseIdentity::Verified),
        "frozen {} golden must self-verify (doc-level bgraph_sha256); got {:?}",
        ch.dir_name(),
        result.identity
    );
}

#[test]
fn golden_freeze_demo_md_reproduces_bgraph_md() {
    check_light_reproduces(LightChannel::Md);
}

#[test]
fn golden_freeze_demo_md_roundtrips_verified() {
    check_light_roundtrips(LightChannel::Md);
}

#[test]
fn golden_freeze_demo_docx_reproduces_bgraph_md() {
    check_light_reproduces(LightChannel::Docx);
}

#[test]
fn golden_freeze_demo_docx_roundtrips_verified() {
    check_light_roundtrips(LightChannel::Docx);
}

#[test]
fn golden_freeze_demo_ocr_reproduces_bgraph_md() {
    check_light_reproduces(LightChannel::Ocr);
}

#[test]
fn golden_freeze_demo_ocr_roundtrips_verified() {
    check_light_roundtrips(LightChannel::Ocr);
}

// =========================================================================
// OCR S2 — the grafted golden (demo-ocr + companion PDF).
//
// The premium path: `parse_ocr_with_pdf` over `demo-ocr/source.json` and the
// attention PDF — the real attention twin (one source PDF behind both the
// OCR payload and the native golden). Frozen as a SECOND pair in the
// demo-ocr family (`document.graft.bgraph.md` / `.json`); the single-arm
// pair above stays byte-identical (`companion_pdf_sha256` serializes
// absent-when-None, so the graft changes nothing it wasn't asked to).
//
// JVM-free by the attention pattern (committed-cache replay): the wrapper's
// PDF → XHTML hop replays Tika's *committed* C1 XHTML for the companion's
// sha (`snapshots/c1-xhtml/<sha256(pdf)>.xhtml` — the same cache tier the
// attention freeze replays at C2), so the full graft composition —
// parse_ocr → seam extraction → pure merge — runs for real with no JVM.
// The real-Tika lane for this seam is `make jvm-smoke` (which asserts a
// fresh JNI graft parse reproduces this frozen pair), mirroring how the
// attention golden splits hermetic replay from the JVM gate.
// =========================================================================

/// Replays the committed C1 XHTML for a PDF instead of invoking Tika.
/// Panics loudly if the committed tier is absent (the graft golden must
/// never silently depend on a live JVM) or if the body-side XHTML parse
/// is ever reached (the graft is metadata-only — no graph build).
struct C1XhtmlReplayPreprocessor;

impl Preprocessor for C1XhtmlReplayPreprocessor {
    fn parse_pdf_to_markup_language(&self, pdf_bytes: &[u8]) -> anyhow::Result<String> {
        let sha = bragi_io_core::preprocessors::ocr::companion_sha256(pdf_bytes);
        let path = cache_dir().join(format!("c1-xhtml/{sha}.xhtml"));
        Ok(std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!(
                "Committed C1 XHTML missing at {} ({e}) — the graft golden replays Tika's \
                 committed output and must never invoke a live JVM. Rebuild the attention \
                 family (which owns the C1 tier) with `make golden-generate`.",
                path.display()
            )
        }))
    }

    fn parse_markup_to_preprocessor_output(
        &self,
        _markup: &str,
    ) -> anyhow::Result<PreprocessorOutput> {
        panic!(
            "The body-side XHTML parse was invoked by the metadata graft — the graft is \
             extraction-only (no graph build) and must never reach this step."
        );
    }

    fn name(&self) -> &str {
        "c1-xhtml-replay-graft-stub"
    }

    fn supports_file_type(&self, _path: &Path) -> bool {
        true
    }
}

/// The companion PDF — same bytes as `golden/1.0.0/attention/attention.pdf`
/// (the twin pairing is the point: Tika's committed C1 XHTML is keyed by
/// this file's sha).
fn graft_companion_pdf_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_fixtures/pdfs/attention-is-all-you-need.pdf")
}

fn graft_golden_md_path() -> PathBuf {
    light_dir(LightChannel::Ocr).join("document.graft.bgraph.md")
}

fn graft_golden_json_path() -> PathBuf {
    light_dir(LightChannel::Ocr).join("document.graft.bgraph.json")
}

/// The grafted parse, JVM-free: the real `parse_ocr_with_pdf` composition
/// with the committed-C1 replay standing in for Tika.
fn regenerate_ocr_graft() -> (DocumentGraph, ParseProvenance) {
    let source = light_dir(LightChannel::Ocr).join(LightChannel::Ocr.source_name());
    let ocr_bytes =
        std::fs::read(&source).unwrap_or_else(|e| panic!("read {}: {e}", source.display()));
    let pdf_path = graft_companion_pdf_path();
    let pdf_bytes =
        std::fs::read(&pdf_path).unwrap_or_else(|e| panic!("read {}: {e}", pdf_path.display()));
    let result = bragi_io_core::preprocessors::ocr::parse_ocr_with_pdf(
        &ocr_bytes,
        &pdf_bytes,
        ParseOptions::default(),
        &C1XhtmlReplayPreprocessor,
    )
    .expect("grafted demo-ocr parse succeeds");
    (result.graph, result.provenance)
}

fn regenerate_ocr_graft_bgraph_md() -> String {
    let (graph, provenance) = regenerate_ocr_graft();
    emit_markdown(&graph, &provenance)
}

/// Test A analog for the grafted pair — byte-identity, bless-writable.
#[test]
fn golden_freeze_demo_ocr_graft_reproduces_bgraph_md() {
    let regenerated = regenerate_ocr_graft_bgraph_md();
    let path = graft_golden_md_path();

    if bless_enabled() {
        std::fs::write(&path, &regenerated).expect("write frozen graft golden bgraph.md");
        eprintln!(
            "✅ BLESS_GOLDEN: re-froze {} ({} bytes)",
            path.display(),
            regenerated.len()
        );
        return;
    }

    let frozen = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "Missing frozen graft golden bgraph.md at {}: {e}.\n\
             Generate it JVM-free with:\n  \
             BLESS_GOLDEN=1 cargo test -p bragi-io-core --test golden_freeze_tests",
            path.display()
        )
    });

    if regenerated != frozen {
        let max = regenerated.len().min(frozen.len());
        let first_diff = (0..max)
            .find(|&i| regenerated.as_bytes()[i] != frozen.as_bytes()[i])
            .unwrap_or(max);
        let start = first_diff.saturating_sub(80);
        let end_r = (first_diff + 120).min(regenerated.len());
        let end_f = (first_diff + 120).min(frozen.len());
        panic!(
            "Golden freeze mismatch (demo-ocr graft): HEAD no longer reproduces the frozen \
             document.graft.bgraph.md.\n\
             First divergence at byte {first_diff} (regenerated={} bytes, frozen={} bytes).\n\
             --- frozen window ---\n{}\n--- regenerated window ---\n{}\n\
             \nIf this legitimately moves the output, re-freeze:\n  \
             BLESS_GOLDEN=1 cargo test -p bragi-io-core --test golden_freeze_tests",
            regenerated.len(),
            frozen.len(),
            &frozen[start..end_f],
            &regenerated[start..end_r],
        );
    }
}

/// Test B analog — the frozen grafted md self-verifies.
#[test]
fn golden_freeze_demo_ocr_graft_roundtrips_verified() {
    let md = if bless_enabled() {
        regenerate_ocr_graft_bgraph_md()
    } else {
        let path = graft_golden_md_path();
        std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!("Missing frozen graft golden bgraph.md at {}: {e}.", path.display())
        })
    };
    let result =
        parse_markdown(&md, ParseOptions::default()).expect("frozen graft golden parses cleanly");
    assert!(
        matches!(result.identity, ParseIdentity::Verified),
        "frozen demo-ocr graft golden must self-verify; got {:?}",
        result.identity
    );
}

/// JSON wire for the grafted pair — same freeze/verify/parity/contract arms
/// as every other channel.
#[test]
fn golden_freeze_demo_ocr_graft_json_wire() {
    let (graph, provenance) = regenerate_ocr_graft();
    check_json_wire(&graph, &provenance, &graft_golden_json_path(), "demo-ocr-graft");
}

/// The S2 exit criterion, pinned on the frozen artifact: canonical fields
/// not all-null, both namespaces present, the companion linkage set — and
/// the single-arm invariants (provenance, `ocr:` run facts) untouched.
///
/// Attention's PDF container carries no `dc:title`, so the grafted title is
/// honestly `None` (the native golden's title is body-side inference, out
/// of the extraction-layer graft's reach); `created` carries the criterion.
#[test]
fn golden_freeze_demo_ocr_graft_metadata_reads_true() {
    let (graph, provenance) = regenerate_ocr_graft();
    let md = &graph.document_info.document_metadata;

    // Canonical: what the container knows.
    assert_eq!(md.created.as_deref(), Some("2024-04-10T21:11:43Z"));
    assert!(md.title.is_none(), "no dc:title in attention's container");
    assert!(md.author.is_none());

    // Both namespaces present.
    let pdf = md.pdf.as_ref().expect("pdf namespace grafted");
    assert_eq!(pdf.page_count, Some(15));
    assert_eq!(pdf.producer.as_deref(), Some("pdfTeX-1.40.25"));
    assert_eq!(pdf.creator_tool.as_deref(), Some("LaTeX with hyperref"));
    let ocr = md.ocr.as_ref().expect("ocr namespace kept");
    assert_eq!(ocr.model.as_deref(), Some("mistral-ocr-4-0"));
    assert_eq!(ocr.pages_processed, Some(15));

    // Companion linkage = sha256 of the companion PDF bytes.
    let pdf_bytes = std::fs::read(graft_companion_pdf_path()).expect("companion pdf");
    assert_eq!(
        ocr.companion_pdf_sha256.as_deref(),
        Some(bragi_io_core::preprocessors::ocr::companion_sha256(&pdf_bytes).as_str())
    );

    // Provenance unchanged — the PDF is a companion, not a second source.
    assert_eq!(provenance.source_format, "ocr");
    assert_eq!(provenance.config_hash, "none");
    let ocr_bytes = std::fs::read(light_dir(LightChannel::Ocr).join("source.json")).unwrap();
    assert_eq!(
        provenance.source_sha256,
        bragi_io_core::preprocessors::ocr::companion_sha256(&ocr_bytes),
        "source_sha256 stays the mist.json bytes"
    );
}

/// CR-98: the frozen attention outline reads TRUE — asserted, not
/// eyeballed. Chapters 1–7 at depth 1, `x.y` at 2, `x.y.z` at 3,
/// Abstract/References at 1; the leading document title keeps its S1
/// level (2, from Mistral's `##`); the unnumbered appendix lands at 1.
/// Pre-CR-98, OCR-4's per-page levels put same-rank chapters on
/// different depths (1–3 at level 1, 4–7 at level 2) — this pins the
/// normalized shape so a regression re-blesses loudly, not silently.
#[test]
fn golden_freeze_demo_ocr_outline_reads_true() {
    let md = if bless_enabled() {
        // Bless race-avoidance, same as check_light_roundtrips: Test A
        // may be rewriting the file concurrently.
        regenerate_light_bgraph_md(LightChannel::Ocr)
    } else {
        let path = light_golden_md_path(LightChannel::Ocr);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Missing frozen golden bgraph.md at {}: {e}.", path.display()))
    };
    let graph = parse_markdown(&md, ParseOptions::default())
        .expect("frozen demo-ocr golden parses")
        .graph;
    let outline = graph
        .document_info
        .outline_data
        .as_ref()
        .expect("demo-ocr golden carries outline_data");
    let got: Vec<(&str, u32)> = outline
        .sections
        .iter()
        .map(|s| (s.title.as_str(), s.level))
        .collect();
    let want: Vec<(&str, u32)> = vec![
        ("Attention Is All You Need", 1), // leading title — pinned to the top (CR-98 decision note)
        ("Abstract", 1),
        ("1 Introduction", 1),
        ("2 Background", 1),
        ("3 Model Architecture", 1),
        ("3.1 Encoder and Decoder Stacks", 2),
        ("3.2 Attention", 2),
        ("3.2.1 Scaled Dot-Product Attention", 3),
        ("3.2.2 Multi-Head Attention", 3),
        ("3.2.3 Applications of Attention in our Model", 3),
        ("3.3 Position-wise Feed-Forward Networks", 2),
        ("3.4 Embeddings and Softmax", 2),
        ("3.5 Positional Encoding", 2),
        ("4 Why Self-Attention", 1),
        ("5 Training", 1),
        ("5.1 Training Data and Batching", 2),
        ("5.2 Hardware and Schedule", 2),
        ("5.3 Optimizer", 2),
        ("5.4 Regularization", 2),
        ("6 Results", 1),
        ("6.1 Machine Translation", 2),
        ("6.2 Model Variations", 2),
        ("6.3 English Constituency Parsing", 2),
        ("7 Conclusion", 1),
        ("References", 1),
        ("Attention Visualizations", 1),
    ];
    assert_eq!(got, want, "frozen demo-ocr outline must read the true ranks");
}

// =========================================================================
// JSON wire — the customer-facing envelope (B6 / CR-85 item 7).
//
// bgraph.md is the human/git-friendly encoding; `graph.json`
// (`SortedDocumentGraph`) is the machine wire — what the API serves and the
// `pip install` SDK deserializes. It was frozen NOWHERE, so a json-shape drift
// had no core tripwire. These arms freeze it too, and — crucially — tie it to
// the md encoding so the two can't silently diverge. Per channel:
//
//   * **freeze** — byte-identity against a committed `document.bgraph.json`
//     (bless-writable, un-ignored via `test_fixtures/**/*.json`). The one
//     wall-clock field, `created_at`, is pinned to the epoch sentinel core
//     already uses for "no real emission time" (`default_created_at`), so the
//     bytes are deterministic. It is an envelope field (outside `bgraph_sha256`),
//     so pinning it moves bytes, never identity.
//   * **self-verify** — `verify_identity() == Verified`: the loaded json proves
//     it is untampered from its own embedded hash (CR-88's library capability,
//     now tested against the golden).
//   * **sha-parity** — the json envelope `bgraph_sha256` equals the content-body
//     hash the md encoding embeds for the same graph. This is the json↔md
//     honesty check: the two serializations must agree on identity.
//   * **round-trip** — serialize → `from_str` → `verify_identity()` still
//     `Verified` (exercises the deserialize read path end to end).
//   * **contract** — an explicit 1.0.0 wire contract (below): a breaking change
//     fails a named assertion, an additive one does not.
// =========================================================================

/// The epoch-0 sentinel — the same "no real emission time" value core stamps
/// via `default_created_at` (types.rs). Pinning `created_at` to it makes the
/// frozen json byte-deterministic without lying about an emission time.
fn epoch_sentinel() -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp(0, 0).expect("epoch is always valid")
}

/// Emit the deterministic golden json for a graph: the exact
/// `SortedDocumentGraph` the API serves and the SDK reads, with `created_at`
/// pinned so the bytes are frozen-stable.
fn emit_golden_json(graph: &DocumentGraph, provenance: &ParseProvenance) -> String {
    let mut sorted = graph.to_sorted_graph(Some(provenance));
    sorted.created_at = epoch_sentinel();
    serde_json::to_string_pretty(&sorted).expect("SortedDocumentGraph serializes to json")
}

fn golden_json_path() -> PathBuf {
    golden_dir().join("document.bgraph.json")
}

fn light_golden_json_path(ch: LightChannel) -> PathBuf {
    light_dir(ch).join("document.bgraph.json")
}

/// The explicit 1.0.0 wire contract for `SortedDocumentGraph` — the value-level
/// guarantees a consumer (API, SDK) may rely on, beyond what the Rust types
/// already enforce on deserialize (field presence + enum domains). A *breaking*
/// change (a dropped required field, a renamed field, a wrong version) returns
/// `Err` with a named reason; an *additive* change (a new optional field) does
/// NOT — serde drops unknowns on the way in, so this stays green. That gap is
/// the breaking-vs-non-breaking boundary the golden byte-freeze alone can't draw.
fn assert_schema_contract(g: &SortedDocumentGraph) -> Result<(), String> {
    let expected = FormatVersion::CURRENT.schema_str();
    if g.schema_version != expected {
        return Err(format!(
            "schema_version: expected {expected:?}, got {:?}",
            g.schema_version
        ));
    }
    if g.bgraph_sha256.len() != 64 || !g.bgraph_sha256.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!(
            "bgraph_sha256 must be 64 hex chars, got {:?} ({} chars)",
            g.bgraph_sha256,
            g.bgraph_sha256.len()
        ));
    }
    if g.nodes.is_empty() {
        return Err("nodes must be non-empty".to_string());
    }
    let root_id = g.document_info.root_id;
    if !g.nodes.iter().any(|n| n.id == root_id) {
        return Err(format!(
            "document_info.root_id {root_id} does not resolve to any node"
        ));
    }
    for n in &g.nodes {
        if n.node_type.is_empty() {
            return Err(format!("node {} has an empty node_type", n.id));
        }
    }
    match &g.parse_provenance {
        None => return Err("parse_provenance must be present on an emitted graph".to_string()),
        Some(p) => {
            if p.source_format.is_empty() {
                return Err("parse_provenance.source_format must be non-empty".to_string());
            }
            if p.source_sha256.len() != 64 {
                return Err(format!(
                    "parse_provenance.source_sha256 must be 64 hex chars, got {} chars",
                    p.source_sha256.len()
                ));
            }
        }
    }
    Ok(())
}

/// Freeze + verify one channel's json wire (see the section header).
fn check_json_wire(graph: &DocumentGraph, provenance: &ParseProvenance, path: &Path, label: &str) {
    let regenerated = emit_golden_json(graph, provenance);

    if bless_enabled() {
        std::fs::write(path, &regenerated).expect("write frozen golden json");
        eprintln!(
            "✅ BLESS_GOLDEN: re-froze {} ({} bytes)",
            path.display(),
            regenerated.len()
        );
    } else {
        let frozen = std::fs::read_to_string(path).unwrap_or_else(|e| {
            panic!(
                "Missing frozen golden json at {}: {e}.\n\
                 Generate it with:\n  \
                 BLESS_GOLDEN=1 cargo test -p bragi-io-core --test golden_freeze_tests",
                path.display()
            )
        });
        if regenerated != frozen {
            let max = regenerated.len().min(frozen.len());
            let first_diff = (0..max)
                .find(|&i| regenerated.as_bytes()[i] != frozen.as_bytes()[i])
                .unwrap_or(max);
            let start = first_diff.saturating_sub(80);
            let end_r = (first_diff + 120).min(regenerated.len());
            let end_f = (first_diff + 120).min(frozen.len());
            panic!(
                "Golden json wire mismatch ({label}): HEAD no longer reproduces the frozen \
                 document.bgraph.json.\n\
                 First divergence at byte {first_diff} (regenerated={} bytes, frozen={} bytes).\n\
                 --- frozen window ---\n{}\n--- regenerated window ---\n{}\n\
                 \nIf this legitimately moves the json wire (a schema change or version bump), \
                 re-freeze:\n  \
                 BLESS_GOLDEN=1 cargo test -p bragi-io-core --test golden_freeze_tests\n\
                 and commit the updated document.bgraph.json. Otherwise investigate — the json \
                 wire drifted from the frozen 1.0.0 shape.",
                regenerated.len(),
                frozen.len(),
                &frozen[start..end_f],
                &regenerated[start..end_r],
            );
        }
    }

    // Deserialize the emitted json (round-trip read path) and assert the wire
    // properties on the reconstructed wrapper.
    let sorted: SortedDocumentGraph = serde_json::from_str(&regenerated)
        .unwrap_or_else(|e| panic!("golden json ({label}) must deserialize: {e}"));

    assert!(
        matches!(sorted.verify_identity(), ParseIdentity::Verified),
        "golden json ({label}) must self-verify (envelope bgraph_sha256); got {:?}",
        sorted.verify_identity()
    );

    let content_hash = bgraph_sha256(graph);
    assert_eq!(
        sorted.bgraph_sha256, content_hash,
        "golden json ({label}) envelope bgraph_sha256 disagrees with the md/content-body hash — \
         the json and md encodings have diverged on identity"
    );

    assert_schema_contract(&sorted)
        .unwrap_or_else(|v| panic!("golden json ({label}) violates the 1.0.0 schema contract: {v}"));
}

#[test]
fn golden_freeze_attention_json_wire() {
    let (graph, provenance) = regenerate_attention();
    check_json_wire(&graph, &provenance, &golden_json_path(), "attention");
}

#[test]
fn golden_freeze_demo_md_json_wire() {
    let (graph, provenance) = regenerate_light(LightChannel::Md);
    check_json_wire(
        &graph,
        &provenance,
        &light_golden_json_path(LightChannel::Md),
        "demo-md",
    );
}

#[test]
fn golden_freeze_demo_docx_json_wire() {
    let (graph, provenance) = regenerate_light(LightChannel::Docx);
    check_json_wire(
        &graph,
        &provenance,
        &light_golden_json_path(LightChannel::Docx),
        "demo-docx",
    );
}

#[test]
fn golden_freeze_demo_ocr_json_wire() {
    let (graph, provenance) = regenerate_light(LightChannel::Ocr);
    check_json_wire(
        &graph,
        &provenance,
        &light_golden_json_path(LightChannel::Ocr),
        "demo-ocr",
    );
}

// =========================================================================
// Boundary proof — the design-flow's literal acceptance test: a deliberate
// BREAKING schema change fails a fixture assertion; a NON-BREAKING (additive)
// one does not. Encodes the accept/reject matrix so the contract's boundary is
// *proven*, not merely asserted. Operates on the attention golden json.
// =========================================================================

/// The attention golden json as a mutable `serde_json::Value` to tamper with.
fn attention_json_value() -> serde_json::Value {
    let (graph, provenance) = regenerate_attention();
    serde_json::from_str(&emit_golden_json(&graph, &provenance))
        .expect("attention golden json parses to Value")
}

/// Deserialize a (possibly tampered) Value and run the contract, mirroring the
/// consumer read path.
fn contract_of(v: &serde_json::Value) -> Result<(), String> {
    let sorted: SortedDocumentGraph =
        serde_json::from_value(v.clone()).map_err(|e| format!("deserialize failed: {e}"))?;
    assert_schema_contract(&sorted)
}

#[test]
fn boundary_clean_golden_satisfies_contract() {
    assert!(
        contract_of(&attention_json_value()).is_ok(),
        "the clean golden json must satisfy the contract"
    );
}

#[test]
fn boundary_breaking_version_drift_is_caught() {
    let mut v = attention_json_value();
    v["schema_version"] = serde_json::json!("2.0.0");
    let err = contract_of(&v).expect_err("a schema_version bump must fail the contract");
    assert!(
        err.contains("schema_version"),
        "the failure must name schema_version; got: {err}"
    );
}

#[test]
fn boundary_breaking_dropped_content_is_caught() {
    // Required content removed — the nodes array emptied.
    let mut v = attention_json_value();
    v["nodes"] = serde_json::json!([]);
    let err = contract_of(&v).expect_err("an emptied node set must fail the contract");
    assert!(
        err.contains("nodes"),
        "the failure must name nodes; got: {err}"
    );
}

#[test]
fn boundary_breaking_enum_out_of_domain_is_rejected_by_the_type_layer() {
    // An out-of-domain enum value is rejected at deserialize (the type layer) —
    // a loud failure at the boundary, before the contract fn even runs.
    let mut v = attention_json_value();
    v["document_info"]["flow_type"] = serde_json::json!("Sideways");
    let err = contract_of(&v).expect_err("an out-of-domain flow_type must be rejected");
    assert!(
        err.contains("deserialize failed"),
        "the enum-domain break must fail at deserialize; got: {err}"
    );
}

#[test]
fn boundary_breaking_renamed_required_field_is_rejected_by_the_type_layer() {
    // A *renamed* required field reads as a missing one. `bragi_version` carries
    // no `#[serde(default)]`, so the break is loud at deserialize — the same
    // class of change T1.5b R2 made when the producer-version field took its
    // current name. Renaming the key must never read as "absent and fine".
    let mut v = attention_json_value();
    let prov = v["parse_provenance"]
        .as_object_mut()
        .expect("parse_provenance is a json object");
    let carried = prov
        .remove("bragi_version")
        .expect("the clean golden carries bragi_version");
    prov.insert("renamed_version_field".to_string(), carried);
    let err = contract_of(&v).expect_err("a renamed required field must be rejected");
    assert!(
        err.contains("deserialize failed"),
        "a renamed required field must fail at deserialize; got: {err}"
    );
}

#[test]
fn boundary_breaking_renamed_defaulted_field_is_caught_by_the_contract() {
    // The asymmetric case, and the reason this contract exists. `bgraph_sha256`
    // *does* carry `#[serde(default)]` (to keep pre-Block-C fixtures loadable),
    // so renaming it does NOT fail at deserialize — it silently defaults to the
    // empty string, and `verify_identity` reads empty as "no embedded hash to
    // check against" rather than as a failure. The type layer cannot catch this
    // one; only the contract's shape check stands between a renamed identity
    // field and a silent pass.
    let mut v = attention_json_value();
    let obj = v.as_object_mut().expect("envelope is a json object");
    let carried = obj
        .remove("bgraph_sha256")
        .expect("the clean golden carries bgraph_sha256");
    obj.insert("renamed_digest_field".to_string(), carried);

    // Precondition: the type layer lets this through, so the assertion below is
    // really testing the contract and not serde.
    let sorted: SortedDocumentGraph = serde_json::from_value(v.clone())
        .expect("a renamed defaulted field still deserializes — that is the hazard");
    assert!(
        matches!(sorted.verify_identity(), ParseIdentity::Verified),
        "identity verification cannot see the loss either — it treats the \
         defaulted empty hash as 'nothing to check'"
    );

    let err = contract_of(&v).expect_err("a renamed defaulted field must fail the contract");
    assert!(
        err.contains("bgraph_sha256"),
        "the failure must name bgraph_sha256; got: {err}"
    );
}

#[test]
fn boundary_additive_optional_field_is_non_breaking() {
    // A new, unknown envelope field is additive — serde drops it on read, so the
    // wire stays backward-compatible. Both deserialize and the contract pass, and
    // identity is untouched.
    let mut v = attention_json_value();
    v.as_object_mut()
        .expect("envelope is a json object")
        .insert("future_field_v2".to_string(), serde_json::json!({"anything": 123}));
    assert!(
        contract_of(&v).is_ok(),
        "an additive optional field must remain non-breaking (contract stays green)"
    );
    let sorted: SortedDocumentGraph =
        serde_json::from_value(v).expect("additive field still deserializes");
    assert!(
        matches!(sorted.verify_identity(), ParseIdentity::Verified),
        "an additive envelope field must not affect identity"
    );
}
