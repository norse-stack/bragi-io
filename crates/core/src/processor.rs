use crate::analytics::{
    AnalysisBuilder, DocumentAnalysis, FontStatsBuilder, GeometryStatsBuilder, PageStatsBuilder,
    RegionStatsBuilder, Statistic,
};
use crate::cache::GraphCacheKey;
use crate::classifier::DocumentClassifier;
use crate::config::ParsingConfig;
use crate::graphs::builder::GraphBuilder;
use crate::graphs::NodeIdGenerator;
use crate::preprocessors::pdf::project_to_semantic_tree;
use crate::preprocessors::{Preprocessor, TikaPreprocessor};
use crate::rules::RuleEngine;
use crate::storage::{
    calculate_config_hash, calculate_source_hash, CacheDefaults, CachePoint, DocumentStorage,
    FileStorage, FreshFrom,
};
use crate::types::*;
use anyhow::Result;
use std::time::{Duration, Instant};
use tracing::{debug, info};

/// Simple profiler that collects timings for pipeline steps
pub struct StepProfiler {
    enabled: bool,
    timings: Vec<(String, Duration)>,
}

impl StepProfiler {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            timings: Vec::new(),
        }
    }

    pub fn time_step<F, R>(&mut self, step_name: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        if !self.enabled {
            return f();
        }

        let start = Instant::now();
        let result = f();
        let elapsed = start.elapsed();

        self.timings.push((step_name.to_string(), elapsed));
        debug!(
            target: PROFILE_TARGET,
            step = step_name,
            duration_ms = elapsed.as_millis() as u64,
            "step timed"
        );

        result
    }

    /// Emit the timing table: one `info` event per step, then the total, on
    /// the `bragi_io_core::profile` target. Only called when profiling was
    /// requested, so hosts can let that target through unconditionally.
    pub fn print_summary(&self) {
        if !self.enabled || self.timings.is_empty() {
            return;
        }

        let total: Duration = self.timings.iter().map(|(_, d)| *d).sum();

        for (step, duration) in &self.timings {
            let percentage = (duration.as_secs_f64() / total.as_secs_f64()) * 100.0;
            info!(
                target: PROFILE_TARGET,
                "{:.<35} {:>6}ms ({:.1}%)",
                step,
                duration.as_millis(),
                percentage
            );
        }
        info!(
            target: PROFILE_TARGET,
            "{:.<35} {:>6}ms",
            "Total",
            total.as_millis()
        );
    }
}

/// Tracing target for the `--profile` step timings.
pub const PROFILE_TARGET: &str = "bragi_io_core::profile";

pub struct DocumentProcessor {
    preprocessor: Box<dyn Preprocessor>,
    storage: Box<dyn DocumentStorage + Send + Sync>,
    classifier: DocumentClassifier,
    rule_engine: RuleEngine,
    graph_builder: GraphBuilder,
}

impl DocumentProcessor {
    /// Create DocumentProcessor with full dependency injection
    pub fn new_with_dependencies(
        preprocessor: Box<dyn Preprocessor>,
        storage: Box<dyn DocumentStorage + Send + Sync>,
    ) -> Result<Self> {
        Ok(Self {
            preprocessor,
            storage,
            classifier: DocumentClassifier::new(),
            rule_engine: RuleEngine::new()?,
            graph_builder: GraphBuilder::new(),
        })
    }

    /// Convenience constructor for CLI usage with JNI backend (cross-platform)
    #[cfg(feature = "jni-backend")]
    pub fn new_cli_jni(jre_path: &std::path::Path, jar_path: &std::path::Path) -> Result<Self> {
        let preprocessor = Box::new(TikaPreprocessor::new_with_jni(jre_path, jar_path)?);
        let storage = Box::new(FileStorage::new("cache")?);
        Self::new_with_dependencies(preprocessor, storage)
    }

    /// Convenience constructor for CLI with JNI backend and custom cache directory
    #[cfg(feature = "jni-backend")]
    pub fn new_cli_jni_with_cache(
        jre_path: &std::path::Path,
        jar_path: &std::path::Path,
        cache_dir: &str,
    ) -> Result<Self> {
        let preprocessor = Box::new(TikaPreprocessor::new_with_jni(jre_path, jar_path)?);
        let storage = Box::new(FileStorage::new(cache_dir)?);
        Self::new_with_dependencies(preprocessor, storage)
    }

    // =========================================================================
    // Main entry points
    // =========================================================================

    /// Process document with cache point awareness (CR-11).
    /// This is the primary entry point for CLI usage.
    ///
    /// Returns the graph together with the `ParseProvenance` for this
    /// parse run (Block A / Amendment M): provenance is derived at the
    /// entry point from the source bytes + config — independent of any
    /// cache hit — and rides beside the graph as an explicit value.
    /// `DocumentGraph` itself carries content only.
    pub fn process_document_with_cache(
        &mut self,
        input_path: &str,
        config: &ParsingConfig,
        fresh_from: FreshFrom,
        cache_defaults: &CacheDefaults,
        enable_profiling: bool,
    ) -> Result<(DocumentGraph, ParseProvenance)> {
        let mut profiler = StepProfiler::new(enable_profiling);
        let start_time = Instant::now();

        // Read PDF and calculate hash
        let pdf_bytes = std::fs::read(input_path)?;
        let pdf_hash = calculate_source_hash(&pdf_bytes);

        info!(bytes = pdf_bytes.len(), "pdf parse started");
        debug!(path = input_path, "pdf input");

        // Build the provenance record that identifies this parse run.
        // CR-83: `(source_sha256, config_hash)` no longer feed node-ID
        // derivation — node IDs are content+breadcrumb-derived, not
        // document-namespace-scoped. These fields remain *document*
        // discriminators recorded in the doc-level envelope fence and the
        // graph.json wrapper. Block A: provenance is NOT stamped on the
        // graph (it is not content, so it must not feed `bgraph_sha256`);
        // it is constructed here — before any cache check, since it is a
        // pure function of (source bytes, config, build) — and returned
        // beside the graph. `bragi_version` rides along as
        // provenance documentation only.
        let config_hash = calculate_config_hash(config)?;
        let provenance = ParseProvenance {
            bragi_version: crate::VERSION.to_string(),
            source_format: "pdf".to_string(),
            source_sha256: pdf_hash.clone(),
            config_hash: config_hash.clone(),
        };

        // --- C3: cached output (the DocumentGraph) ---
        // C3 caches the finished, config-keyed graph — a hit skips the
        // deterministic build and returns the output directly. The read gates
        // on cache-eligibility alone (CR-89 removed the prior `should_write`
        // mis-gate — a read must not depend on the write flag). The writer is
        // wired by the C3 output-cache feature CR; until then this is a cheap
        // miss. (The golden freeze replays via `FreshFrom::C3`, for which
        // `should_use_cache(C3)` is false, so this branch is skipped there.)
        if fresh_from.should_use_cache(CachePoint::C3) {
            let cache_key = GraphCacheKey::new(pdf_hash.clone(), config_hash.clone());
            if let Some(cached) = self.storage.get_graph_output(&cache_key)? {
                info!(
                    nodes = cached.graph.nodes.len(),
                    source = "c3 cache",
                    duration_ms = start_time.elapsed().as_millis() as u64,
                    "pdf parse complete"
                );
                return Ok((cached.graph, provenance));
            }
        }

        let id_gen = NodeIdGenerator::new();

        // --- C2: Preprocessor cache check ---
        let extract_start = Instant::now();
        let (preprocessor_output, source) = if fresh_from.should_use_cache(CachePoint::C2) {
            if let Some(cached) = self.storage.get_preprocessor_output(&pdf_hash)? {
                debug!("c2 preprocessor cache hit, extraction skipped");
                (cached, "c2 cache")
            } else {
                self.extract_and_parse(
                    input_path,
                    &pdf_bytes,
                    &pdf_hash,
                    &fresh_from,
                    cache_defaults,
                    &mut profiler,
                )?
            }
        } else {
            self.extract_and_parse(
                input_path,
                &pdf_bytes,
                &pdf_hash,
                &fresh_from,
                cache_defaults,
                &mut profiler,
            )?
        };
        let extract_ms = extract_start.elapsed().as_millis() as u64;

        // --- Stages 2-5: Classification → Rules → Graph → Post-processing ---
        let graph = self.rules_and_graph(
            &preprocessor_output,
            config,
            &id_gen,
            &provenance,
            &pdf_hash,
            cache_defaults,
            &mut profiler,
            ExtractSummary { source, extract_ms },
        )?;

        if enable_profiling {
            profiler.print_summary();
        }
        info!(
            nodes = graph.nodes.len(),
            duration_ms = start_time.elapsed().as_millis() as u64,
            "pdf parse complete"
        );

        Ok((graph, provenance))
    }

    /// Simple document processing function using default config (no cache awareness)
    pub fn process_document(
        &mut self,
        input_path: &str,
    ) -> Result<(DocumentGraph, ParseProvenance)> {
        let default_config = ParsingConfig::default();
        self.process_document_with_cache(
            input_path,
            &default_config,
            FreshFrom::None,
            &CacheDefaults::default(),
            false,
        )
    }

    /// Process document with config loaded from file
    pub fn process_document_with_config_file(
        &mut self,
        input_path: &str,
        config_path: &str,
    ) -> Result<(DocumentGraph, ParseProvenance)> {
        let config = ParsingConfig::load_from_file(config_path)?;
        self.process_document_with_cache(
            input_path,
            &config,
            FreshFrom::None,
            &CacheDefaults::default(),
            false,
        )
    }

    // =========================================================================
    // Internal: extraction + parsing with C1/C2 cache awareness
    // =========================================================================

    /// Extract XHTML and parse to PreprocessorOutput, respecting C1 and C2 caches.
    ///
    /// Returns the output and where the XHTML came from (`"tika"` or
    /// `"c1 cache"`), for the stage summary event.
    fn extract_and_parse(
        &mut self,
        _input_path: &str,
        pdf_bytes: &[u8],
        pdf_hash: &str,
        fresh_from: &FreshFrom,
        cache_defaults: &CacheDefaults,
        profiler: &mut StepProfiler,
    ) -> Result<(PreprocessorOutput, &'static str)> {
        // --- C1: XHTML cache check ---
        let mut source = "tika";
        let xhtml = if fresh_from.should_use_cache(CachePoint::C1) {
            if let Some(cached) = self.storage.get_xhtml(pdf_hash)? {
                debug!("c1 xhtml cache hit, Tika skipped");
                source = "c1 cache";
                cached
            } else {
                let markup = profiler.time_step("C1: PDF → XHTML (Tika)", || {
                    self.preprocessor.parse_pdf_to_markup_language(pdf_bytes)
                })?;
                if cache_defaults.should_write(CachePoint::C1) {
                    self.storage.store_xhtml(pdf_hash, &markup)?;
                    debug!(bytes = markup.len(), "c1 xhtml cached");
                }
                markup
            }
        } else {
            // Fresh extraction requested
            let markup = profiler.time_step("C1: PDF → XHTML (Tika, fresh)", || {
                self.preprocessor.parse_pdf_to_markup_language(pdf_bytes)
            })?;
            if cache_defaults.should_write(CachePoint::C1) {
                self.storage.store_xhtml(pdf_hash, &markup)?;
                debug!(bytes = markup.len(), "c1 xhtml cached (refreshed)");
            }
            markup
        };

        // --- C2: Parse XHTML → PreprocessorOutput ---
        let output = profiler.time_step("C2: XHTML → PreprocessorOutput", || {
            self.preprocessor
                .parse_markup_to_preprocessor_output(&xhtml)
        })?;

        if cache_defaults.should_write(CachePoint::C2) {
            self.storage.store_preprocessor_output(pdf_hash, &output)?;
            debug!("c2 preprocessor output cached");
        }

        Ok((output, source))
    }

    // =========================================================================
    // Internal: classification → rules → graph (shared by all entry points)
    // =========================================================================

    /// Run classification, rules, and graph building on PreprocessorOutput.
    fn rules_and_graph(
        &mut self,
        preprocessor_output: &PreprocessorOutput,
        config: &ParsingConfig,
        id_gen: &NodeIdGenerator,
        parse_provenance: &ParseProvenance,
        pdf_hash: &str,
        cache_defaults: &CacheDefaults,
        profiler: &mut StepProfiler,
        extract: ExtractSummary,
    ) -> Result<DocumentGraph> {
        let rules_start = Instant::now();

        // Classification
        let classification = profiler.time_step("Classification", || {
            self.classifier.classify(preprocessor_output)
        })?;

        // Document analytics pre-pass (read by rules; sidecar-dumped to
        // `{cache_dir}/stat/<name>/<pdf_hash>.json` when `config.dump_analytics`).
        // No longer persisted into graph.json — that field went away with schema 0.4.0.
        let document_analysis = profiler.time_step("Document Analytics", || {
            run_analytics(&preprocessor_output.text_elements)
        });
        // Gated on both axes: `dump_analytics` (does the user want analytics
        // at all) AND `cache_defaults.stat` (may this run write the sidecar
        // into the cache dir). A read-only replay disables the latter, so the
        // committed fixture stays clean without a config pin.
        if config.dump_analytics && cache_defaults.stat {
            dump_stats(&*self.storage, pdf_hash, &document_analysis)?;
        }

        // Reading-order resort + region tagging (Block 06b). Annotates each
        // element with its Region tree leaf label and reorders the stream so
        // multi-column pages no longer interleave columns. Owned-clone of
        // `text_elements` because PreprocessorOutput is borrowed immutably
        // here; the cost is one Vec clone per document, negligible vs the
        // rules / graph-build work that follows.
        let text_elements = profiler.time_step("Reading-Order Resort", || {
            crate::analytics::tag_and_resort(
                preprocessor_output.text_elements.clone(),
                &document_analysis,
            )
        });

        // Rule processing
        let parsed_elements = if config.minimal_parse {
            debug!("minimal parse: rule processing skipped");
            self.rule_engine
                .convert_text_elements_to_parsed(&text_elements)
        } else {
            let font_size_analysis = profiler.time_step("Font Analysis", || {
                self.rule_engine
                    .analyze_font_sizes(&text_elements, &preprocessor_output.style_data)
            });

            profiler.time_step("Rules Processing", || {
                self.rule_engine.apply_rules_with_config(
                    &text_elements,
                    &classification,
                    &document_analysis,
                    &font_size_analysis,
                    &preprocessor_output.style_data,
                    config,
                )
            })?
        };
        let rules_ms = rules_start.elapsed().as_millis() as u64;
        let elements = parsed_elements.len();

        // Infer title before graph build consumes elements
        let inferred_title = infer_title(&parsed_elements);

        // PDF channel exit: project rule output onto SemanticTreeElement.
        // Everything from here is channel-agnostic.
        let semantic_elements = profiler.time_step("Channel Projection", || {
            project_to_semantic_tree(parsed_elements)
        });

        // Block A / A3: the CR-78 detection-confidence signal no longer
        // rides on DocumentNode (it left the wire + the hash). Capture it
        // as a transient text_order-keyed sidecar for the CR-78 Phase B
        // min_confidence filter in graph_sanity before the elements move
        // into the builder. Non-zero entries only (absent == 0).
        let section_confidence: std::collections::HashMap<u32, u8> = semantic_elements
            .iter()
            .filter(|e| e.confidence > 0)
            .map(|e| (e.text_order, e.confidence))
            .collect();

        // Graph construction (deterministic UUIDv5 node IDs). Block A:
        // the builder no longer takes provenance — the graph is content
        // only; provenance stays a value in this scope and is threaded
        // where needed (evidence artifact below; emit/serialize by our
        // caller).
        let graph_start = Instant::now();
        let mut graph = profiler.time_step("Graph Construction", || {
            self.graph_builder
                .build_graph_deterministic(semantic_elements, id_gen)
        })?;

        info!(
            source = extract.source,
            pages = page_count(preprocessor_output),
            text_elements = preprocessor_output.text_elements.len(),
            elements,
            nodes = graph.nodes.len(),
            extract_ms = extract.extract_ms,
            rules_ms,
            graph_ms = graph_start.elapsed().as_millis() as u64,
            "pdf stages complete"
        );

        // Post-processing: metadata, analysis, breadcrumbs. CR-57: direct
        // assignment replaces the old merge_extracted call (each channel
        // now writes a complete DocumentMetadata in its extractor).
        graph.document_info.document_metadata = preprocessor_output.metadata.clone();
        // Body-side title inference is honored only when source-native
        // extraction returned None — see the entry-point analog above
        // for the F-02 deferral rationale.
        if graph.document_info.document_metadata.title.is_none() {
            if let Some(title) = inferred_title {
                graph.document_info.document_metadata.title = Some(title);
            }
        }
        graph.document_info.outline_data = preprocessor_output.bookmark_data.clone();
        // Block A / A2: the structural profile no longer lives on the
        // graph — it is a json-only aggregate recomputed at
        // serialization time, so the pre/post-sanity recompute dance
        // (CR-66) is gone. graph_sanity reads the graph directly.
        crate::graphs::graph_sanity::apply(
            &mut graph,
            &config.graph_sanity,
            Some(parse_provenance),
            Some(&section_confidence),
        );
        graph.compute_breadcrumbs();

        // CR-84: node identity is finalized as late as possible — after
        // every topology-mutating pass — and derived from the final
        // emitted structure. `graph_sanity` may have re-parented /
        // re-depthed nodes (CR-70 rebalance, demotions) without re-keying
        // them; this re-runs the builder's shared ID-derivation walk over
        // the settled topology so the emitted IDs equal what the reverse
        // parser re-derives from the emitted tree (the round-trip
        // contract). No-op when sanity didn't move topology (idempotent).
        let rekeyed = profiler.time_step("Node ID Re-key (post-sanity)", || {
            crate::graphs::builder::rekey_node_ids(&mut graph)
        });
        if rekeyed.ids_moved > 0 || rekeyed.paths_moved > 0 {
            debug!(
                ids_moved = rekeyed.ids_moved,
                paths_moved = rekeyed.paths_moved,
                "node ids re-keyed to post-sanity topology"
            );
        }

        // CR-86 / DT-12: build-time value gate for `style_info`. The PDF
        // pipeline populates `DocumentNode.style_info` on every body node
        // (via `project_style` → the builder), and the rules above consume
        // it (font-based section detection, detectors) — so we keep it
        // populated through the whole build. Here, as the last step before
        // the graph leaves the builder, we gate the *emitted* value on the
        // config: unless `include_style_info` is on, strip it to `None` so
        // the graph carries the config-correct value. `bgraph_sha256`, md,
        // and json all serialize this one graph → hash equals wire by
        // construction (the field is always on the wire as `null`; only the
        // value is gated here). Distinct `config_hash` per edition keeps the
        // C3 graph cache keyed correctly (no null/data collision).
        if !config.include_style_info {
            for node in graph.nodes.values_mut() {
                node.style_info = None;
            }
        }

        Ok(graph)
    }
}

/// Extraction-stage facts carried into the `pdf stages complete` event.
struct ExtractSummary {
    /// Where the elements came from: `"tika"`, `"c1 cache"` or `"c2 cache"`.
    source: &'static str,
    extract_ms: u64,
}

/// Page count for the stage summary: the PDF's declared page count when
/// its metadata carries one, else the highest page number seen in the
/// extracted text.
fn page_count(output: &PreprocessorOutput) -> u32 {
    let declared = output.metadata.pdf.as_ref().and_then(|p| p.page_count);
    declared.unwrap_or_else(|| {
        output
            .text_elements
            .iter()
            .map(|e| e.placement.page_number)
            .max()
            .unwrap_or(0)
    })
}

/// Run the document-analytics pre-pass over a slice of text elements.
///
/// Single-pass walk: dispatches each element to every enabled stat kind via
/// `AnalysisBuilder`, then finalizes in dependency order. Output is consumed
/// in pipeline memory by downstream rules and (when `dump_analytics`) written
/// to per-stat sidecar files via [`dump_stats`].
fn run_analytics(text_elements: &[PdfTextElement]) -> DocumentAnalysis {
    let mut builder = AnalysisBuilder::new();
    for element in text_elements {
        builder.observe(element);
    }
    builder.finalize()
}

/// Per-stat sidecar dump. One JSON file per stat kind under
/// `{cache_dir}/stat/<Statistic::NAME>/<pdf_hash>.json`. Folder-per-stat
/// scoping (Marcus, Block 05) lets future stat kinds (RegionStats,
/// PageOutlier, …) drop in without colliding. The full composite is the
/// in-memory shape; the sidecar splits it for grep-ability and per-stat diff
/// against Python prototype outputs.
fn dump_stats(
    storage: &dyn DocumentStorage,
    pdf_hash: &str,
    analysis: &DocumentAnalysis,
) -> Result<()> {
    let font_json = serde_json::to_string_pretty(&analysis.font)?;
    storage.store_stat(pdf_hash, FontStatsBuilder::NAME, &font_json)?;

    let geometry_json = serde_json::to_string_pretty(&analysis.geometry)?;
    storage.store_stat(pdf_hash, GeometryStatsBuilder::NAME, &geometry_json)?;

    let page_stats_json = serde_json::to_string_pretty(&analysis.page_stats)?;
    storage.store_stat(pdf_hash, PageStatsBuilder::NAME, &page_stats_json)?;

    let region_json = serde_json::to_string_pretty(&analysis.region)?;
    storage.store_stat(pdf_hash, RegionStatsBuilder::NAME, &region_json)?;

    Ok(())
}
