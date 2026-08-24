//! CR-97 WP1 — the sanity *threshold tail* as a standalone callable unit.
//!
//! The tail is the suffix of the post-build pipeline that turns a finished
//! baseline graph into a stricter-threshold candidate without re-running
//! detection: CR-78 `min_confidence` demote → CR-70 topology rebalance →
//! breadcrumbs → CR-84 node-ID re-key. Inside the pipeline this sequence
//! only runs config-bound (`graph_sanity::apply()` + the `processor.rs`
//! post-steps); [`apply_threshold_tail`] exposes it over a *clone* of an
//! already-processed graph so a caller can generate candidate parses
//! `P_k` in milliseconds (CR-97 Stage 2, ~4 ms p50 / 11 ms p95 per rung).
//!
//! ## Capture-point contract (CR-97 implementation flow, decision 1)
//!
//! The input graph is `P_0` **post-`apply()`** — the finished baseline
//! graph after full `graph_sanity::apply()`, `compute_breadcrumbs()`, and
//! `rekey_node_ids()`, but **before** the CR-86 `style_info` strip: the
//! CR-70 rebalance reads `style_info` (font stem + size) to level the
//! surviving sections, so a style-stripped input would diverge from a
//! full pipeline run at the same threshold. Callers that gate
//! `style_info` off apply the strip to the *returned* candidate, exactly
//! where the pipeline applies it to `P_0`.
//!
//! Equivalence obligation: `apply_threshold_tail(P_0, k, sidecar, cfg)`
//! must be byte-identical (`bgraph_sha256`) to a full pipeline run with
//! `min_confidence = k` — proven per-doc across the bookmarked corpus in
//! lab experiment `2026-08-24-tail-equivalence` before anything builds on
//! this function.
//!
//! Cross-references: CR-97 (config-optimiser v1 API integration) § Stage
//! 2 + gap G1; design flow `2026-08-24-cr-97-implementation.md` decisions
//! 1–2; handoff `2026-08-24-wp1-threshold-tail-core.md`.

use crate::config::GraphSanityConfig;
use crate::graphs::graph_sanity::{rebalance_topology, SanityReport};
use crate::types::{DocumentGraph, DocumentNode};
use std::collections::HashMap;

/// Re-run the sanity threshold tail over a clone of a finished graph,
/// producing the candidate graph for `min_confidence`.
///
/// Steps (mirroring the pipeline exactly):
///
/// 1. **Clone** — the input graph is never mutated.
/// 2. **CR-78 demote** — every `Section` whose detection-time confidence
///    (the `text_order`-keyed `section_confidence` sidecar; an absent
///    entry reads as 0) is below `min_confidence` becomes a `Paragraph`.
///    Same semantics as the filter inside `graph_sanity::apply()`.
/// 3. **CR-70 rebalance** — rebuild parent/child/depth over the
///    survivors, with the config's `topology_rebalance` +
///    `numbering_restart` blocks, exactly as `apply()` invokes it.
/// 4. **Breadcrumbs** — recompute ancestry trails from the settled tree.
/// 5. **CR-84 re-key** — finalize node IDs + `location.semantic.path`
///    from the settled topology (idempotent when nothing moved).
///
/// Gating mirrors the pipeline: with `config.enabled == false` the
/// demote and rebalance are skipped (as `apply()` would skip them), and
/// the rebalance additionally honors its own `check`/`correct` toggles.
/// Steps 4–5 always run — in the pipeline they live outside `apply()`
/// and run unconditionally.
///
/// Pure function of its inputs: no I/O, no logging, deterministic —
/// repeated calls over the same inputs yield byte-identical graphs
/// (canonical JSON; the CR-84 determinism obligation).
pub fn apply_threshold_tail(
    graph: &DocumentGraph,
    min_confidence: u8,
    section_confidence: &HashMap<u32, u8>,
    config: &GraphSanityConfig,
) -> DocumentGraph {
    let mut candidate = graph.clone();

    if config.enabled {
        // ── CR-78 demote (graph_sanity::apply()'s min_confidence filter,
        // silent). `min_confidence == 0` is structurally a no-op (u8
        // confidence is never < 0), matching the pipeline's `> 0` gate.
        if min_confidence > 0 {
            let confidence_of = |n: &DocumentNode| -> u8 {
                n.text_order
                    .and_then(|t| section_confidence.get(&t).copied())
                    .unwrap_or(0)
            };
            for n in candidate.nodes.values_mut() {
                if n.node_type == "Section" && confidence_of(n) < min_confidence {
                    n.node_type = "Paragraph".to_string();
                }
            }
        }

        // ── CR-70 rebalance, mirroring apply()'s invocation. The report is
        // diagnostic-only here and intentionally dropped: the tail's contract
        // is the candidate graph, deterministically.
        let tr = &config.invariants.topology_rebalance;
        let nr = &config.invariants.numbering_restart;
        if tr.check || tr.correct {
            let mut report = SanityReport::default();
            rebalance_topology(&mut candidate, tr, nr, &mut report);
        }
    }

    // ── Breadcrumbs + CR-84 re-key: the pipeline's unconditional
    // post-sanity steps (processor.rs / api processing.rs).
    candidate.compute_breadcrumbs();
    crate::graphs::builder::rekey_node_ids(&mut candidate);

    candidate
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GraphSanityConfig;
    use crate::graphs::serialization::canonical::canonical_json;
    use crate::types::{DocumentNode, StyleMetadata};
    use uuid::Uuid;

    /// Build a small pipeline-shaped graph: a Document root parenting a
    /// flat run of nodes in text_order, with numbered + font-levelled
    /// sections and paragraph content. Deliberately *unsettled* (flat
    /// topology, stale depths) so the tail's rebalance + re-key have real
    /// work to do.
    fn make_graph() -> DocumentGraph {
        let specs: &[(&str, &str, Option<f32>)] = &[
            ("Section", "1. Introduction", Some(18.0)),
            ("Paragraph", "Opening prose.", None),
            ("Section", "1.1 Background", Some(14.0)),
            ("Paragraph", "Background prose.", None),
            ("Section", "Sidebar noise", Some(10.0)),
            ("Paragraph", "Noise body.", None),
            ("Section", "2. Methods", Some(18.0)),
            ("Paragraph", "Methods prose.", None),
        ];
        let root_id = Uuid::new_v4();
        let mut graph = DocumentGraph::new_with_root(root_id);
        let mut root = DocumentNode::new_with_id(root_id, "Document", "root".into());
        root.location.semantic.depth = 0;
        // Pipeline convention: the Document root carries no text_order
        // (CR-84's re-key walks body nodes only — `text_order.is_some()`).
        root.text_order = None;
        for (i, (node_type, text, size)) in specs.iter().enumerate() {
            let id = Uuid::new_v4();
            let mut node = DocumentNode::new_with_id(id, node_type, (*text).to_string());
            node.text_order = Some(i as u32);
            node.location.semantic.depth = 1;
            node.parent = Some(root_id);
            if let Some(sz) = size {
                node.style_info = Some(StyleMetadata {
                    font_class: String::new(),
                    font_size: Some(*sz),
                    is_bold: false,
                    is_italic: false,
                    font_family: Some("TestFont".to_string()),
                    foreground_color: None,
                    background_color: None,
                });
            }
            root.children.push(id);
            graph.nodes.insert(id, node);
        }
        graph.nodes.insert(root_id, root);
        graph
    }

    /// text_order-keyed confidence sidecar: the numbered sections are
    /// high-confidence, the sidebar FP sits below the mc4 floor.
    fn sidecar() -> HashMap<u32, u8> {
        HashMap::from([(0, 7), (2, 5), (4, 2), (6, 7)])
    }

    #[test]
    fn tail_is_deterministic_across_repeated_calls_and_clones() {
        let graph = make_graph();
        let sc = sidecar();
        let config = GraphSanityConfig::default();

        let first = apply_threshold_tail(&graph, 4, &sc, &config);
        let second = apply_threshold_tail(&graph, 4, &sc, &config);
        assert_eq!(
            canonical_json(&first),
            canonical_json(&second),
            "repeated tails over the same inputs must be byte-identical"
        );

        // CR-84 determinism across clones: an independently cloned input
        // yields the same bytes.
        let cloned_input = graph.clone();
        let third = apply_threshold_tail(&cloned_input, 4, &sc, &config);
        assert_eq!(
            canonical_json(&first),
            canonical_json(&third),
            "a tail over a cloned input must be byte-identical"
        );
    }

    #[test]
    fn tail_does_not_mutate_its_input() {
        let graph = make_graph();
        let before = canonical_json(&graph);
        let _ = apply_threshold_tail(&graph, 4, &sidecar(), &GraphSanityConfig::default());
        assert_eq!(
            before,
            canonical_json(&graph),
            "the input graph must be untouched (pure function of its inputs)"
        );
    }

    #[test]
    fn tail_demotes_below_floor_and_keeps_survivors() {
        let graph = make_graph();
        let out = apply_threshold_tail(&graph, 4, &sidecar(), &GraphSanityConfig::default());

        let sections: Vec<&str> = {
            let mut s: Vec<(&u32, &str)> = out
                .nodes
                .values()
                .filter(|n| n.node_type == "Section")
                .map(|n| (n.text_order.as_ref().unwrap(), n.content.text.as_str()))
                .collect();
            s.sort();
            s.into_iter().map(|(_, t)| t).collect()
        };
        assert_eq!(
            sections,
            vec!["1. Introduction", "1.1 Background", "2. Methods"],
            "the confidence-2 sidebar must demote at min_confidence=4; the rest survive"
        );
        // Node count is preserved: demotion changes type, never drops nodes.
        assert_eq!(out.nodes.len(), graph.nodes.len());
        // The demoted sidebar is now a leaf Paragraph.
        let sidebar = out
            .nodes
            .values()
            .find(|n| n.content.text == "Sidebar noise")
            .expect("demoted node still present");
        assert_eq!(sidebar.node_type, "Paragraph");
        assert!(sidebar.children.is_empty());
    }

    #[test]
    fn tail_at_zero_on_a_settled_graph_is_identity() {
        // Settle the graph the way the pipeline settles P_0: full apply()
        // → breadcrumbs → rekey. A min_confidence=0 tail over the settled
        // graph must then be a byte-identical no-op (the additive
        // baseline-rung contract).
        let mut settled = make_graph();
        let sc = sidecar();
        let config = GraphSanityConfig::default();
        crate::graphs::graph_sanity::apply(&mut settled, &config, None, Some(&sc));
        settled.compute_breadcrumbs();
        crate::graphs::builder::rekey_node_ids(&mut settled);

        let out = apply_threshold_tail(&settled, 0, &sc, &config);
        assert_eq!(
            canonical_json(&settled),
            canonical_json(&out),
            "tail(P_0, 0) must reproduce P_0 byte-identically"
        );
    }

    #[test]
    fn tail_with_sanity_disabled_skips_demote_and_rebalance() {
        let mut settled = make_graph();
        let sc = sidecar();
        let disabled = GraphSanityConfig {
            enabled: false,
            ..GraphSanityConfig::default()
        };
        // Settle breadcrumbs + IDs so the tail's unconditional post-steps
        // are no-ops and identity is checkable.
        settled.compute_breadcrumbs();
        crate::graphs::builder::rekey_node_ids(&mut settled);

        let out = apply_threshold_tail(&settled, 4, &sc, &disabled);
        assert_eq!(
            canonical_json(&settled),
            canonical_json(&out),
            "with graph_sanity disabled the pipeline never demotes, so neither may the tail"
        );
    }
}
