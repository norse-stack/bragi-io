//! OCR preprocessor (S1 of the OCR arc).
//!
//! ## Body channel
//!
//! [`body::parse_ocr`] is the live entry point: Mistral OCR-4 JSON bytes
//! (`mist.json`) → `DocumentGraph`, mirroring the DOCX channel's
//! [`crate::preprocessors::docx::parse_docx`] shape (free function
//! returning a built graph; no `Preprocessor` trait). Single arm only:
//! no native-arm reconciliation, no Mistral API call, no network — the
//! JSON is pinned source bytes, which is the arc's determinism boundary.
//!
//! [`numbering`] is the CR-98 outline post-pass: OCR-4's per-page
//! heading levels are noisy (no cross-page memory), so where a coherent
//! majority of Section titles carry a numbering scheme, the numbering's
//! rank replaces the per-page level (capped at
//! `numbering::MAX_OUTLINE_DEPTH`). Unnumbered documents pass through
//! byte-identical.
//!
//! [`furniture`] is the CR-99 repetition guard: OCR-4 labels blocks
//! `header`/`footer` by page position, not content, so one-off body
//! prose that touches the page edge (scholarly footnotes, cover-page
//! blocks) can arrive supplier-labeled as furniture.
//! The label survives only when its
//! normalized text recurs on ≥ `furniture::FURNITURE_MIN_RECURRENCE_PAGES`
//! distinct pages; non-repeating "furniture" reclassifies to Paragraph
//! in body flow. Small docs (below the same page floor) pass through
//! byte-identical.
//!
//! ## Payload contract
//!
//! [`payload`] holds the tolerant serde structs. **`pages[].blocks[]` is
//! the single source of truth**; every parallel page-level field
//! (`confidence_scores`, `header`, `footer`, `tables`, `hyperlinks`,
//! `images`) is nullable enrichment and is not consumed (2026-08-24
//! fixture experiment, deviations 1–4). Consumed: `pages[].blocks[]` +
//! `pages[].dimensions` + `pages[].markdown` (title-level inference
//! fallback only) + top-level `model` / `usage_info`.
//!
//! ## Metadata channel
//!
//! [`OcrMetadataExtractor`] implements
//! [`crate::preprocessors::metadata::MetadataExtractor`]. **Canonical
//! fields (`title`/`author`/`description`/`language`/`created`) are all
//! `None` by design** — the OCR JSON has no source-native document
//! metadata, and synthesizing a title from the first heading would be a
//! body-side fallback (09-metadata-first-class § F-02). Leaving them
//! null *is* the single-arm design. The `ocr:` namespace
//! ([`crate::types::OcrMetadata`]) carries what the payload knows about
//! its own run.
//!
//! ## Native metadata graft (S2)
//!
//! [`graft`] fills the gap on the two-arm path: the native arm's
//! metadata extraction runs over a companion PDF and its doc-level
//! fields merge into the OCR graph with a fixed precedence policy
//! (native wins canonical fields; `pdf` namespace verbatim; `ocr`
//! namespace untouched except `supplier_sha256`).
//! [`graft::parse_ocr_with_pdf`] is the composed entry point
//! (`jni-backend`); [`graft::graft_native_metadata`] is the pure,
//! JVM-free merge. No companion → byte-identical to the single-arm
//! output.

pub mod body;
mod furniture;
pub mod graft;
mod numbering;
pub mod payload;

pub use body::parse_ocr;
pub use graft::{companion_sha256, graft_native_metadata, native_metadata_from_xhtml};
#[cfg(feature = "jni-backend")]
pub use graft::parse_ocr_with_pdf;
pub use payload::is_ocr_json;

use crate::preprocessors::metadata::MetadataExtractor;
use crate::types::{ChannelMetadata, OcrMetadata};

/// OCR metadata extractor. Holds the run-level facts pulled from the
/// payload; the pre-parsed state lives on `self`, so `type Input = ()`
/// (the documented pattern, same as [`super::docx::DocxMetadataExtractor`]).
#[derive(Debug, Default, Clone)]
pub struct OcrMetadataExtractor {
    model: Option<String>,
    pages_processed: Option<u32>,
    doc_size_bytes: Option<u64>,
    dpi: Option<u32>,
}

impl OcrMetadataExtractor {
    /// Build an extractor over the already-parsed payload facts.
    pub(super) fn new(
        model: Option<String>,
        pages_processed: Option<u32>,
        doc_size_bytes: Option<u64>,
        dpi: Option<u32>,
    ) -> Self {
        Self {
            model,
            pages_processed,
            doc_size_bytes,
            dpi,
        }
    }
}

impl MetadataExtractor for OcrMetadataExtractor {
    type Input = ();

    /// Title source: **none**. The OCR payload carries no source-native
    /// title; a first-heading fallback would be body-side (F-02) —
    /// always `None`. S2 grafts this from the native arm.
    fn extract_title(&self, _: &()) -> Option<String> {
        None
    }

    /// Author source: **none** — always `None` (no source slot exists).
    fn extract_author(&self, _: &()) -> Option<String> {
        None
    }

    /// Description source: **none** — always `None`. The top-level
    /// `document_annotation` is an opt-in API feature (out of S1 scope),
    /// not document metadata.
    fn extract_description(&self, _: &()) -> Option<String> {
        None
    }

    /// Language source: **none** — always `None` (the payload does not
    /// report a detected language; inferring one from body text would
    /// be a body-side fallback).
    fn extract_language(&self, _: &()) -> Option<String> {
        None
    }

    /// Created source: **none** — always `None`. **No mtime / run-time
    /// fallback** (CR-56 § Invariance).
    fn extract_created(&self, _: &()) -> Option<String> {
        None
    }

    /// `ocr:` namespace bag: `model` (top-level), `pages_processed` +
    /// `doc_size_bytes` (`usage_info`), `dpi` (first page's
    /// `dimensions.dpi`). Absent slots → `None`; `extras` stays empty in
    /// S1 (no unrecognized-field sweep — the payload's parallel
    /// page-level arrays are enrichment, not metadata).
    fn extract_channel_metadata(&self, _: &()) -> ChannelMetadata {
        ChannelMetadata::Ocr(OcrMetadata {
            model: self.model.clone(),
            pages_processed: self.pages_processed,
            doc_size_bytes: self.doc_size_bytes,
            dpi: self.dpi,
            // Supplier identity — set only by the graft
            // ([`super::ocr::graft_native_metadata`]), never by the
            // single-arm extraction (whose doc-level `source` fence
            // already names these very bytes).
            supplier_sha256: None,
            extras: std::collections::BTreeMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preprocessors::metadata::extract_document_metadata;

    #[test]
    fn canonical_fields_all_none_ocr_namespace_populated() {
        let extractor = OcrMetadataExtractor::new(
            Some("mistral-ocr-4-0".to_string()),
            Some(15),
            Some(2_215_244),
            Some(93),
        );
        let md = extract_document_metadata(&extractor, &());

        // Canonical fields stay None — leaving them null IS the design.
        assert!(md.title.is_none());
        assert!(md.author.is_none());
        assert!(md.description.is_none());
        assert!(md.language.is_none());
        assert!(md.created.is_none());

        // Cross-channel slots stay empty.
        assert!(md.pdf.is_none());
        assert!(md.md.is_none());
        assert!(md.docx.is_none());

        let ocr = md.ocr.expect("ocr namespace populated");
        assert_eq!(ocr.model.as_deref(), Some("mistral-ocr-4-0"));
        assert_eq!(ocr.pages_processed, Some(15));
        assert_eq!(ocr.doc_size_bytes, Some(2_215_244));
        assert_eq!(ocr.dpi, Some(93));
        assert!(ocr.extras.is_empty());
    }

    #[test]
    fn empty_payload_yields_empty_namespace() {
        let md = extract_document_metadata(&OcrMetadataExtractor::default(), &());
        assert!(md.title.is_none());
        let ocr = md.ocr.expect("ocr namespace always present");
        assert!(ocr.model.is_none());
        assert!(ocr.pages_processed.is_none());
        assert!(ocr.dpi.is_none());
    }
}
