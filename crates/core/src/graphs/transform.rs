//! The graph transform seam — `DocumentGraph` + typed change-set →
//! **rebuilt** `DocumentGraph`.
//!
//! ## Why a seam and not a patch
//!
//! CR-98 taught the lesson this module generalizes. Normalizing OCR
//! outline levels *looks* like a small edit to a finished graph — bump a
//! few Section depths and move on — but a `DocumentGraph`'s depths are
//! load-bearing: parent/child edges, `SemanticLocation` path + depth,
//! breadcrumbs, node IDs (CR-83 keys the ID off the ancestor-heading
//! path), the re-keyed document root, and `outline_data` are all
//! *derived* from them. So CR-98 ran its normalization **before** the
//! build ([`crate::preprocessors::ocr::parse_ocr`], step 2b) and let the
//! normal builder derive everything downstream.
//!
//! A change that arrives *after* the graph is finished — a model's
//! opinion about outline levels, say, which cannot be known at parse time
//! — has nowhere to go under that rule. This module is the answer:
//! decompose the finished graph back into the `Vec<SemanticTreeElement>`
//! the builder consumes, apply the change there, and re-run the *normal*
//! build path. Nothing is hand-patched; every derived structure is
//! re-derived by the code that owns it.
//!
//! ```text
//! DocumentGraph ─ decompose ─→ Vec<SemanticTreeElement>
//!                                      │  apply change-set
//!                                      ▼
//!                          GraphBuilder::build_graph_deterministic
//!                                      │
//!                       restore document_info ─→ outline assembly
//!                                      │
//!                            compute_breadcrumbs ─→ DocumentGraph
//! ```
//!
//! ## Contract
//!
//! [`transform_graph`] is **pure and deterministic**: no I/O, no clock,
//! no network, no model call — those all live outside core, on the caller's
//! side of the DT-13 determinism boundary. Given the same graph and the
//! same change-set it produces the same graph, byte for byte.
//!
//! - An **empty** change-set is the identity: decompose + rebuild
//!   reproduces the input graph exactly (node IDs included — the builder's
//!   derivation is a pure function of the `(node_type, depth, text_order,
//!   content)` projection, which is what decomposition recovers).
//! - Everything the change-set does not name survives byte-exact:
//!   document metadata, `internal_refs` / `external_refs` (so a grafted
//!   graph keeps its grafted refs), physical locations, text, token
//!   counts, style, `flow_type`, `kind`, `topology`, and emission order.
//! - A malformed change-set is a **typed error** ([`TransformError`]);
//!   the caller decides whether to degrade or fail.
//! - No `FormatVersion` bump: values move, the shape does not.
//!
//! ## v1 change kinds
//!
//! Exactly one: [`GraphChange::RelevelSections`]. [`GraphChange`] is
//! `#[non_exhaustive]` so future kinds slot in without a breaking change,
//! but nothing beyond re-leveling is built (CR-12).
//!
//! ## Known wrinkle — `outline_data` provenance
//!
//! `outline_data` is **section-derived** on the OCR channel (assembled
//! from the emitted Sections) but **source-native** on the PDF channel
//! (the PDF's own `/Outlines` bookmarks, which are not a function of the
//! node set at all). This seam re-derives it from Sections
//! ([`BookmarkData::from_leveled_sections`], the same call `parse_ocr`
//! makes), which is *correct on the OCR channel* — the only channel v1 is
//! exercised on — and would **replace** a PDF graph's native bookmarks
//! with a section-derived outline.
//!
//! That is a known, deliberately-unsolved gap, not an oversight: solving
//! it generally means teaching `DocumentInfo` where its outline came from,
//! which is a schema question this CR does not open. Until then: **do not
//! run this transform on a source-native-outline graph** without deciding
//! that question first. [`tests::pdf_style_native_outline_is_rederived`]
//! pins the current behavior so the gap is visible rather than latent.

use crate::graphs::builder::GraphBuilder;
use crate::graphs::node_id::NodeIdGenerator;
use crate::types::{
    BookmarkData, DocumentGraph, DocumentNode, SemanticElementType, SemanticTreeElement,
};

/// Deepest section level a change-set may assign. Matches the OCR
/// numbering normalization's `MAX_OUTLINE_DEPTH` (CR-98) — four ranks is
/// as deep as any outline we emit goes, and a deeper value is far more
/// likely to be a producer bug than a real seventh-level subsection.
pub const MAX_SECTION_LEVEL: u32 = 4;

/// One typed change to apply to a graph before it is rebuilt.
///
/// `#[non_exhaustive]`: v1 ships exactly one kind, and the enum is open
/// so a future "deterministic improvement round" (merge adjacent
/// paragraphs, reclassify a block, drop running noise) can add a variant
/// without breaking callers. Matching exhaustively outside this crate is
/// deliberately not possible.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GraphChange {
    /// Replace every Section's hierarchy level, in **Section emission
    /// order** (`text_order` ascending over the graph's Section nodes).
    ///
    /// `levels` must carry exactly one entry per Section, each in
    /// `1..=`[`MAX_SECTION_LEVEL`] — anything else is a
    /// [`TransformError`], never a silent repair.
    ///
    /// Non-Section leaves follow their governing Section by the same
    /// delta, so a leaf keeps its offset below the heading it hangs from
    /// (levels are clamped at 1). Leaves that precede every Section are
    /// untouched.
    RelevelSections { levels: Vec<u32> },
}

/// Why a transform could not be applied. Every variant is a caller-side
/// bug or a malformed change-set — the transform itself has no failure
/// modes of its own.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TransformError {
    /// A [`GraphChange::RelevelSections`] carried a different number of
    /// levels than the graph has Sections.
    #[error("re-level change-set has {got} levels but the graph has {expected} sections")]
    SectionCountMismatch { expected: usize, got: usize },

    /// A level fell outside `1..=`[`MAX_SECTION_LEVEL`].
    #[error(
        "re-level change-set level {level} at section index {index} is outside 1..={max}",
        max = MAX_SECTION_LEVEL
    )]
    LevelOutOfRange { index: usize, level: u32 },

    /// The input graph could not be decomposed: its body nodes'
    /// `text_order` is not the gap-free `0..N` sequence the builder
    /// contract requires. Only reachable on a hand-built or corrupted
    /// graph.
    #[error("graph body node at position {position} has text_order {text_order} (expected {position})")]
    TextOrderDrift { position: usize, text_order: u32 },

    /// A body node carried a `node_type` with no
    /// [`SemanticElementType`] (a wire type this build does not know, or
    /// the orphan `Message` sentinel).
    #[error("graph body node carries unknown node_type {node_type:?}")]
    UnknownNodeType { node_type: String },

    /// The rebuild failed inside [`GraphBuilder`]. Carries the builder's
    /// message; not expected in practice.
    #[error("graph rebuild failed: {0}")]
    Rebuild(String),
}

/// Apply `changes` to `graph` and return the **rebuilt** graph.
///
/// Pure and deterministic — see the module docs for the full contract.
/// An empty `changes` slice is the identity transform (a decompose +
/// rebuild round-trip that must reproduce the input exactly), which is
/// what makes this seam safe to sit unconditionally in a pipeline.
pub fn transform_graph(
    graph: &DocumentGraph,
    changes: &[GraphChange],
) -> Result<DocumentGraph, TransformError> {
    let mut elements = decompose(graph)?;

    for change in changes {
        match change {
            GraphChange::RelevelSections { levels } => apply_relevel(&mut elements, levels)?,
        }
    }

    rebuild(graph, elements)
}

/// Every Section's `(text, level)` in emission order — the skeleton a
/// caller re-levels, and the input [`GraphChange::RelevelSections`]
/// expects one level per entry for.
///
/// Exposed because whoever produces a change-set
/// needs exactly this projection, and re-deriving it independently would
/// be a chance for the two orders to disagree.
pub fn section_skeleton(graph: &DocumentGraph) -> Vec<(&DocumentNode, u32)> {
    let mut sections: Vec<&DocumentNode> = graph
        .nodes
        .values()
        .filter(|n| n.text_order.is_some() && n.node_type == "Section")
        .collect();
    sections.sort_by_key(|n| n.text_order.unwrap_or(0));
    sections
        .into_iter()
        .map(|n| {
            let level = n.location.semantic.depth;
            (n, level)
        })
        .collect()
}

// =============================================================================
// Decompose → change → rebuild.
// =============================================================================

/// Project a finished graph back into the element vec the builder
/// consumes. The inverse of `GraphBuilder::create_node`: everything the
/// builder *stored* comes back; everything it *derived* (ids, parent,
/// children, path, breadcrumbs, root) is dropped and will be re-derived.
///
/// `confidence` is not recoverable — it deliberately never reaches a
/// `DocumentNode` (Block A / A3, the CR-78 signal stays parser-internal)
/// — so it comes back `0`. It is a build-time sidecar for `graph_sanity`
/// and has no effect on any derived structure, so the round-trip stays
/// exact.
fn decompose(graph: &DocumentGraph) -> Result<Vec<SemanticTreeElement>, TransformError> {
    let mut body: Vec<&DocumentNode> = graph
        .nodes
        .values()
        .filter(|n| n.text_order.is_some())
        .collect();
    body.sort_by_key(|n| n.text_order.unwrap_or(0));

    let mut elements = Vec::with_capacity(body.len());
    for (position, node) in body.into_iter().enumerate() {
        let text_order = node.text_order.unwrap_or(0);
        if text_order as usize != position {
            return Err(TransformError::TextOrderDrift {
                position,
                text_order,
            });
        }
        let element_type = semantic_type_for(&node.node_type).ok_or_else(|| {
            TransformError::UnknownNodeType {
                node_type: node.node_type.clone(),
            }
        })?;
        elements.push(SemanticTreeElement {
            text: node.content.text.clone(),
            element_type,
            // The builder writes `element.hierarchy_level` straight onto
            // `location.semantic.depth`, so the stored depth *is* the
            // level it was built from.
            hierarchy_level: node.location.semantic.depth,
            text_order,
            physical_location: node.location.physical.clone(),
            style: node.style_info.clone(),
            token_count: node.token_count,
            internal_refs: node.internal_refs.clone(),
            external_refs: node.external_refs.clone(),
            confidence: 0,
            // CR-100: the image payload survives the decompose/rebuild
            // round the transform seam performs — a re-level must not
            // silently drop the pictures it is not touching.
            image: node.image.clone(),
        });
    }
    Ok(elements)
}

/// Apply [`GraphChange::RelevelSections`] to a decomposed element vec.
///
/// Sections take their new level positionally; each non-Section leaf
/// shifts by the same delta its governing Section moved, so leaf
/// attachment survives the re-level exactly as it was authored (the OCR
/// channel's leaves sit at `section + 1`, so they land back at
/// `new_section + 1`). Levels floor at 1.
fn apply_relevel(
    elements: &mut [SemanticTreeElement],
    levels: &[u32],
) -> Result<(), TransformError> {
    let section_count = elements
        .iter()
        .filter(|e| e.element_type == SemanticElementType::Section)
        .count();
    if levels.len() != section_count {
        return Err(TransformError::SectionCountMismatch {
            expected: section_count,
            got: levels.len(),
        });
    }
    for (index, level) in levels.iter().enumerate() {
        if *level < 1 || *level > MAX_SECTION_LEVEL {
            return Err(TransformError::LevelOutOfRange {
                index,
                level: *level,
            });
        }
    }

    let mut section_idx = 0usize;
    // Delta the currently-governing Section moved by; `0` before any
    // Section, so leading leaves are untouched.
    let mut delta: i64 = 0;
    for element in elements.iter_mut() {
        if element.element_type == SemanticElementType::Section {
            let new_level = levels[section_idx];
            delta = i64::from(new_level) - i64::from(element.hierarchy_level);
            element.hierarchy_level = new_level;
            section_idx += 1;
        } else {
            element.hierarchy_level =
                (i64::from(element.hierarchy_level) + delta).max(1) as u32;
        }
    }
    Ok(())
}

/// Re-run the normal build path over the (possibly changed) elements and
/// restore the document-level facts the builder does not own.
///
/// Mirrors `parse_ocr`'s post-build sequence exactly — document_info
/// fields, then outline assembly, then `compute_breadcrumbs` (which needs
/// the title already in place, since the title is the first crumb).
fn rebuild(
    source: &DocumentGraph,
    elements: Vec<SemanticTreeElement>,
) -> Result<DocumentGraph, TransformError> {
    let id_gen = NodeIdGenerator::new();
    let mut graph = GraphBuilder::new()
        .build_graph_deterministic(elements, &id_gen)
        .map_err(|e| TransformError::Rebuild(e.to_string()))?;

    // Document-level facts the builder does not derive. `root_id` is
    // NOT restored — the builder re-keys it from the finished node set
    // (CR-83 / DT-10), which is exactly the derivation we want.
    graph.document_info.kind = source.document_info.kind.clone();
    graph.document_info.document_metadata = source.document_info.document_metadata.clone();
    graph.document_info.flow_type = source.document_info.flow_type.clone();
    graph.document_info.topology = source.document_info.topology.clone();
    // CR-20: a resolved title is a document-level fact the builder does
    // not derive, exactly like the four above — and `compute_breadcrumbs`
    // below reads it, so it has to be in place first. Without this line
    // a re-level would silently drop a resolved title on the floor.
    graph.document_info.resolved_title = source.document_info.resolved_title.clone();

    // Outline: re-derived from the rebuilt Sections. See the module
    // docs' "Known wrinkle" — correct on the OCR channel, replaces a
    // source-native (PDF bookmark) outline.
    let skeleton: Vec<(String, u32)> = section_skeleton(&graph)
        .into_iter()
        .map(|(node, level)| (node.content.text.clone(), level))
        .collect();
    graph.document_info.outline_data = BookmarkData::from_leveled_sections(&skeleton);

    graph.compute_breadcrumbs();
    Ok(graph)
}

/// Wire `node_type` → [`SemanticElementType`]. The inverse of
/// [`crate::graphs::builder::node_type_for`]; `None` for a type this
/// build does not know and for the orphan `Message` sentinel (which has
/// no tree-topology production path — see `types.rs`).
fn semantic_type_for(node_type: &str) -> Option<SemanticElementType> {
    Some(match node_type {
        "Section" => SemanticElementType::Section,
        "Paragraph" => SemanticElementType::Paragraph,
        "Header" => SemanticElementType::Header,
        "Footer" => SemanticElementType::Footer,
        "Margin" => SemanticElementType::Margin,
        "CodeBlock" => SemanticElementType::CodeBlock,
        "List" => SemanticElementType::List,
        "Blockquote" => SemanticElementType::Blockquote,
        "Table" => SemanticElementType::Table,
        "Equation" => SemanticElementType::Equation,
        "Image" => SemanticElementType::Image,
        _ => return None,
    })
}

// =============================================================================
// Tests. Pure — no fixtures on disk, no JVM, no I/O.
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphs::serialization::canonical::bgraph_sha256;
    use crate::graphs::serialization::markdown::emit_markdown;
    use crate::types::{
        BoundingBox, ExternalRef, ExternalRefTarget, FlowType, InternalRef, InternalRefTarget,
        ParseProvenance, PhysicalLocation,
    };

    fn provenance() -> ParseProvenance {
        ParseProvenance {
            bragi_version: "test".to_string(),
            source_format: "ocr".to_string(),
            source_sha256: "test-source".to_string(),
            config_hash: "none".to_string(),
        }
    }

    /// Build a graph the way a channel does: elements → builder →
    /// document_info → outline → breadcrumbs. This is the shape the
    /// transform must round-trip exactly.
    fn build(rows: &[(&str, &str, u32)]) -> DocumentGraph {
        let elements: Vec<SemanticTreeElement> = rows
            .iter()
            .enumerate()
            .map(|(i, (node_type, text, level))| SemanticTreeElement {
                text: (*text).to_string(),
                element_type: semantic_type_for(node_type).expect("known test node type"),
                hierarchy_level: *level,
                text_order: i as u32,
                physical_location: Some(PhysicalLocation {
                    page: (i as u32 / 3) + 1,
                    bounding_box: BoundingBox {
                        x: i as f32,
                        y: i as f32 * 2.0,
                        width: 100.0,
                        height: 12.5,
                    },
                }),
                style: None,
                token_count: text.len(),
                internal_refs: vec![],
                external_refs: vec![],
                confidence: 0,
                image: None,
            })
            .collect();

        let mut graph = GraphBuilder::new()
            .build_graph_deterministic(elements, &NodeIdGenerator::new())
            .expect("test graph builds");
        graph.document_info.flow_type = FlowType::Fixed;
        graph.document_info.document_metadata.title = Some("Test Document".to_string());
        let skeleton: Vec<(String, u32)> = section_skeleton(&graph)
            .into_iter()
            .map(|(n, lvl)| (n.content.text.clone(), lvl))
            .collect();
        graph.document_info.outline_data = BookmarkData::from_leveled_sections(&skeleton);
        graph.compute_breadcrumbs();
        graph
    }

    /// The wobbly-outline shape CR-12 exists for: an appendix heading
    /// promoted to level 1 alongside real chapters.
    fn wobbly() -> DocumentGraph {
        build(&[
            ("Section", "Deep Residual Learning", 1),
            ("Paragraph", "Abstract text.", 2),
            ("Section", "Introduction", 1),
            ("Paragraph", "Body text.", 2),
            ("Section", "Experiments", 1),
            ("Paragraph", "More body.", 2),
            ("Section", "A. Appendix", 1),
            ("Paragraph", "Appendix body.", 2),
            ("Section", "PASCAL VOC", 1),
            ("Paragraph", "Detail.", 2),
        ])
    }

    fn levels_by_order(graph: &DocumentGraph) -> Vec<(u32, String, u32)> {
        let mut rows: Vec<(u32, String, u32)> = graph
            .nodes
            .values()
            .filter_map(|n| {
                n.text_order.map(|t| {
                    (
                        t,
                        n.content.text.clone(),
                        n.location.semantic.depth,
                    )
                })
            })
            .collect();
        rows.sort_by_key(|(t, _, _)| *t);
        rows
    }

    // --- identity ---------------------------------------------------

    /// The empty change-set is the identity: decompose + rebuild
    /// reproduces the graph byte-for-byte, node IDs and root included.
    #[test]
    fn empty_change_set_is_byte_identical() {
        let graph = wobbly();
        let out = transform_graph(&graph, &[]).expect("identity transform succeeds");

        assert_eq!(bgraph_sha256(&out), bgraph_sha256(&graph));
        assert_eq!(
            emit_markdown(&out, &provenance()),
            emit_markdown(&graph, &provenance())
        );
        assert_eq!(out.document_info.root_id, graph.document_info.root_id);
        assert_eq!(out.nodes.len(), graph.nodes.len());
        for (id, node) in &graph.nodes {
            let rebuilt = out.nodes.get(id).expect("every node id survives identity");
            assert_eq!(rebuilt.location.semantic.path, node.location.semantic.path);
            assert_eq!(
                rebuilt.location.semantic.breadcrumbs,
                node.location.semantic.breadcrumbs
            );
            assert_eq!(rebuilt.parent, node.parent);
            assert_eq!(rebuilt.children, node.children);
        }
    }

    /// A re-level change-set that hands back the graph's *current* levels
    /// is also the identity — the levels are what they already were.
    #[test]
    fn relevel_to_current_levels_is_byte_identical() {
        let graph = wobbly();
        let levels: Vec<u32> = section_skeleton(&graph)
            .into_iter()
            .map(|(_, lvl)| lvl)
            .collect();
        let out = transform_graph(&graph, &[GraphChange::RelevelSections { levels }])
            .expect("no-op re-level succeeds");
        assert_eq!(bgraph_sha256(&out), bgraph_sha256(&graph));
    }

    // --- determinism -------------------------------------------------

    /// Same graph + same change-set, twice → byte-identical emit. The
    /// property any cached replay of a change-set depends on.
    #[test]
    fn same_change_set_twice_is_byte_identical() {
        let graph = wobbly();
        let change = GraphChange::RelevelSections {
            levels: vec![1, 1, 1, 2, 3],
        };
        let a = transform_graph(&graph, std::slice::from_ref(&change)).expect("first run");
        let b = transform_graph(&graph, std::slice::from_ref(&change)).expect("second run");

        assert_eq!(bgraph_sha256(&a), bgraph_sha256(&b));
        assert_eq!(emit_markdown(&a, &provenance()), emit_markdown(&b, &provenance()));
    }

    // --- the re-level itself ----------------------------------------

    /// Depths land exactly as given, and leaves follow their heading.
    #[test]
    fn depths_land_as_given_and_leaves_follow() {
        let graph = wobbly();
        // Appendix + its subsection demoted under the body chapters.
        let out = transform_graph(
            &graph,
            &[GraphChange::RelevelSections {
                levels: vec![1, 1, 1, 2, 3],
            }],
        )
        .expect("re-level succeeds");

        assert_eq!(
            levels_by_order(&out),
            vec![
                (0, "Deep Residual Learning".to_string(), 1),
                (1, "Abstract text.".to_string(), 2),
                (2, "Introduction".to_string(), 1),
                (3, "Body text.".to_string(), 2),
                (4, "Experiments".to_string(), 1),
                (5, "More body.".to_string(), 2),
                (6, "A. Appendix".to_string(), 2),
                (7, "Appendix body.".to_string(), 3),
                (8, "PASCAL VOC".to_string(), 3),
                (9, "Detail.".to_string(), 4),
            ]
        );
    }

    /// The re-level is a *rebuild*, so the tree re-parents: the demoted
    /// appendix hangs under the last body chapter, and breadcrumbs say so.
    #[test]
    fn relevel_reparents_and_recomputes_breadcrumbs() {
        let graph = wobbly();
        let out = transform_graph(
            &graph,
            &[GraphChange::RelevelSections {
                levels: vec![1, 1, 1, 2, 3],
            }],
        )
        .expect("re-level succeeds");

        let appendix = out
            .nodes
            .values()
            .find(|n| n.content.text == "A. Appendix")
            .expect("appendix node present");
        assert_eq!(
            appendix.location.semantic.breadcrumbs,
            vec![
                "Test Document".to_string(),
                "Experiments".to_string(),
                "A. Appendix".to_string()
            ],
            "the demoted appendix nests under the last body chapter"
        );

        let parent = out
            .nodes
            .get(&appendix.parent.expect("appendix has a parent"))
            .expect("parent resolves");
        assert_eq!(parent.content.text, "Experiments");
    }

    /// `outline_data` is re-derived from the *new* levels, rebased so the
    /// shallowest is 1 — the builder's own assembly, not a patch.
    #[test]
    fn outline_data_is_rederived_from_new_levels() {
        let graph = wobbly();
        let out = transform_graph(
            &graph,
            &[GraphChange::RelevelSections {
                levels: vec![1, 1, 1, 2, 3],
            }],
        )
        .expect("re-level succeeds");

        let outline = out
            .document_info
            .outline_data
            .as_ref()
            .expect("outline re-derived");
        let rows: Vec<(&str, u32, u32)> = outline
            .sections
            .iter()
            .map(|s| (s.title.as_str(), s.order, s.level))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("Deep Residual Learning", 0, 1),
                ("Introduction", 1, 1),
                ("Experiments", 2, 1),
                ("A. Appendix", 3, 2),
                ("PASCAL VOC", 4, 3),
            ]
        );
    }

    // --- preservation ------------------------------------------------

    /// Refs, metadata, physical locations, text and token counts survive
    /// the transform byte-exact — the grafted-graph guarantee.
    #[test]
    fn refs_metadata_and_physical_survive_the_transform() {
        let mut graph = wobbly();
        graph.document_info.document_metadata.author = Some("A. Author".to_string());
        graph.document_info.topology = Some("tree".to_string());

        // Graft-shaped refs onto a body paragraph, the way S3 does.
        let target_id = graph
            .nodes
            .values()
            .find(|n| n.content.text == "Body text.")
            .expect("body paragraph present")
            .id;
        {
            let node = graph.nodes.get_mut(&target_id).unwrap();
            node.internal_refs = vec![InternalRef {
                text: "Figure 1".to_string(),
                source_page: Some(2),
                source_bbox: Some(BoundingBox {
                    x: 1.0,
                    y: 2.0,
                    width: 3.0,
                    height: 4.0,
                }),
                target: InternalRefTarget::Page {
                    page: 7,
                    point: None,
                },
            }];
            node.external_refs = vec![ExternalRef {
                text: "arxiv".to_string(),
                source_page: Some(2),
                source_bbox: None,
                target: ExternalRefTarget::Uri {
                    url: "https://arxiv.org/abs/1512.03385".to_string(),
                },
            }];
        }
        let before = graph.nodes.get(&target_id).unwrap().clone();

        let out = transform_graph(
            &graph,
            &[GraphChange::RelevelSections {
                levels: vec![1, 1, 1, 2, 3],
            }],
        )
        .expect("re-level succeeds");

        // Document-level facts, verbatim.
        assert_eq!(
            out.document_info.document_metadata.title,
            graph.document_info.document_metadata.title
        );
        assert_eq!(
            out.document_info.document_metadata.author.as_deref(),
            Some("A. Author")
        );
        assert!(matches!(out.document_info.flow_type, FlowType::Fixed));
        assert_eq!(out.document_info.topology.as_deref(), Some("tree"));
        assert_eq!(out.document_info.kind, graph.document_info.kind);

        // Node-level facts, verbatim — even though the node was re-keyed.
        let after = out
            .nodes
            .values()
            .find(|n| n.content.text == "Body text.")
            .expect("body paragraph survives");
        assert_eq!(after.internal_refs.len(), 1);
        assert_eq!(after.internal_refs[0].text, "Figure 1");
        assert!(matches!(
            after.internal_refs[0].target,
            InternalRefTarget::Page { page: 7, .. }
        ));
        assert_eq!(after.external_refs.len(), 1);
        assert!(matches!(
            &after.external_refs[0].target,
            ExternalRefTarget::Uri { url } if url == "https://arxiv.org/abs/1512.03385"
        ));
        let (before_phys, after_phys) = (
            before.location.physical.as_ref().unwrap(),
            after.location.physical.as_ref().unwrap(),
        );
        assert_eq!(before_phys.page, after_phys.page);
        assert_eq!(before_phys.bounding_box.x, after_phys.bounding_box.x);
        assert_eq!(before_phys.bounding_box.y, after_phys.bounding_box.y);
        assert_eq!(before_phys.bounding_box.width, after_phys.bounding_box.width);
        assert_eq!(
            before_phys.bounding_box.height,
            after_phys.bounding_box.height
        );
        assert_eq!(before.token_count, after.token_count);
        assert_eq!(
            serde_json::to_value(&before.style_info).unwrap(),
            serde_json::to_value(&after.style_info).unwrap()
        );

        // Emission order is untouched — the change moves levels, not text.
        let texts: Vec<String> = levels_by_order(&out)
            .into_iter()
            .map(|(_, t, _)| t)
            .collect();
        let before_texts: Vec<String> = levels_by_order(&graph)
            .into_iter()
            .map(|(_, t, _)| t)
            .collect();
        assert_eq!(texts, before_texts);
    }

    // --- typed errors ------------------------------------------------

    #[test]
    fn wrong_level_count_is_a_typed_error() {
        let graph = wobbly();
        let err = transform_graph(
            &graph,
            &[GraphChange::RelevelSections {
                levels: vec![1, 1, 1],
            }],
        )
        .expect_err("short change-set rejected");
        assert_eq!(
            err,
            TransformError::SectionCountMismatch {
                expected: 5,
                got: 3
            }
        );
    }

    #[test]
    fn level_zero_is_a_typed_error() {
        let graph = wobbly();
        let err = transform_graph(
            &graph,
            &[GraphChange::RelevelSections {
                levels: vec![1, 0, 1, 2, 3],
            }],
        )
        .expect_err("level 0 rejected");
        assert_eq!(err, TransformError::LevelOutOfRange { index: 1, level: 0 });
    }

    #[test]
    fn level_above_the_cap_is_a_typed_error() {
        let graph = wobbly();
        let err = transform_graph(
            &graph,
            &[GraphChange::RelevelSections {
                levels: vec![1, 1, 1, 2, 5],
            }],
        )
        .expect_err("level 5 rejected");
        assert_eq!(err, TransformError::LevelOutOfRange { index: 4, level: 5 });
    }

    /// A rejected change-set leaves the input graph untouched — the
    /// transform borrows, it does not mutate.
    #[test]
    fn a_rejected_change_set_does_not_touch_the_input() {
        let graph = wobbly();
        let before = bgraph_sha256(&graph);
        let _ = transform_graph(
            &graph,
            &[GraphChange::RelevelSections { levels: vec![9; 5] }],
        );
        assert_eq!(bgraph_sha256(&graph), before);
    }

    /// A graph whose body `text_order` has a gap cannot be decomposed.
    #[test]
    fn text_order_drift_is_a_typed_error() {
        let mut graph = build(&[("Section", "A", 1), ("Paragraph", "B.", 2)]);
        let id = graph
            .nodes
            .values()
            .find(|n| n.content.text == "B.")
            .unwrap()
            .id;
        graph.nodes.get_mut(&id).unwrap().text_order = Some(7);
        let err = transform_graph(&graph, &[]).expect_err("drifted text_order rejected");
        assert_eq!(
            err,
            TransformError::TextOrderDrift {
                position: 1,
                text_order: 7
            }
        );
    }

    /// A body node with an unmappable `node_type` is a typed error, not
    /// a panic — the orphan `Message` sentinel included.
    #[test]
    fn unknown_node_type_is_a_typed_error() {
        let mut graph = build(&[("Section", "A", 1), ("Paragraph", "B.", 2)]);
        let id = graph
            .nodes
            .values()
            .find(|n| n.content.text == "B.")
            .unwrap()
            .id;
        graph.nodes.get_mut(&id).unwrap().node_type = "Message".to_string();
        let err = transform_graph(&graph, &[]).expect_err("unknown node_type rejected");
        assert_eq!(
            err,
            TransformError::UnknownNodeType {
                node_type: "Message".to_string()
            }
        );
    }

    // --- the documented wrinkle --------------------------------------

    /// Pins the known gap the module docs describe: `outline_data` is
    /// always re-derived from Sections, so a graph carrying a
    /// *source-native* outline (PDF `/Outlines` bookmarks, which are not
    /// a function of the node set) comes back with a section-derived one.
    /// Correct on the OCR channel; **not** a general-purpose behavior.
    /// If this test ever needs to change, the schema question in the
    /// module docs is the one to answer first.
    #[test]
    fn pdf_style_native_outline_is_rederived() {
        let mut graph = build(&[("Section", "Chapter One", 1), ("Paragraph", "Body.", 2)]);
        graph.document_info.outline_data = Some(BookmarkData {
            sections: vec![crate::types::BookmarkSection {
                title: "A bookmark the node set does not contain".to_string(),
                order: 0,
                level: 1,
            }],
        });

        let out = transform_graph(&graph, &[]).expect("identity transform succeeds");
        let titles: Vec<&str> = out
            .document_info
            .outline_data
            .as_ref()
            .expect("outline present")
            .sections
            .iter()
            .map(|s| s.title.as_str())
            .collect();
        assert_eq!(
            titles,
            vec!["Chapter One"],
            "native bookmarks are replaced by the section-derived outline (known wrinkle)"
        );
    }

    /// A graph with no Sections at all transforms cleanly (the change-set
    /// is empty by construction) and comes out with no outline.
    #[test]
    fn sectionless_graph_round_trips() {
        let graph = build(&[("Paragraph", "Just prose.", 1)]);
        let out = transform_graph(&graph, &[GraphChange::RelevelSections { levels: vec![] }])
            .expect("empty re-level on a sectionless graph succeeds");
        assert!(out.document_info.outline_data.is_none());
        assert_eq!(bgraph_sha256(&out), bgraph_sha256(&graph));
    }
}
