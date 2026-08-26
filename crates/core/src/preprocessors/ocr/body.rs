//! OCR body channel — Mistral OCR-4 JSON bytes → `DocumentGraph`.
//!
//! This is the OCR counterpart to [`super::super::docx::parse_docx`]:
//! a free function returning a built graph (no `Preprocessor` trait).
//! Where the DOCX channel walks OOXML, this module deserializes the
//! pinned `mist.json` payload and projects `pages[].blocks[]` — the
//! single source of truth — into the same `Vec<SemanticTreeElement>` →
//! [`GraphBuilder::build_graph_deterministic`] shape.
//!
//! ## Block → variant mapping (design-arc table, fold-first lean)
//!
//! | block `type`                                      | variant             |
//! |---------------------------------------------------|---------------------|
//! | `text`                                            | Paragraph           |
//! | `title`                                           | Section             |
//! | `list`                                            | List                |
//! | `table`                                           | Table               |
//! | `code`                                            | CodeBlock           |
//! | `header` / `footer`                               | Header / Footer     |
//! | `equation`                                        | Equation (1.1.0)    |
//! | `caption` / `references` / `aside_text` / `signature` | Paragraph (fold) |
//! | `image`                                           | *skipped*           |
//! | anything else                                     | Paragraph (fold)    |
//!
//! Folds carry the body verbatim — promotion to dedicated variants waits
//! on evidence. `code` content is **verbatim monospace, not necessarily
//! source code** (rfc-quic's ASCII protocol diagrams arrive as `code`);
//! no language inference. `image` blocks are skipped (no Figure variant
//! in S1).
//!
//! ## Fixed-flow physical location
//!
//! `flow_type = Fixed`; every node carries
//! `physical: Some(PhysicalLocation)` from its block bbox. OCR bboxes
//! are absolute pixels at `dimensions.dpi`, top-left origin — the same
//! origin convention the PDF channel's Tika bboxes use — so the
//! conversion is a pure scale: `pdf_points = pixels * 72 / dpi` (the S3
//! bbox join between the two arms depends on unit agreement). A missing
//! or zero dpi degrades to scale 1.0 (raw pixels), never a panic.
//! `text_order` is block sequence across pages.
//!
//! ## Section hierarchy
//!
//! `title` blocks carry no explicit level field, but every observed
//! title's `content` arrives **with its markdown heading prefix inline**
//! (`"## Attention Is All You Need"`, 234/234 titles across both
//! fixtures) — so the primary inference is a direct `#`-count parse of
//! the content itself. Fallbacks, in order: match the bare title text
//! against a `#`-prefixed heading line in the same page's `markdown`
//! field; else flatten to the depth of the last successfully-leveled
//! title (level 1 before any). Multi-line titles (rfc-quic's
//! `"# RFC 9000\n## QUIC: …"`) split into one Section per heading line —
//! a single Section body must stay single-line on the bgraph.md wire.
//! Degrades gracefully; never panics.
//!
//! Those per-page levels are then subject to the CR-98 normalization
//! post-pass ([`super::numbering`]): where a coherent majority of the
//! Section titles carry a numbering scheme (`3.2.1` / `A.1`), the
//! numbering's rank replaces the OCR level — see
//! [`normalize_outline_by_numbering`]. Unnumbered documents pass through
//! byte-identical.
//!
//! ## ParseIdentity / provenance
//!
//! Returns [`ParseIdentity::Verified`] (same "we parsed successfully"
//! signal as DOCX/MD). Provenance: `source_format: "ocr"`,
//! `source_sha256` = sha256 of the JSON bytes, `config_hash: "none"` (no
//! tunables; the model id lives *inside* the source bytes, so it is
//! covered by `source_sha256` at this boundary).

use sha2::{Digest, Sha256};

use crate::graphs::builder::GraphBuilder;
use crate::graphs::node_id::NodeIdGenerator;
use crate::tokens::estimate_token_count;
use crate::types::{
    BookmarkData, BookmarkSection, BoundingBox, FlowType, PhysicalLocation, ParseProvenance,
    SemanticElementType, SemanticTreeElement,
};

use super::super::md::types::{ParseError, ParseIdentity, ParseOptions, ParseResult};
use super::payload::{OcrBlock, OcrDocument};

/// Parse a Mistral OCR-4 JSON byte buffer into a `DocumentGraph`.
///
/// `opts` is currently unused — the OCR path has no strict-vs-drift
/// distinction (no embedded `bgraph_sha256` to verify against). The
/// argument is present for API symmetry with
/// [`super::super::docx::parse_docx`].
///
/// Returns [`ParseError::MalformedOcr`] if the bytes are not valid JSON
/// or the JSON does not carry the OCR document shape (a top-level
/// `pages` array).
pub fn parse_ocr(bytes: &[u8], _opts: ParseOptions) -> Result<ParseResult, ParseError> {
    // 1. Deserialize (tolerant: only `pages` is required — the shape
    //    gate that keeps a bgraph.json from parsing as OCR).
    let doc: OcrDocument = serde_json::from_slice(bytes)
        .map_err(|e| ParseError::MalformedOcr(format!("not an OCR-4 JSON payload: {e}")))?;

    // 2. Project blocks → elements.
    let mut projection = project_blocks(&doc);

    // 2b. CR-98: numbering-rank outline normalization. Runs after
    //     block→variant mapping and BEFORE the graph build + outline
    //     assembly, so Section node depths, `SemanticLocation`
    //     path/depth/breadcrumbs, and `outline_data` are all derived by
    //     the existing builder from the already-normalized levels
    //     (no post-build patching). No-op when the coherence gate fails.
    normalize_outline_by_numbering(&mut projection);

    // 3. Provenance. `config_hash = "none"` (no tunables — like DOCX/MD).
    let provenance = ParseProvenance {
        bragi_version: crate::VERSION.to_string(),
        source_format: "ocr".to_string(),
        source_sha256: sha256_hex(bytes),
        config_hash: "none".to_string(),
    };
    let id_gen = NodeIdGenerator::new();

    // 4. Build the graph (the builder asserts `text_order == vec pos`;
    //    `project_blocks` pushes in order).
    let mut graph = GraphBuilder::new()
        .build_graph_deterministic(projection.elements, &id_gen)
        .map_err(|e| ParseError::MalformedOcr(format!("graph build failed: {e}")))?;

    // 5. Populate fields the builder doesn't. OCR is a raster of a fixed
    //    layout — physical location is meaningful on every node.
    graph.document_info.flow_type = FlowType::Fixed;
    graph.document_info.outline_data = build_outline(&projection.outline);

    //    Metadata: canonical fields all-None by design (S2 grafts them
    //    from the native arm); the `ocr:` namespace carries the run facts.
    let extractor = super::OcrMetadataExtractor::new(
        doc.model.clone(),
        doc.usage_info.as_ref().and_then(|u| u.pages_processed),
        doc.usage_info.as_ref().and_then(|u| u.doc_size_bytes),
        projection.dpi,
    );
    graph.document_info.document_metadata =
        crate::preprocessors::metadata::extract_document_metadata(&extractor, &());

    // 6. Canonical post-build sequence (mirrors the DOCX/MD paths).
    graph.compute_breadcrumbs();

    Ok(ParseResult {
        graph,
        identity: ParseIdentity::Verified,
        provenance,
    })
}

/// The result of projecting `pages[].blocks[]`.
struct Projection {
    elements: Vec<SemanticTreeElement>,
    /// `(heading text, level)` of every Section, in emission order —
    /// feeds `outline_data`.
    outline: Vec<(String, u32)>,
    /// First observed `dimensions.dpi` — feeds `ocr:` metadata.
    dpi: Option<u32>,
    /// Skipped `image` blocks (no Figure variant in S1).
    #[allow(dead_code)] // read by tests; reported in the AAR.
    images_skipped: usize,
    /// Blocks skipped for empty/whitespace-only content (C-7a guard).
    #[allow(dead_code)]
    empty_skipped: usize,
}

/// Walk every page's `blocks[]` in order, producing the element vec with
/// `text_order = 0..N` across pages.
fn project_blocks(doc: &OcrDocument) -> Projection {
    let mut elements: Vec<SemanticTreeElement> = Vec::new();
    let mut outline: Vec<(String, u32)> = Vec::new();
    let mut first_dpi: Option<u32> = None;
    let mut images_skipped = 0usize;
    let mut empty_skipped = 0usize;

    // Depth of the currently-open Section (0 before any) — non-Section
    // leaves attach one below it (same convention as the DOCX walk).
    let mut current_section_level: u32 = 0;
    // Depth of the last successfully-leveled title — the flatten target
    // for titles whose level cannot be inferred (0 → default 1).
    let mut last_title_level: u32 = 0;

    for (page_pos, page) in doc.pages.iter().enumerate() {
        // 1-indexed page number, matching the PDF channel's convention.
        let page_number = page.index.map(|i| i + 1).unwrap_or(page_pos as u32 + 1);
        let page_dpi = page.dimensions.as_ref().and_then(|d| d.dpi);
        if first_dpi.is_none() {
            first_dpi = page_dpi;
        }
        // Pixel → PDF-point scale. Missing/zero dpi degrades to 1.0.
        let scale = match page_dpi {
            Some(dpi) if dpi > 0 => 72.0 / dpi as f32,
            _ => 1.0,
        };
        let page_markdown = page.markdown.as_deref().unwrap_or("");

        for block in page.blocks.iter().flatten() {
            let block_type = block.block_type.as_deref().unwrap_or("text");
            if block_type == "image" {
                // No Figure variant in S1 — skip, counted for the AAR.
                images_skipped += 1;
                continue;
            }
            let content = block.content.as_deref().unwrap_or("");
            if content.trim().is_empty() {
                // C-7a: every wire variant requires a non-empty body.
                empty_skipped += 1;
                continue;
            }
            let physical = block_physical(block, page_number, scale);

            if block_type == "title" {
                project_title_block(
                    content,
                    page_markdown,
                    &physical,
                    &mut elements,
                    &mut outline,
                    &mut current_section_level,
                    &mut last_title_level,
                );
                continue;
            }

            let element_type = map_block_type(block_type);
            let text = content.trim().to_string();
            elements.push(
                SemanticTreeElement {
                    token_count: estimate_token_count(&text),
                    text,
                    element_type,
                    hierarchy_level: current_section_level + 1,
                    text_order: elements.len() as u32,
                    physical_location: Some(physical),
                    style: None,
                    internal_refs: vec![],
                    external_refs: vec![],
                    confidence: 0,
                }
                .validate(),
            );
        }
    }

    Projection {
        elements,
        outline,
        dpi: first_dpi,
        images_skipped,
        empty_skipped,
    }
}

/// CR-98 post-pass: where a coherent majority of the document's Section
/// titles carry a numbering scheme, the numbering's rank (component
/// count of `3.2.1` / `A.1`, capped at
/// [`super::numbering::MAX_OUTLINE_DEPTH`]) replaces Mistral's per-page
/// heading levels. The decision core is
/// [`super::numbering::normalized_levels`]; this function applies its
/// verdict to the projection:
///
/// - each Section element (and its `outline` twin) takes its normalized
///   level, in emission order;
/// - each non-Section leaf re-attaches under the *normalized* level of
///   the last Section before it (`level + 1`; `1` before any Section) —
///   the same attachment rule `project_blocks` used with the raw levels,
///   so node depths and the outline stay in agreement.
///
/// When the gate fails (`None`) the projection is left untouched —
/// unnumbered documents come out byte-identical to pre-CR-98 output.
fn normalize_outline_by_numbering(projection: &mut Projection) {
    let titles: Vec<&str> = projection.outline.iter().map(|(t, _)| t.as_str()).collect();
    let s1_levels: Vec<u32> = projection.outline.iter().map(|(_, lvl)| *lvl).collect();
    let Some(levels) = super::numbering::normalized_levels(&titles, &s1_levels) else {
        return;
    };

    for ((_, lvl), new) in projection.outline.iter_mut().zip(&levels) {
        *lvl = *new;
    }

    let mut section_idx = 0usize;
    let mut current_section_level: u32 = 0;
    for element in &mut projection.elements {
        if element.element_type == SemanticElementType::Section {
            element.hierarchy_level = levels[section_idx];
            current_section_level = levels[section_idx];
            section_idx += 1;
        } else {
            element.hierarchy_level = current_section_level + 1;
        }
    }
}

/// Map a non-title, non-image block type to its variant. Unknown types
/// fold to Paragraph (verbatim body) — the same fold-first lean as
/// `caption`/`references`/`aside_text`/`signature`, so a payload
/// carrying a block type this build has never seen degrades to prose
/// rather than erroring.
fn map_block_type(block_type: &str) -> SemanticElementType {
    match block_type {
        "text" => SemanticElementType::Paragraph,
        "list" => SemanticElementType::List,
        "table" => SemanticElementType::Table,
        // Verbatim monospace block — not necessarily source code
        // (rfc-quic's ASCII protocol diagrams arrive as `code`). No
        // language inference.
        "code" => SemanticElementType::CodeBlock,
        "equation" => SemanticElementType::Equation,
        "header" => SemanticElementType::Header,
        "footer" => SemanticElementType::Footer,
        // Fold-first: promote later on evidence.
        "caption" | "references" | "aside_text" | "signature" => SemanticElementType::Paragraph,
        // Unknown → fold to Paragraph, never an error.
        _ => SemanticElementType::Paragraph,
    }
}

/// Project one `title` block into Section element(s).
///
/// Every line of the content is treated as a candidate heading (observed
/// multi-line titles are stacked headings, e.g. `"# RFC 9000\n## QUIC:
/// …"`). Level inference per line, in order: inline `#`-prefix →
/// page-markdown heading match → flatten to the last inferred level
/// (default 1). Lines that end up with empty text are dropped.
fn project_title_block(
    content: &str,
    page_markdown: &str,
    physical: &PhysicalLocation,
    elements: &mut Vec<SemanticTreeElement>,
    outline: &mut Vec<(String, u32)>,
    current_section_level: &mut u32,
    last_title_level: &mut u32,
) {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (level, text) = match split_heading_prefix(line) {
            Some((level, rest)) => (level, rest),
            None => match level_from_page_markdown(page_markdown, line) {
                Some(level) => (level, line),
                None => ((*last_title_level).max(1), line),
            },
        };
        let text = text.trim();
        if text.is_empty() {
            continue;
        }
        *last_title_level = level;
        *current_section_level = level;
        outline.push((text.to_string(), level));
        elements.push(
            SemanticTreeElement {
                text: text.to_string(),
                element_type: SemanticElementType::Section,
                hierarchy_level: level,
                text_order: elements.len() as u32,
                physical_location: Some(physical.clone()),
                style: None,
                token_count: estimate_token_count(text),
                internal_refs: vec![],
                external_refs: vec![],
                confidence: 0,
            }
            .validate(),
        );
    }
}

/// `"## Foo"` → `Some((2, "Foo"))`. Requires at least one `#` followed
/// by whitespace (or nothing but `#`s is rejected — no text). Returns
/// `None` for non-heading lines.
fn split_heading_prefix(line: &str) -> Option<(u32, &str)> {
    let hashes = line.bytes().take_while(|&b| b == b'#').count();
    if hashes == 0 {
        return None;
    }
    let rest = &line[hashes..];
    if !rest.starts_with(' ') && !rest.starts_with('\t') {
        return None;
    }
    Some((hashes as u32, rest.trim()))
}

/// Fallback level inference: find a heading line in the page's
/// `markdown` whose text equals `title` (trimmed) and return its
/// `#`-count. `None` when no heading matches.
fn level_from_page_markdown(page_markdown: &str, title: &str) -> Option<u32> {
    for line in page_markdown.lines() {
        if let Some((level, text)) = split_heading_prefix(line.trim()) {
            if text == title {
                return Some(level);
            }
        }
    }
    None
}

/// Convert a block's pixel corner coords to a PDF-point
/// `PhysicalLocation` (top-left origin both sides; pure scale). Missing
/// coords default to 0; a bottom-right that precedes top-left clamps the
/// extent to 0 rather than going negative.
fn block_physical(block: &OcrBlock, page_number: u32, scale: f32) -> PhysicalLocation {
    let tlx = block.top_left_x.unwrap_or(0.0);
    let tly = block.top_left_y.unwrap_or(0.0);
    let brx = block.bottom_right_x.unwrap_or(tlx);
    let bry = block.bottom_right_y.unwrap_or(tly);
    PhysicalLocation {
        page: page_number,
        bounding_box: BoundingBox {
            x: tlx * scale,
            y: tly * scale,
            width: (brx - tlx).max(0.0) * scale,
            height: (bry - tly).max(0.0) * scale,
        },
    }
}

/// Assemble `outline_data` from the emitted Sections: levels rebased so
/// the shallowest is 1 (same semantic as the DOCX ToC projection and the
/// PDF `/Outlines` depth), `order` = 0-based emission sequence. `None`
/// when the document has no Sections.
fn build_outline(sections: &[(String, u32)]) -> Option<BookmarkData> {
    let min_level = sections.iter().map(|(_, lvl)| *lvl).min()?;
    Some(BookmarkData {
        sections: sections
            .iter()
            .enumerate()
            .map(|(i, (title, lvl))| BookmarkSection {
                title: title.clone(),
                order: i as u32,
                level: lvl - min_level + 1,
            })
            .collect(),
    })
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

// =============================================================================
// Tests.
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> ParseOptions {
        ParseOptions::default()
    }

    /// Wrap page JSON fragments into a minimal payload.
    fn payload(pages_json: &str) -> Vec<u8> {
        format!(
            r#"{{"pages":[{pages_json}],"model":"mistral-ocr-4-0","usage_info":{{"pages_processed":1,"doc_size_bytes":100}}}}"#
        )
        .into_bytes()
    }

    fn block(t: &str, content: &str, y: f32) -> String {
        format!(
            r#"{{"top_left_x":10,"top_left_y":{y},"bottom_right_x":110,"bottom_right_y":{},"content":{},"type":"{t}"}}"#,
            y + 20.0,
            serde_json::to_string(content).unwrap(),
        )
    }

    fn nodes_of_type<'a>(
        graph: &'a crate::types::DocumentGraph,
        t: &str,
    ) -> Vec<&'a crate::types::DocumentNode> {
        let mut v: Vec<_> = graph.nodes.values().filter(|n| n.node_type == t).collect();
        v.sort_by_key(|n| n.text_order);
        v
    }

    // --- block mapping ------------------------------------------------

    #[test]
    fn maps_each_block_type_to_its_variant() {
        let blocks = [
            block("text", "prose", 10.0),
            block("title", "# Heading", 30.0),
            block("list", "- a\n- b", 50.0),
            block("table", "| a | b |\n| - | - |", 70.0),
            block("code", "verbatim  block", 90.0),
            block("equation", "$$x^2 \\tag{1}$$", 110.0),
            block("header", "running head", 130.0),
            block("footer", "page 1", 150.0),
        ]
        .join(",");
        let json = payload(&format!(
            r#"{{"index":0,"dimensions":{{"dpi":72}},"blocks":[{blocks}]}}"#
        ));
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        for (ty, expected) in [
            ("Paragraph", 1),
            ("Section", 1),
            ("List", 1),
            ("Table", 1),
            ("CodeBlock", 1),
            ("Equation", 1),
            ("Header", 1),
            ("Footer", 1),
        ] {
            assert_eq!(nodes_of_type(&graph, ty).len(), expected, "{ty} count");
        }
        // Equation body is verbatim, tag intact.
        assert_eq!(
            nodes_of_type(&graph, "Equation")[0].content.text,
            "$$x^2 \\tag{1}$$"
        );
    }

    #[test]
    fn folds_caption_references_aside_signature_and_unknown_to_paragraph() {
        let blocks = [
            block("caption", "Figure 1: caption", 10.0),
            block("references", "[1] A citation.", 30.0),
            block("aside_text", "an aside", 50.0),
            block("signature", "signed, X", 70.0),
            block("mystery_future_type", "unknown carrier", 90.0),
        ]
        .join(",");
        let json = payload(&format!(r#"{{"index":0,"blocks":[{blocks}]}}"#));
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        assert_eq!(nodes_of_type(&graph, "Paragraph").len(), 5);
    }

    #[test]
    fn skips_image_blocks_and_empty_content() {
        let blocks = [
            block("text", "kept", 10.0),
            block("image", "![img-0.jpeg](img-0.jpeg)", 30.0),
            block("text", "   ", 50.0),
            block("text", "", 70.0),
        ]
        .join(",");
        let json = payload(&format!(r#"{{"index":0,"blocks":[{blocks}]}}"#));
        let doc: OcrDocument = serde_json::from_slice(&json).unwrap();
        let projection = project_blocks(&doc);
        assert_eq!(projection.elements.len(), 1);
        assert_eq!(projection.images_skipped, 1);
        assert_eq!(projection.empty_skipped, 2);
    }

    // --- null tolerance ----------------------------------------------

    #[test]
    fn tolerates_null_enrichment_and_missing_fields() {
        // Page with null markdown/dimensions, block with only content —
        // parses; physical degrades to zeros at scale 1.0.
        let json = br#"{"pages":[{"markdown":null,"dimensions":null,"confidence_scores":null,
            "images":null,"tables":null,"hyperlinks":null,"header":null,"footer":null,
            "blocks":[{"content":"hello","type":"text","confidence_scores":null}]}]}"#;
        let graph = parse_ocr(json, opts()).expect("parses").graph;
        let paras = nodes_of_type(&graph, "Paragraph");
        assert_eq!(paras.len(), 1);
        let phys = paras[0].location.physical.as_ref().expect("physical set");
        assert_eq!(phys.page, 1);
        assert_eq!(phys.bounding_box.x, 0.0);
        assert_eq!(phys.bounding_box.width, 0.0);
    }

    #[test]
    fn non_ocr_json_is_malformed() {
        for bad in [
            &br#"{"schema_version":"1.1.0","nodes":[]}"#[..], // bgraph.json shape
            &b"not json"[..],
            &br#"[1,2,3]"#[..],
        ] {
            assert!(
                matches!(parse_ocr(bad, opts()), Err(ParseError::MalformedOcr(_))),
                "{:?} should be MalformedOcr",
                String::from_utf8_lossy(bad)
            );
        }
    }

    // --- bbox conversion ---------------------------------------------

    #[test]
    fn converts_pixel_bbox_to_pdf_points_at_dpi() {
        // dpi 93 → scale 72/93. Pixel (93, 186, w 93, h 46.5) →
        // points (72, 144, 72, 36).
        let json = payload(
            r#"{"index":0,"dimensions":{"dpi":93,"width":791,"height":1023},
                "blocks":[{"top_left_x":93,"top_left_y":186,"bottom_right_x":186,
                           "bottom_right_y":232.5,"content":"hi","type":"text"}]}"#,
        );
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        let n = &nodes_of_type(&graph, "Paragraph")[0];
        let bb = &n.location.physical.as_ref().unwrap().bounding_box;
        assert!((bb.x - 72.0).abs() < 1e-4);
        assert!((bb.y - 144.0).abs() < 1e-4);
        assert!((bb.width - 72.0).abs() < 1e-4);
        assert!((bb.height - 36.0).abs() < 1e-4);
    }

    #[test]
    fn zero_or_missing_dpi_degrades_to_raw_pixels() {
        let json = payload(
            r#"{"index":0,"dimensions":{"dpi":0},
                "blocks":[{"top_left_x":10,"top_left_y":20,"bottom_right_x":30,
                           "bottom_right_y":50,"content":"hi","type":"text"}]}"#,
        );
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        let bb = &nodes_of_type(&graph, "Paragraph")[0]
            .location
            .physical
            .as_ref()
            .unwrap()
            .bounding_box;
        assert_eq!(bb.x, 10.0);
        assert_eq!(bb.height, 30.0);
    }

    #[test]
    fn page_numbers_are_one_indexed_from_page_index() {
        let json = payload(&format!(
            r#"{{"index":0,"blocks":[{}]}},{{"index":1,"blocks":[{}]}}"#,
            block("text", "p1", 10.0),
            block("text", "p2", 10.0),
        ));
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        let paras = nodes_of_type(&graph, "Paragraph");
        assert_eq!(paras[0].location.physical.as_ref().unwrap().page, 1);
        assert_eq!(paras[1].location.physical.as_ref().unwrap().page, 2);
    }

    // --- section level inference -------------------------------------

    #[test]
    fn title_level_from_inline_hash_prefix() {
        let blocks = [
            block("title", "## Doc Title", 10.0),
            block("title", "### Abstract", 30.0),
            block("text", "body", 50.0),
            block("title", "# 1 Introduction", 70.0),
        ]
        .join(",");
        let json = payload(&format!(r#"{{"index":0,"blocks":[{blocks}]}}"#));
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        let sections = nodes_of_type(&graph, "Section");
        assert_eq!(sections.len(), 3);
        assert_eq!(sections[0].content.text, "Doc Title");
        assert_eq!(sections[0].location.semantic.depth, 2);
        assert_eq!(sections[1].content.text, "Abstract");
        assert_eq!(sections[1].location.semantic.depth, 3);
        assert_eq!(sections[2].content.text, "1 Introduction");
        assert_eq!(sections[2].location.semantic.depth, 1);
        // The body paragraph attaches under the open "### Abstract"
        // section (depth 3 + 1).
        let para = &nodes_of_type(&graph, "Paragraph")[0];
        assert_eq!(para.location.semantic.depth, 4);
        // Outline populated + rebased (shallowest → 1).
        let outline = graph.document_info.outline_data.as_ref().unwrap();
        assert_eq!(outline.sections.len(), 3);
        assert_eq!(outline.sections[0].title, "Doc Title");
        assert_eq!(outline.sections[0].level, 2);
        assert_eq!(outline.sections[2].level, 1);
    }

    #[test]
    fn multiline_title_splits_into_stacked_sections() {
        // rfc-quic shape: one title block carrying two heading lines.
        let blocks = [block("title", "# RFC 9000\n## QUIC: A Transport", 10.0)].join(",");
        let json = payload(&format!(r#"{{"index":0,"blocks":[{blocks}]}}"#));
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        let sections = nodes_of_type(&graph, "Section");
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].content.text, "RFC 9000");
        assert_eq!(sections[0].location.semantic.depth, 1);
        assert_eq!(sections[1].content.text, "QUIC: A Transport");
        assert_eq!(sections[1].location.semantic.depth, 2);
    }

    #[test]
    fn bare_title_matches_page_markdown_heading() {
        // Title content without a hash prefix — level comes from the
        // page markdown's matching heading line.
        let page = format!(
            r#"{{"index":0,"markdown":"intro\n\n### Methods\n\nbody","blocks":[{}]}}"#,
            block("title", "Methods", 10.0)
        );
        let json = payload(&page);
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        let sections = nodes_of_type(&graph, "Section");
        assert_eq!(sections[0].content.text, "Methods");
        assert_eq!(sections[0].location.semantic.depth, 3);
    }

    #[test]
    fn unmatched_title_flattens_to_last_matched_level_never_panics() {
        let blocks = [
            block("title", "## Known", 10.0),
            block("title", "Unmatched Title", 30.0), // no prefix, no markdown match
        ]
        .join(",");
        let json = payload(&format!(r#"{{"index":0,"markdown":null,"blocks":[{blocks}]}}"#));
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        let sections = nodes_of_type(&graph, "Section");
        assert_eq!(sections[1].content.text, "Unmatched Title");
        assert_eq!(sections[1].location.semantic.depth, 2, "flat at last match's level");
    }

    #[test]
    fn unmatched_first_title_defaults_to_level_one() {
        let json = payload(&format!(
            r#"{{"index":0,"blocks":[{}]}}"#,
            block("title", "Bare First Title", 10.0)
        ));
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        assert_eq!(
            nodes_of_type(&graph, "Section")[0].location.semantic.depth,
            1
        );
    }

    // --- CR-98: numbering-rank outline normalization ------------------

    #[test]
    fn numbering_rank_overrides_noisy_per_page_levels() {
        // The attention defect in miniature: same-rank chapters land on
        // different markdown levels page to page (no cross-page memory).
        // The numbering rank must win.
        let page1 = [
            block("title", "## Attention Is All You Need", 10.0),
            block("title", "### Abstract", 30.0),
            block("title", "# 1 Introduction", 50.0),
            block("text", "intro body", 70.0),
            block("title", "# 2 Background", 90.0),
        ]
        .join(",");
        let page2 = [
            // Per-page amnesia: chapter 3 arrives one level deep.
            block("title", "## 3 Model Architecture", 10.0),
            block("title", "### 3.1 Encoder and Decoder Stacks", 30.0),
            block("text", "encoder body", 50.0),
            block("title", "### 3.2 Attention", 70.0),
            block("title", "#### 3.2.1 Scaled Dot-Product Attention", 90.0),
        ]
        .join(",");
        let page3 = [
            block("title", "## A. Training Details", 10.0),
            block("title", "### A.1 Optimizer", 30.0),
            block("title", "## References", 50.0),
        ]
        .join(",");
        let json = payload(&format!(
            r#"{{"index":0,"blocks":[{page1}]}},{{"index":1,"blocks":[{page2}]}},{{"index":2,"blocks":[{page3}]}}"#
        ));
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        let sections = nodes_of_type(&graph, "Section");
        let got: Vec<(String, u32)> = sections
            .iter()
            .map(|n| (n.content.text.clone(), n.location.semantic.depth))
            .collect();
        let want: Vec<(&str, u32)> = vec![
            ("Attention Is All You Need", 1), // leading title: pinned to the top
            ("Abstract", 1),                  // unnumbered → depth 1
            ("1 Introduction", 1),
            ("2 Background", 1),
            ("3 Model Architecture", 1), // rank beats the noisy `##`
            ("3.1 Encoder and Decoder Stacks", 2),
            ("3.2 Attention", 2),
            ("3.2.1 Scaled Dot-Product Attention", 3),
            ("A. Training Details", 1), // appendix letter = rank 1
            ("A.1 Optimizer", 2),
            ("References", 1), // unnumbered → depth 1
        ];
        assert_eq!(
            got,
            want.into_iter()
                .map(|(t, l)| (t.to_string(), l))
                .collect::<Vec<_>>()
        );
        // Leaves re-attach under the NORMALIZED section level.
        let paras = nodes_of_type(&graph, "Paragraph");
        assert_eq!(paras[0].content.text, "intro body");
        assert_eq!(paras[0].location.semantic.depth, 2); // under depth-1 "1 Introduction"
        assert_eq!(paras[1].content.text, "encoder body");
        assert_eq!(paras[1].location.semantic.depth, 3); // under depth-2 "3.1"
        // outline_data agrees with the node depths (min level is 1, so
        // the rebase is the identity).
        let outline = graph.document_info.outline_data.as_ref().unwrap();
        assert_eq!(outline.sections.len(), 11);
        assert_eq!(outline.sections[0].level, 1);
        assert_eq!(outline.sections[4].level, 1);
        assert_eq!(outline.sections[7].level, 3);
    }

    #[test]
    fn rfc_shape_trailing_dot_numbering_normalizes() {
        // rfc-quic shape: stacked title block + `N.` / `N.M.` numbering.
        let blocks = [
            block("title", "# RFC 9000\n## QUIC: A UDP-Based Multiplexed Transport", 10.0),
            block("title", "# 1. Introduction", 30.0),
            block("title", "## 1.1. Document Structure", 50.0),
            block("title", "# 2. Streams", 70.0),
            block("title", "## 2.1. Stream Types and Identifiers", 90.0),
            block("title", "# 3. Flow Control", 110.0),
        ]
        .join(",");
        let json = payload(&format!(r#"{{"index":0,"blocks":[{blocks}]}}"#));
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        let sections = nodes_of_type(&graph, "Section");
        let depths: Vec<u32> = sections.iter().map(|n| n.location.semantic.depth).collect();
        // "RFC 9000" is the leading title (RFC is an acronym, not
        // numbering) — pinned to depth 1. The unnumbered subtitle
        // attaches at depth 1; ranks carry the rest.
        assert_eq!(depths, vec![1, 1, 1, 2, 1, 2, 1]);
    }

    #[test]
    fn deep_numbering_folds_to_the_cap_through_the_full_parse() {
        let blocks = [
            block("title", "# 1 Alpha", 10.0),
            block("title", "## 1.1 Beta", 30.0),
            block("title", "### 1.1.1 Gamma", 50.0),
            block("title", "#### 1.1.1.1 Delta", 70.0),
            block("title", "##### 1.1.1.1.1 Epsilon", 90.0),
            block("text", "leaf under the fold", 110.0),
        ]
        .join(",");
        let json = payload(&format!(r#"{{"index":0,"blocks":[{blocks}]}}"#));
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        let sections = nodes_of_type(&graph, "Section");
        let depths: Vec<u32> = sections.iter().map(|n| n.location.semantic.depth).collect();
        assert_eq!(depths, vec![1, 2, 3, 4, 4], "rank 5 folds to the cap");
        let para = &nodes_of_type(&graph, "Paragraph")[0];
        assert_eq!(para.location.semantic.depth, 5);
    }

    #[test]
    fn unnumbered_document_projection_is_untouched() {
        // Gate negative: no numbering anywhere — the pass must leave the
        // projection exactly as S1 built it (levels, outline, order).
        let blocks = [
            block("title", "## Overview", 10.0),
            block("text", "prose", 30.0),
            block("title", "### Details", 50.0),
            block("title", "## Wrap-Up", 70.0),
        ]
        .join(",");
        let json = payload(&format!(r#"{{"index":0,"blocks":[{blocks}]}}"#));
        let doc: OcrDocument = serde_json::from_slice(&json).unwrap();
        let mut projection = project_blocks(&doc);
        let levels_before: Vec<u32> = projection
            .elements
            .iter()
            .map(|e| e.hierarchy_level)
            .collect();
        let outline_before = projection.outline.clone();
        normalize_outline_by_numbering(&mut projection);
        let levels_after: Vec<u32> = projection
            .elements
            .iter()
            .map(|e| e.hierarchy_level)
            .collect();
        assert_eq!(levels_before, levels_after);
        assert_eq!(outline_before, projection.outline);
    }

    // --- flow / provenance / metadata / identity ----------------------

    #[test]
    fn fixed_flow_provenance_and_identity() {
        let json = payload(&format!(
            r#"{{"index":0,"dimensions":{{"dpi":93}},"blocks":[{}]}}"#,
            block("text", "hello", 10.0)
        ));
        let result = parse_ocr(&json, opts()).expect("parses");
        assert!(matches!(
            result.graph.document_info.flow_type,
            FlowType::Fixed
        ));
        assert!(matches!(result.identity, ParseIdentity::Verified));
        assert_eq!(result.provenance.source_format, "ocr");
        assert_eq!(result.provenance.config_hash, "none");
        assert_eq!(result.provenance.source_sha256, sha256_hex(&json));
        assert_eq!(result.provenance.source_sha256.len(), 64);

        let md = &result.graph.document_info.document_metadata;
        assert!(md.title.is_none() && md.author.is_none() && md.created.is_none());
        let ocr = md.ocr.as_ref().expect("ocr namespace");
        assert_eq!(ocr.model.as_deref(), Some("mistral-ocr-4-0"));
        assert_eq!(ocr.pages_processed, Some(1));
        assert_eq!(ocr.dpi, Some(93));
    }

    #[test]
    fn text_order_is_sequential_across_pages_and_physical_everywhere() {
        let json = payload(&format!(
            r#"{{"index":0,"blocks":[{},{}]}},{{"index":1,"blocks":[{}]}}"#,
            block("title", "# A", 10.0),
            block("text", "one", 30.0),
            block("text", "two", 10.0),
        ));
        let graph = parse_ocr(&json, opts()).expect("parses").graph;
        let mut body: Vec<_> = graph
            .nodes
            .values()
            .filter(|n| n.node_type != "Document")
            .collect();
        body.sort_by_key(|n| n.text_order);
        assert_eq!(body.len(), 3);
        for (i, n) in body.iter().enumerate() {
            assert_eq!(n.text_order, Some(i as u32));
            assert!(
                n.location.physical.is_some(),
                "every OCR node carries physical; {} missing it",
                n.node_type
            );
        }
    }

    // --- wire round-trip ----------------------------------------------

    #[test]
    fn ocr_graph_roundtrips_verified_through_bgraph_md() {
        // The full-channel loop: parse OCR → emit bgraph.md → parse back
        // → Verified (covers the Equation fence tag both directions).
        let blocks = [
            block("title", "# Doc", 10.0),
            block("text", "prose body", 30.0),
            block("equation", "$$E = mc^2 \\tag{1}$$", 50.0),
            block("code", "line one\n  indented two", 70.0),
            block("header", "chrome", 90.0),
        ]
        .join(",");
        let json = payload(&format!(
            r#"{{"index":0,"dimensions":{{"dpi":93}},"blocks":[{blocks}]}}"#
        ));
        let result = parse_ocr(&json, opts()).expect("parses");
        let md = crate::graphs::serialization::markdown::emit_markdown(
            &result.graph,
            &result.provenance,
        );
        let back = crate::preprocessors::md::parse_markdown(&md, ParseOptions::default())
            .expect("bgraph.md parses back");
        assert!(
            matches!(back.identity, ParseIdentity::Verified),
            "OCR-produced bgraph.md must self-verify; got {:?}",
            back.identity
        );
        // Verbatim interior whitespace on the code block survives.
        let code = back
            .graph
            .nodes
            .values()
            .find(|n| n.node_type == "CodeBlock")
            .expect("code block");
        assert_eq!(code.content.text, "line one\n  indented two");
    }
}
