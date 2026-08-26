//! OCR native-metadata graft (S2 of the OCR premium arc).
//!
//! Pure-OCR's sharpest gap: `mist.json` reads the *render*, not the
//! container, so S1's canonical metadata fields are all-`None` by
//! design. The graft runs the **native arm's metadata extraction only**
//! — no graph build, no node alignment — over a companion PDF and
//! merges doc-level fields into the OCR graph. Two extractors over one
//! source with a fixed precedence policy is still extraction-layer per
//! `09-metadata-first-class` — that is what makes this legal in core
//! rather than URD composition.
//!
//! Three layers, split so the policy is JVM-free-testable:
//!
//! 1. [`graft_native_metadata`] — the **pure merge**. No I/O, no JVM.
//!    The precedence policy lives here and nowhere else.
//! 2. [`native_metadata_from_xhtml`] — the metadata-extraction seam,
//!    reused verbatim from the PDF channel
//!    ([`crate::preprocessors::pdf::metadata::PdfMetadataExtractor`],
//!    the exact pattern at `pdf/xhtml_parser.rs::parse_xhtml`). No new
//!    PDF-parsing dependency, no direct trailer reads — the graft
//!    yields the same `pdf.*` values the native channel would.
//! 3. [`parse_ocr_with_pdf`] — the composition wrapper:
//!    [`super::parse_ocr`] unchanged, PDF → XHTML via the
//!    [`Preprocessor`] seam (Tika in production), then 2 → 1.
//!    Feature-gated `jni-backend` (it rides the Tika path; the pure
//!    graft and the seam function are *not* gated).
//!
//! ## Graft policy (fixed precedence, definitional)
//!
//! - Canonical fields (`title`/`author`/`description`/`language`/
//!   `created`): **native arm wins**; the OCR-side value is kept only
//!   where the native arm has none. (OCR side is `None` by S1 design,
//!   so today this is "fill" — stated as precedence so it survives S1
//!   evolving.)
//! - `pdf` namespace slot ← the extracted [`PdfMetadata`]
//!   (`crate::types::PdfMetadata`), verbatim.
//! - `ocr` namespace slot: untouched, except `companion_pdf_sha256` ←
//!   sha256 of the companion PDF bytes (the companion linkage).
//! - Provenance unchanged: `source_format: "ocr"`, `source_sha256` =
//!   mist.json bytes. The PDF is a metadata companion, not a second
//!   source.
//!
//! ## Graceful degradation, but explicit
//!
//! A PDF that yields no metadata (scan-wrapped, stripped trailer) is
//! *not* an error — the graft merges what exists, leaves the rest
//! `None`, and the `pdf` slot is present with whatever was extracted.
//! An *unreadable* or empty PDF **is** an error
//! ([`ParseError::CompanionPdf`]) — the caller asked for the graft;
//! silently degrading to a single-arm parse would lie about what the
//! output is.
//!
//! ## Determinism
//!
//! `(mist.json, pdf, config) → bgraph.md` stays a pure function; no
//! companion → byte-identical to the S1 single-arm output (the
//! `companion_pdf_sha256` key serializes absent-when-`None`, so a
//! single-arm parse's hashed content body does not move).

use sha2::{Digest, Sha256};

use crate::preprocessors::md::types::{ParseError, ParseOptions, ParseResult};
use crate::preprocessors::metadata::extract_document_metadata;
use crate::preprocessors::pdf::metadata::PdfMetadataExtractor;
use crate::types::DocumentMetadata;

#[cfg(feature = "jni-backend")]
use crate::preprocessors::traits::Preprocessor;

/// Extract the native arm's document metadata from Tika XHTML — the
/// exact seam the PDF channel's `parse_xhtml` drives
/// (`extract_document_metadata(&PdfMetadataExtractor::new(xhtml), &())`),
/// without the text/style/bookmark walk. Pure and JVM-free: the JVM
/// (Tika) produces the XHTML *before* this point.
///
/// The returned [`DocumentMetadata`] carries the canonical fields plus a
/// populated `pdf` namespace slot; the other namespace slots are `None`.
pub fn native_metadata_from_xhtml(xhtml: &str) -> DocumentMetadata {
    extract_document_metadata(&PdfMetadataExtractor::new(xhtml), &())
}

/// The pure merge — the graft policy's single home (module docs above).
/// No I/O, no JVM; unit-testable from any [`DocumentMetadata`] source
/// (a live seam extraction, a committed XHTML fixture, or metadata
/// pulled from a frozen native-arm graph).
///
/// `companion_pdf_sha256` is the hex sha256 of the companion PDF bytes
/// (compute it with [`companion_sha256`]); it lands in the `ocr`
/// namespace as the companion linkage.
pub fn graft_native_metadata(
    result: &mut ParseResult,
    native: DocumentMetadata,
    companion_pdf_sha256: String,
) {
    let md = &mut result.graph.document_info.document_metadata;

    // Canonical fields: native arm wins; OCR-side value survives only
    // where the native arm has none.
    md.title = native.title.or(md.title.take());
    md.author = native.author.or(md.author.take());
    md.description = native.description.or(md.description.take());
    md.language = native.language.or(md.language.take());
    md.created = native.created.or(md.created.take());

    // `pdf` namespace slot ← the extracted PdfMetadata, verbatim
    // (present even when every field inside is empty — "graft what
    // exists" is visible, not silent).
    md.pdf = native.pdf;

    // `ocr` namespace slot: untouched except the companion linkage.
    // `parse_ocr` always populates the slot; `get_or_insert_with` is
    // defensive against a hand-built ParseResult.
    md.ocr.get_or_insert_with(Default::default).companion_pdf_sha256 = Some(companion_pdf_sha256);
}

/// Hex sha256 of the companion PDF bytes — the value
/// [`graft_native_metadata`] records as the companion linkage.
pub fn companion_sha256(pdf_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pdf_bytes);
    format!("{:x}", hasher.finalize())
}

/// The composition wrapper: [`super::parse_ocr`] (unchanged) + the
/// native arm's metadata extraction over a companion PDF + the pure
/// graft. This is the default premium path — in the API product flow
/// the customer always sends the PDF, so canonical fields are
/// effectively never null in premium output; single-arm [`super::parse_ocr`]
/// survives as the degraded re-derive / raw-image path.
///
/// `preprocessor` supplies the PDF → XHTML hop ([`Preprocessor`] —
/// [`crate::preprocessors::pdf::PdfPreprocessor`] / Tika in
/// production; the golden freeze replays it from a committed C1 XHTML
/// fixture, JVM-free). Feature-gated with the Tika path it rides.
///
/// Errors:
/// - empty `pdf_bytes` or a failed PDF → XHTML extraction →
///   [`ParseError::CompanionPdf`] — never a silent single-arm fallback;
/// - anything [`super::parse_ocr`] rejects, unchanged.
///
/// A PDF whose XHTML carries no metadata is NOT an error (module docs:
/// graceful degradation, but explicit).
#[cfg(feature = "jni-backend")]
pub fn parse_ocr_with_pdf<P: Preprocessor + ?Sized>(
    ocr_bytes: &[u8],
    pdf_bytes: &[u8],
    opts: ParseOptions,
    preprocessor: &P,
) -> Result<ParseResult, ParseError> {
    if pdf_bytes.is_empty() {
        return Err(ParseError::CompanionPdf(
            "companion PDF is empty (0 bytes)".to_string(),
        ));
    }
    let mut result = super::parse_ocr(ocr_bytes, opts)?;
    let xhtml = preprocessor
        .parse_pdf_to_markup_language(pdf_bytes)
        .map_err(|e| {
            ParseError::CompanionPdf(format!("native metadata extraction failed: {e}"))
        })?;
    let native = native_metadata_from_xhtml(&xhtml);
    graft_native_metadata(&mut result, native, companion_sha256(pdf_bytes));
    Ok(result)
}

// =============================================================================
// Tests — all JVM-free. Native-arm metadata for the attention twin comes
// from the frozen golden graph (`golden/1.0.0/attention/document.bgraph.json`)
// and from the committed C1 XHTML cache — Tika's own recorded output over
// the same source PDF that sits behind `demo-ocr/source.json`.
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preprocessors::ocr::parse_ocr;
    use crate::types::PdfMetadata;
    use std::path::PathBuf;

    fn fixtures() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_fixtures")
    }

    /// `demo-ocr/source.json` parsed single-arm — the S1 baseline.
    fn parse_demo_ocr() -> ParseResult {
        let bytes = std::fs::read(fixtures().join("golden/1.0.0/demo-ocr/source.json"))
            .expect("demo-ocr source.json is committed");
        parse_ocr(&bytes, ParseOptions::default()).expect("demo-ocr source parses")
    }

    /// The frozen native-arm metadata for the same source PDF — pulled
    /// from the attention golden graph (this pairing is the real
    /// attention twin: one PDF behind both artifacts).
    fn frozen_native_metadata() -> DocumentMetadata {
        let json = std::fs::read_to_string(
            fixtures().join("golden/1.0.0/attention/document.bgraph.json"),
        )
        .expect("frozen attention golden json is committed");
        let v: serde_json::Value = serde_json::from_str(&json).expect("golden json parses");
        serde_json::from_value(v["document_info"]["document_metadata"].clone())
            .expect("frozen document_metadata deserializes")
    }

    /// Sha256 of the committed attention PDF (identical bytes at
    /// `pdfs/attention-is-all-you-need.pdf` and
    /// `golden/1.0.0/attention/attention.pdf`).
    fn attention_pdf_sha() -> String {
        let bytes = std::fs::read(fixtures().join("pdfs/attention-is-all-you-need.pdf"))
            .expect("attention pdf is committed");
        companion_sha256(&bytes)
    }

    // --- the attention-twin policy test (handoff §6) -------------------

    #[test]
    fn graft_fills_canonical_fields_from_frozen_native_arm() {
        let mut result = parse_demo_ocr();
        let native = frozen_native_metadata();
        let pdf_ns = native.pdf.clone().expect("frozen native carries pdf ns");

        graft_native_metadata(&mut result, native, attention_pdf_sha());

        let md = &result.graph.document_info.document_metadata;
        // Canonical: what the native arm knows, filled; what it doesn't,
        // still None (attention's PDF carries title + created only).
        assert_eq!(md.title.as_deref(), Some("Attention Is All You Need"));
        assert_eq!(md.created.as_deref(), Some("2024-04-10T21:11:43Z"));
        assert!(md.author.is_none());
        assert!(md.description.is_none());
        assert!(md.language.is_none());

        // `pdf` namespace: the native extraction, verbatim.
        let pdf = md.pdf.as_ref().expect("pdf namespace present");
        assert_eq!(
            serde_json::to_value(pdf).unwrap(),
            serde_json::to_value(&pdf_ns).unwrap(),
            "pdf namespace must be the native arm's, verbatim"
        );
        assert_eq!(pdf.page_count, Some(15));
        assert_eq!(pdf.producer.as_deref(), Some("pdfTeX-1.40.25"));

        // `ocr` namespace: untouched except the linkage.
        let ocr = md.ocr.as_ref().expect("ocr namespace present");
        assert_eq!(ocr.model.as_deref(), Some("mistral-ocr-4-0"));
        assert_eq!(ocr.pages_processed, Some(15));
        assert_eq!(ocr.doc_size_bytes, Some(2_215_244));
        assert_eq!(ocr.dpi, Some(93));
        assert!(ocr.extras.is_empty());
        assert_eq!(ocr.companion_pdf_sha256.as_deref(), Some(attention_pdf_sha().as_str()));

        // Provenance unchanged — the PDF is a companion, not a source.
        assert_eq!(result.provenance.source_format, "ocr");
        assert_eq!(result.provenance.config_hash, "none");
    }

    /// The seam-consistency / Tika-stability check, JVM-free: the seam
    /// function over Tika's committed C1 XHTML must yield the metadata
    /// the frozen native-arm graph carries — with one *documented*
    /// exception. Attention's PDF container has **no `dc:title`**: the
    /// frozen graph's title is the PDF pipeline's body-side inference
    /// (`processor.rs` — honored only when extraction returns `None`,
    /// the F-02 deferral), which lives beyond the graft's
    /// extraction-layer reach ("metadata extraction only — no graph
    /// build"). So the seam yields `title: None` here; every
    /// extraction-native field must match. If Tika's extraction (or the
    /// seam) drifts, this and the graft golden fail together.
    #[test]
    fn seam_over_committed_c1_xhtml_matches_frozen_native_arm() {
        let sha = attention_pdf_sha();
        let xhtml = std::fs::read_to_string(
            fixtures().join(format!("snapshots/c1-xhtml/{sha}.xhtml")),
        )
        .expect("committed C1 XHTML for the attention pdf");
        let from_seam = native_metadata_from_xhtml(&xhtml);
        let frozen = frozen_native_metadata();

        // The provenance split, pinned: extraction has no title for this
        // PDF; the frozen graph's title is body-side inference.
        assert!(from_seam.title.is_none(), "no dc:title in the container");
        assert_eq!(frozen.title.as_deref(), Some("Attention Is All You Need"));

        // Every extraction-native field agrees.
        assert_eq!(from_seam.author, frozen.author);
        assert_eq!(from_seam.description, frozen.description);
        assert_eq!(from_seam.language, frozen.language);
        assert_eq!(from_seam.created, frozen.created);
        assert_eq!(from_seam.created.as_deref(), Some("2024-04-10T21:11:43Z"));
        assert_eq!(
            serde_json::to_value(&from_seam.pdf).unwrap(),
            serde_json::to_value(&frozen.pdf).unwrap(),
            "seam pdf namespace must equal the frozen native arm's"
        );
    }

    // --- precedence (stated as precedence, not fill) -------------------

    #[test]
    fn native_arm_wins_canonical_fields_ocr_side_survives_where_native_is_none() {
        let mut result = parse_demo_ocr();
        // Simulate S1 evolving an OCR-side canonical value.
        {
            let md = &mut result.graph.document_info.document_metadata;
            md.title = Some("ocr-side title".to_string());
            md.language = Some("ocr-side lang".to_string());
        }
        let native = DocumentMetadata {
            title: Some("native title".to_string()),
            language: None,
            ..Default::default()
        };
        graft_native_metadata(&mut result, native, "0".repeat(64));

        let md = &result.graph.document_info.document_metadata;
        assert_eq!(md.title.as_deref(), Some("native title"), "native wins");
        assert_eq!(
            md.language.as_deref(),
            Some("ocr-side lang"),
            "ocr side survives only where the native arm has none"
        );
    }

    // --- metadata-less degradation (handoff §5) ------------------------

    #[test]
    fn metadata_less_pdf_grafts_what_exists_without_error() {
        let mut result = parse_demo_ocr();
        // A scan-wrapped / stripped-trailer PDF: Tika XHTML with no
        // <meta> tags at all.
        let native = native_metadata_from_xhtml(
            "<html xmlns=\"http://www.w3.org/1999/xhtml\"><head></head><body></body></html>",
        );
        let sha = "a".repeat(64);
        graft_native_metadata(&mut result, native, sha.clone());

        let md = &result.graph.document_info.document_metadata;
        // Body-side inferred title (PDF-pipeline rule, post-S2-review)
        // survives a metadata-less graft: native None never erases it.
        assert_eq!(md.title.as_deref(), Some("Attention Is All You Need"));
        assert!(md.created.is_none());
        // `pdf` slot present with whatever was extracted (here: nothing).
        let pdf = md.pdf.as_ref().expect("pdf slot present even when empty");
        assert_eq!(
            serde_json::to_value(pdf).unwrap(),
            serde_json::to_value(PdfMetadata::default()).unwrap()
        );
        // Linkage still recorded.
        assert_eq!(
            md.ocr.as_ref().unwrap().companion_pdf_sha256.as_deref(),
            Some(sha.as_str())
        );
    }

    // --- no-companion byte-identity (wire shape) -----------------------

    #[test]
    fn no_companion_parse_is_byte_identical_to_s1_wire_shape() {
        // The single-arm canonical body must not carry the S2 key at all
        // — absent-when-None keeps the S1 bgraph_sha256 unchanged.
        let result = parse_demo_ocr();
        let canonical =
            crate::graphs::serialization::canonical::canonical_json(&result.graph);
        assert!(
            !canonical.contains("companion_pdf_sha256"),
            "single-arm parse must not serialize the companion key"
        );
        // And the golden anchor: the sha must equal the frozen demo-ocr
        // golden's (the S1 output) — the graft changed nothing it wasn't
        // asked to.
        let frozen: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(
                fixtures().join("golden/1.0.0/demo-ocr/document.bgraph.json"),
            )
            .expect("frozen demo-ocr golden json"),
        )
        .unwrap();
        assert_eq!(
            crate::graphs::serialization::canonical::bgraph_sha256(&result.graph),
            frozen["bgraph_sha256"].as_str().unwrap(),
            "no-companion bgraph_sha256 must equal the S1 golden"
        );
    }

    #[test]
    fn companion_key_serializes_when_set_and_roundtrips() {
        let mut result = parse_demo_ocr();
        graft_native_metadata(&mut result, frozen_native_metadata(), attention_pdf_sha());
        let md = &result.graph.document_info.document_metadata;
        let json = serde_json::to_string(md).unwrap();
        assert!(json.contains("companion_pdf_sha256"));
        let back: DocumentMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(
            back.ocr.unwrap().companion_pdf_sha256,
            md.ocr.as_ref().unwrap().companion_pdf_sha256
        );
    }

    // --- wrapper error shapes (JVM-free: errors fire before the seam) --

    #[cfg(feature = "jni-backend")]
    mod wrapper {
        use super::*;
        use crate::preprocessors::traits::Preprocessor;
        use crate::types::PreprocessorOutput;

        /// Serves a fixed XHTML string — the committed-C1-replay pattern
        /// in miniature.
        struct FixedXhtml(&'static str);
        /// Always fails the PDF → XHTML hop.
        struct FailingSeam;

        impl Preprocessor for FixedXhtml {
            fn parse_pdf_to_markup_language(&self, _: &[u8]) -> anyhow::Result<String> {
                Ok(self.0.to_string())
            }
            fn parse_markup_to_preprocessor_output(
                &self,
                _: &str,
            ) -> anyhow::Result<PreprocessorOutput> {
                panic!("the graft wrapper must never run the body-side XHTML parse")
            }
            fn name(&self) -> &str {
                "fixed-xhtml-test-seam"
            }
            fn supports_file_type(&self, _: &std::path::Path) -> bool {
                true
            }
        }

        impl Preprocessor for FailingSeam {
            fn parse_pdf_to_markup_language(&self, _: &[u8]) -> anyhow::Result<String> {
                Err(anyhow::anyhow!("tika exploded"))
            }
            fn parse_markup_to_preprocessor_output(
                &self,
                _: &str,
            ) -> anyhow::Result<PreprocessorOutput> {
                unreachable!()
            }
            fn name(&self) -> &str {
                "failing-test-seam"
            }
            fn supports_file_type(&self, _: &std::path::Path) -> bool {
                true
            }
        }

        fn ocr_bytes() -> Vec<u8> {
            std::fs::read(fixtures().join("golden/1.0.0/demo-ocr/source.json")).unwrap()
        }

        #[test]
        fn empty_pdf_is_companion_error_never_silent_single_arm() {
            let err = parse_ocr_with_pdf(&ocr_bytes(), &[], ParseOptions::default(), &FixedXhtml("<html/>"))
                .expect_err("empty companion must error");
            assert!(
                matches!(err, ParseError::CompanionPdf(_)),
                "got {err:?}"
            );
        }

        #[test]
        fn failed_extraction_is_companion_error_never_silent_single_arm() {
            let err = parse_ocr_with_pdf(
                &ocr_bytes(),
                b"%PDF-1.5 not really",
                ParseOptions::default(),
                &FailingSeam,
            )
            .expect_err("failed seam must error");
            assert!(matches!(err, ParseError::CompanionPdf(_)), "got {err:?}");
        }

        #[test]
        fn wrapper_composes_parse_seam_and_graft() {
            let xhtml = "<html xmlns=\"http://www.w3.org/1999/xhtml\"><head>\
                         <meta name=\"dc:title\" content=\"Wrapped Title\" />\
                         </head><body></body></html>";
            let pdf = b"%PDF-1.5 stand-in bytes";
            let result =
                parse_ocr_with_pdf(&ocr_bytes(), pdf, ParseOptions::default(), &FixedXhtml(xhtml))
                    .expect("wrapper parses");
            let md = &result.graph.document_info.document_metadata;
            assert_eq!(md.title.as_deref(), Some("Wrapped Title"));
            assert!(md.pdf.is_some());
            assert_eq!(
                md.ocr.as_ref().unwrap().companion_pdf_sha256.as_deref(),
                Some(companion_sha256(pdf).as_str())
            );
        }

        #[test]
        fn malformed_ocr_still_errors_as_ocr_not_companion() {
            let err = parse_ocr_with_pdf(
                b"not json",
                b"%PDF-1.5",
                ParseOptions::default(),
                &FixedXhtml("<html/>"),
            )
            .expect_err("malformed ocr errors");
            assert!(matches!(err, ParseError::MalformedOcr(_)), "got {err:?}");
        }
    }
}
