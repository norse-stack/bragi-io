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
//!     identity verdict is `Verified` (doc-level `graph_sha256`
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
use bragi_io_core::graphs::serialization::canonical::graph_sha256;
use bragi_io_core::graphs::serialization::markdown::emit_markdown;
use bragi_io_core::graphs::serialization::version::FormatVersion;
use bragi_io_core::preprocessors::docx::parse_docx;
use bragi_io_core::preprocessors::md::{parse_markdown, ParseIdentity, ParseOptions};
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
    // `style_info: None` on every node. `graph_sha256` covers that (`null`),
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

/// `blazegraph-io` git HEAD sha — the codebase_sha binding recorded in the
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
        "golden bgraph.md must self-verify (doc-level graph_sha256); got {:?}",
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
}

impl LightChannel {
    fn dir_name(self) -> &'static str {
        match self {
            LightChannel::Md => "demo-md",
            LightChannel::Docx => "demo-docx",
        }
    }

    fn source_name(self) -> &'static str {
        match self {
            LightChannel::Md => "source.md",
            LightChannel::Docx => "source.docx",
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

/// Test B analog — the frozen md self-verifies (doc-level graph_sha256).
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
        "frozen {} golden must self-verify (doc-level graph_sha256); got {:?}",
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
//     bytes are deterministic. It is an envelope field (outside `graph_sha256`),
//     so pinning it moves bytes, never identity.
//   * **self-verify** — `verify_identity() == Verified`: the loaded json proves
//     it is untampered from its own embedded hash (CR-88's library capability,
//     now tested against the golden).
//   * **sha-parity** — the json envelope `graph_sha256` equals the content-body
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
    if g.graph_sha256.len() != 64 || !g.graph_sha256.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!(
            "graph_sha256 must be 64 hex chars, got {:?} ({} chars)",
            g.graph_sha256,
            g.graph_sha256.len()
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
        "golden json ({label}) must self-verify (envelope graph_sha256); got {:?}",
        sorted.verify_identity()
    );

    let content_hash = graph_sha256(graph);
    assert_eq!(
        sorted.graph_sha256, content_hash,
        "golden json ({label}) envelope graph_sha256 disagrees with the md/content-body hash — \
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
