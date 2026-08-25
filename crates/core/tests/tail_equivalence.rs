//! CR-97 WP1 — corpus equivalence harness for the threshold tail.
//!
//! Proves (or refutes) the capture-point hypothesis locked in the CR-97
//! implementation flow (decision 1):
//!
//! > `apply_threshold_tail(P_0, k)` ≡ a full pipeline run at
//! > `min_confidence = k`, byte-identical `bgraph_sha256`.
//!
//! Per doc, three graphs are computed **JVM-free** from a C2
//! preprocessor cache (the golden-freeze replay pattern —
//! `FreshFrom::C3` + a panicking preprocessor stub + write-disabled
//! `CacheDefaults`):
//!
//!   1. `full_mc0` — `process_document_with_cache` at the baseline
//!      config (the real pipeline entry point; post `style_info` gate).
//!   2. `full_mck` — the same entry point with
//!      `graph_sanity.invariants.min_confidence = k` (the ladder rung).
//!   3. `tail` — a hand-mirrored baseline pipeline run (the API's
//!      `processing.rs` mirror shape) captures the pre-strip `P_0` plus
//!      the CR-78 confidence sidecar, then `apply_threshold_tail(P_0,
//!      k)` and the CR-86 `style_info` gate.
//!
//! The mirror's own fidelity is controlled per doc: stripping the
//! mirrored `P_0` must reproduce `full_mc0`'s hash exactly
//! (`mirror_ok`). The equivalence verdict is `sha(tail) == sha(full_mck)`.
//! A repeated tail per doc re-checks the CR-84 determinism obligation on
//! real graphs.
//!
//! `#[ignore]`d: this walks a corpus + cache that live outside the repo.
//! Driven by lab experiment `2026-08-24-tail-equivalence` via env vars:
//!
//!   BRAGI_TAIL_EQUIV_SPEC       spec file, lines: `<name>\t<pdf_path>`
//!   BRAGI_TAIL_EQUIV_CACHE_DIR  cache root containing `c2-preprocessor/`
//!   BRAGI_TAIL_EQUIV_CONFIG     baseline (mc0) config yaml
//!   BRAGI_TAIL_EQUIV_MIN_CONF   the rung k (default 4)
//!   BRAGI_TAIL_EQUIV_OUT        per-doc CSV report path (written even on
//!                               refutation — the experiment's raw table)

use bragi_io_core::analytics::{tag_and_resort, AnalysisBuilder, DocumentAnalysis};
use bragi_io_core::classifier::DocumentClassifier;
use bragi_io_core::config::ParsingConfig;
use bragi_io_core::graphs::builder::{rekey_node_ids, GraphBuilder};
use bragi_io_core::graphs::serialization::canonical::bgraph_sha256;
use bragi_io_core::graphs::{apply_threshold_tail, graph_sanity, NodeIdGenerator};
use bragi_io_core::preprocessors::pdf::project_to_semantic_tree;
use bragi_io_core::preprocessors::Preprocessor;
use bragi_io_core::processor::DocumentProcessor;
use bragi_io_core::rules::RuleEngine;
use bragi_io_core::storage::{
    calculate_source_hash, CacheDefaults, DocumentStorage, FileStorage, FreshFrom,
};
use bragi_io_core::types::{infer_title, DocumentGraph, PdfTextElement, PreprocessorOutput};
use std::collections::HashMap;
use std::path::Path;

// =========================================================================
// JVM-free guard (golden-freeze pattern): every doc must replay from the
// C2 cache; reaching Tika or the XHTML parse is a loud failure.
// =========================================================================

struct NoJvmPreprocessor;

impl Preprocessor for NoJvmPreprocessor {
    fn parse_pdf_to_markup_language(&self, _pdf_bytes: &[u8]) -> anyhow::Result<String> {
        panic!(
            "Tika/JVM extraction invoked — the equivalence harness must replay from the C2 \
             preprocessor cache (missing/mismatched entry for this doc?)"
        );
    }

    fn parse_markup_to_preprocessor_output(
        &self,
        _markup: &str,
    ) -> anyhow::Result<PreprocessorOutput> {
        panic!("XHTML parse invoked — the equivalence harness must replay from the C2 cache");
    }

    fn name(&self) -> &str {
        "no-jvm-tail-equivalence-stub"
    }

    fn supports_file_type(&self, _path: &Path) -> bool {
        true
    }
}

fn read_only() -> CacheDefaults {
    CacheDefaults {
        c0_pdf: false,
        c1_xhtml: false,
        c2_preprocessor: false,
        c3_graph: false,
        stat: false,
    }
}

/// The real pipeline entry point, C2-replayed. Returns the finished
/// (post-`style_info`-gate) graph.
fn full_run(pdf_path: &str, config: &ParsingConfig, cache_dir: &str) -> DocumentGraph {
    let storage = FileStorage::new(cache_dir).expect("FileStorage opens at the cache dir");
    let mut processor =
        DocumentProcessor::new_with_dependencies(Box::new(NoJvmPreprocessor), Box::new(storage))
            .expect("DocumentProcessor builds with the no-JVM stub");
    let (graph, _provenance) = processor
        .process_document_with_cache(pdf_path, config, FreshFrom::C3, &read_only(), false)
        .expect("C2 replay builds the graph");
    graph
}

fn run_analytics(text_elements: &[PdfTextElement]) -> DocumentAnalysis {
    let mut builder = AnalysisBuilder::new();
    for element in text_elements {
        builder.observe(element);
    }
    builder.finalize()
}

/// Hand-mirrored baseline pipeline (the API `processing.rs` shape / core
/// `rules_and_graph`), stopping at the CR-97 capture point: after
/// `apply()` + breadcrumbs + re-key, **before** the CR-86 `style_info`
/// gate. Returns the pre-strip `P_0` and the CR-78 confidence sidecar.
fn mirrored_p0(
    preprocessor_output: &PreprocessorOutput,
    config: &ParsingConfig,
) -> (DocumentGraph, HashMap<u32, u8>) {
    let classifier = DocumentClassifier::new();
    let classification = classifier
        .classify(preprocessor_output)
        .expect("classification");

    let document_analysis = run_analytics(&preprocessor_output.text_elements);
    let resorted = tag_and_resort(
        preprocessor_output.text_elements.clone(),
        &document_analysis,
    );

    let rule_engine = RuleEngine::new().expect("rule engine");
    let font_size_analysis =
        rule_engine.analyze_font_sizes(&resorted, &preprocessor_output.style_data);
    let parsed_elements = rule_engine
        .apply_rules_with_config(
            &resorted,
            &classification,
            &document_analysis,
            &font_size_analysis,
            &preprocessor_output.style_data,
            config,
        )
        .expect("rules");

    let inferred_title = infer_title(&parsed_elements);
    let semantic_elements = project_to_semantic_tree(parsed_elements);
    let section_confidence: HashMap<u32, u8> = semantic_elements
        .iter()
        .filter(|e| e.confidence > 0)
        .map(|e| (e.text_order, e.confidence))
        .collect();

    let id_gen = NodeIdGenerator::new();
    let graph_builder = GraphBuilder::new();
    let mut graph = graph_builder
        .build_graph_deterministic(semantic_elements, &id_gen)
        .expect("graph build");

    graph.document_info.document_metadata = preprocessor_output.metadata.clone();
    if graph.document_info.document_metadata.title.is_none() {
        if let Some(title) = inferred_title {
            graph.document_info.document_metadata.title = Some(title);
        }
    }
    graph.document_info.outline_data = preprocessor_output.bookmark_data.clone();

    // Provenance is only consumed for the CR-71A evidence-artifact dump
    // (debug); `None` matches the provenance-free replay path.
    graph_sanity::apply(&mut graph, &config.graph_sanity, None, Some(&section_confidence));
    graph.compute_breadcrumbs();
    rekey_node_ids(&mut graph);

    (graph, section_confidence)
}

/// CR-86 / DT-12 gate, exactly as the pipeline applies it to the graph
/// that leaves the builder.
fn style_gate(mut graph: DocumentGraph, config: &ParsingConfig) -> DocumentGraph {
    if !config.include_style_info {
        for node in graph.nodes.values_mut() {
            node.style_info = None;
        }
    }
    graph
}

struct Row {
    name: String,
    full_mc0: String,
    full_mck: String,
    tail: String,
    mirror_ok: bool,
    tail_deterministic: bool,
    equal: bool,
}

#[test]
#[ignore = "corpus harness — driven by lab experiment 2026-08-24-tail-equivalence via env vars"]
fn corpus_tail_equivalence() {
    let spec_path = std::env::var("BRAGI_TAIL_EQUIV_SPEC").expect("BRAGI_TAIL_EQUIV_SPEC");
    let cache_dir = std::env::var("BRAGI_TAIL_EQUIV_CACHE_DIR").expect("BRAGI_TAIL_EQUIV_CACHE_DIR");
    let config_path = std::env::var("BRAGI_TAIL_EQUIV_CONFIG").expect("BRAGI_TAIL_EQUIV_CONFIG");
    let min_conf: u8 = std::env::var("BRAGI_TAIL_EQUIV_MIN_CONF")
        .unwrap_or_else(|_| "4".to_string())
        .parse()
        .expect("BRAGI_TAIL_EQUIV_MIN_CONF parses as u8");
    let out_path = std::env::var("BRAGI_TAIL_EQUIV_OUT").expect("BRAGI_TAIL_EQUIV_OUT");

    // Baseline (mc0) config — hard-fail on a parse problem rather than
    // silently falling back to defaults (the lab's known divergence trap).
    let config_mc0 = ParsingConfig::load_from_file(&config_path).expect("baseline config loads");
    assert_eq!(
        config_mc0.graph_sanity.invariants.min_confidence, 0,
        "the baseline config must be the mc0 rung"
    );
    // The rung config: baseline + min_confidence = k (the same struct the
    // CLI would load from a surgically-edited yaml).
    let mut config_mck = config_mc0.clone();
    config_mck.graph_sanity.invariants.min_confidence = min_conf;

    let spec = std::fs::read_to_string(&spec_path).expect("spec file reads");
    let docs: Vec<(String, String)> = spec
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|l| {
            let (name, pdf) = l.split_once('\t').expect("spec line is `name\\tpdf_path`");
            (name.to_string(), pdf.to_string())
        })
        .collect();
    assert!(!docs.is_empty(), "spec file lists no docs");

    let storage = FileStorage::new(&cache_dir).expect("FileStorage opens at the cache dir");
    let mut rows: Vec<Row> = Vec::with_capacity(docs.len());

    for (name, pdf_path) in &docs {
        println!("── {name} ──");
        let pdf_bytes = std::fs::read(pdf_path).expect("pdf reads");
        let pdf_hash = calculate_source_hash(&pdf_bytes);
        let preprocessor_output = storage
            .get_preprocessor_output(&pdf_hash)
            .expect("C2 read")
            .unwrap_or_else(|| panic!("no C2 cache entry for {name} ({pdf_hash})"));

        // (1) + (2): the real pipeline at both rungs.
        let full_mc0 = bgraph_sha256(&full_run(pdf_path, &config_mc0, &cache_dir));
        let full_mck = bgraph_sha256(&full_run(pdf_path, &config_mck, &cache_dir));

        // (3): mirrored P_0 capture → tail → style gate.
        let (p0, sidecar) = mirrored_p0(&preprocessor_output, &config_mc0);
        let mirror_sha = bgraph_sha256(&style_gate(p0.clone(), &config_mc0));
        let mirror_ok = mirror_sha == full_mc0;

        let tail_graph =
            apply_threshold_tail(&p0, min_conf, &sidecar, &config_mc0.graph_sanity);
        let tail_repeat =
            apply_threshold_tail(&p0, min_conf, &sidecar, &config_mc0.graph_sanity);
        let tail = bgraph_sha256(&style_gate(tail_graph, &config_mc0));
        let tail_deterministic = tail == bgraph_sha256(&style_gate(tail_repeat, &config_mc0));

        let equal = tail == full_mck;
        println!(
            "{name}: mirror_ok={mirror_ok} tail_deterministic={tail_deterministic} equal={equal}"
        );
        rows.push(Row {
            name: name.clone(),
            full_mc0,
            full_mck,
            tail,
            mirror_ok,
            tail_deterministic,
            equal,
        });
    }

    // Write the raw per-doc table BEFORE asserting — a refutation must
    // still produce the experiment's data.
    let mut csv = String::from(
        "doc,full_mc0_sha,full_mck_sha,tail_sha,mirror_ok,tail_deterministic,equal\n",
    );
    for r in &rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            r.name, r.full_mc0, r.full_mck, r.tail, r.mirror_ok, r.tail_deterministic, r.equal
        ));
    }
    std::fs::write(&out_path, csv).expect("report CSV writes");

    let bad_mirror: Vec<&str> = rows
        .iter()
        .filter(|r| !r.mirror_ok)
        .map(|r| r.name.as_str())
        .collect();
    let nondet: Vec<&str> = rows
        .iter()
        .filter(|r| !r.tail_deterministic)
        .map(|r| r.name.as_str())
        .collect();
    let diverged: Vec<&str> = rows
        .iter()
        .filter(|r| !r.equal)
        .map(|r| r.name.as_str())
        .collect();

    assert!(
        bad_mirror.is_empty(),
        "mirror fidelity broken (mirrored P_0 != full mc0 run) on: {bad_mirror:?}"
    );
    assert!(
        nondet.is_empty(),
        "repeated tails were not byte-identical on: {nondet:?}"
    );
    assert!(
        diverged.is_empty(),
        "EQUIVALENCE REFUTED — tail(P_0, {min_conf}) != full run at mc{min_conf} on: {diverged:?}"
    );
}
