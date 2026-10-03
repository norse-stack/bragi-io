//! Tolerant serde model of the Mistral OCR-4 JSON payload, plus the
//! [`is_ocr_json`] content sniff.
//!
//! **`blocks[]` is the single source of truth — with one join.** The
//! 2026-08-24 fixture experiment confirmed that every parallel
//! page-level field is nullable enrichment: `confidence_scores` is null
//! everywhere (both fixtures, 166 pages), page-level `header`/`footer`
//! are null even where header/footer *blocks* exist, page-level
//! `tables[]` is empty while `table` blocks carry the full grid inline,
//! and `hyperlinks[]` is a bare URL list with no bbox/anchor.
//! `images[]` is the exception (CR-100): an `image` block carries only
//! the ref text plus an `image_id`, so the bytes and annotation must be
//! joined from the page-level array. Accordingly this model declares
//! what the channel consumes — `pages[].blocks[]`, `pages[].images[]`,
//! `pages[].dimensions`, `pages[].markdown` (title-level inference
//! fallback), `pages[].index`, and top-level `model` / `usage_info` —
//! and every field except `pages` itself is `Option`/defaulted so a
//! payload variant that omits or nulls anything still parses. Unknown
//! fields are dropped by serde (no `deny_unknown_fields` anywhere).

use serde::Deserialize;

/// Top-level OCR payload. `pages` is the one **required** field — its
/// absence means the JSON is not an OCR document (the shape gate
/// [`super::body::parse_ocr`] errors on), and it is what keeps a
/// `bgraph.json` from ever deserializing as an OCR payload.
#[derive(Debug, Clone, Deserialize)]
pub struct OcrDocument {
    pub pages: Vec<OcrPage>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub usage_info: Option<OcrUsageInfo>,
}

/// Top-level `usage_info`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct OcrUsageInfo {
    #[serde(default)]
    pub pages_processed: Option<u32>,
    #[serde(default)]
    pub doc_size_bytes: Option<u64>,
}

/// One page. Only the consumed fields are declared; the remaining
/// page-level enrichment arrays (`tables`, `hyperlinks`, `header`,
/// `footer`, `confidence_scores`) are deliberately absent from the model
/// — serde drops them.
///
/// **`images` is the one exception to "blocks are the single source of
/// truth"** (CR-100). An `image` block carries the ref text, its bbox
/// and an `image_id`, but the bytes and the annotation live only in this
/// page-level array; the two are joined by id. The array is
/// always populated — even a payload requested with
/// `include_image_base64: false` carries every image's id, bbox and
/// annotation, withholding only `image_base64`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct OcrPage {
    /// 0-based page index. Falls back to enumeration order when absent.
    #[serde(default)]
    pub index: Option<u32>,
    /// The page's full markdown rendering — consumed only as the
    /// title-level inference fallback (matching a bare title block
    /// against its `#`-prefixed heading line).
    #[serde(default)]
    pub markdown: Option<String>,
    #[serde(default)]
    pub dimensions: Option<OcrDimensions>,
    #[serde(default)]
    pub blocks: Option<Vec<OcrBlock>>,
    /// CR-100: the page's images, joined to their `image` blocks by id.
    #[serde(default)]
    pub images: Option<Vec<OcrImage>>,
}

/// One page-level image entry. Corner coords are absolute pixels at
/// `dimensions.dpi`, top-left origin — the same convention
/// [`OcrBlock`] uses, so the same `72 / dpi` scale applies.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct OcrImage {
    /// Supplier id, e.g. `img-0.jpeg`. Matches the carrying block's
    /// `image_id`. Absent ids simply never join.
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub top_left_x: Option<f32>,
    #[serde(default)]
    pub top_left_y: Option<f32>,
    #[serde(default)]
    pub bottom_right_x: Option<f32>,
    #[serde(default)]
    pub bottom_right_y: Option<f32>,
    /// The data-URI payload (`data:image/jpeg;base64,…`). `None` when
    /// the payload was requested without image bytes; the model stays
    /// tolerant of either.
    #[serde(default)]
    pub image_base64: Option<String>,
    #[serde(default)]
    pub image_annotation: Option<String>,
}

/// Page raster dimensions. `dpi` drives the pixel → PDF-point bbox
/// conversion (`72 / dpi`).
#[derive(Debug, Clone, Deserialize, Default)]
pub struct OcrDimensions {
    #[serde(default)]
    pub dpi: Option<u32>,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
}

/// One block: absolute-pixel corner coords at `dimensions.dpi`,
/// top-left origin, plus `content` and the block `type`. Block-level
/// `confidence_scores` is null in every observed payload (DT-11 treats
/// confidence as transient anyway) — not modeled.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct OcrBlock {
    #[serde(default)]
    pub top_left_x: Option<f32>,
    #[serde(default)]
    pub top_left_y: Option<f32>,
    #[serde(default)]
    pub bottom_right_x: Option<f32>,
    #[serde(default)]
    pub bottom_right_y: Option<f32>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default, rename = "type")]
    pub block_type: Option<String>,
    /// CR-100: on an `image` block, the join key into the page's
    /// `images[]`. `None` on every other block type.
    #[serde(default)]
    pub image_id: Option<String>,
}

/// Content-sniff: does this byte buffer look like a Mistral OCR-4 JSON
/// payload?
///
/// Heuristic: valid JSON, top-level object, with a `pages` **array**
/// whose entries are objects carrying a `markdown` or `blocks` key.
/// An empty `pages` array still sniffs as OCR when `model` or
/// `usage_info` is present alongside it.
///
/// A `bgraph.json` (`SortedDocumentGraph`) has no `pages` key at all, so
/// the two shapes can never cross-match — the disambiguation contract
/// for the CLI's `.json` routing.
pub fn is_ocr_json(bytes: &[u8]) -> bool {
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(bytes) else {
        return false;
    };
    let Some(obj) = value.as_object() else {
        return false;
    };
    let Some(pages) = obj.get("pages").and_then(|p| p.as_array()) else {
        return false;
    };
    if pages.is_empty() {
        return obj.contains_key("model") || obj.contains_key("usage_info");
    }
    pages.iter().all(|p| {
        p.as_object()
            .map(|o| o.contains_key("markdown") || o.contains_key("blocks"))
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sniffs_ocr_shape() {
        let ocr = br##"{"pages":[{"index":0,"markdown":"# T","blocks":[]}],"model":"m"}"##;
        assert!(is_ocr_json(ocr));
    }

    #[test]
    fn sniffs_empty_pages_with_model() {
        assert!(is_ocr_json(br#"{"pages":[],"model":"mistral-ocr-4-0"}"#));
        assert!(!is_ocr_json(br#"{"pages":[]}"#));
    }

    #[test]
    fn rejects_bgraph_json_shape() {
        // A minimal SortedDocumentGraph-shaped envelope: no `pages` key.
        let bgraph = br#"{"schema_version":"1.1.0","bgraph_sha256":"abc","nodes":[],"document_info":{}}"#;
        assert!(!is_ocr_json(bgraph));
    }

    #[test]
    fn rejects_pages_of_wrong_shape() {
        // A `pages` array of non-OCR entries (no markdown/blocks) must
        // not sniff as OCR — e.g. some other tool's "pages" concept.
        assert!(!is_ocr_json(br#"{"pages":[{"number":1,"text":"hi"}]}"#));
        assert!(!is_ocr_json(br#"{"pages":"not-an-array"}"#));
        assert!(!is_ocr_json(b"not json at all"));
        assert!(!is_ocr_json(br#"[1,2,3]"#));
    }

    #[test]
    fn tolerant_null_enrichment_parses() {
        // Every parallel page-level field null / absent; block fields
        // partially absent — must deserialize without complaint.
        let json = br#"{
            "pages": [{
                "index": 0,
                "markdown": null,
                "images": null,
                "tables": null,
                "hyperlinks": null,
                "header": null,
                "footer": null,
                "confidence_scores": null,
                "dimensions": {"dpi": null, "width": 791},
                "blocks": [
                    {"content": "hello", "type": "text", "confidence_scores": null},
                    {"top_left_x": 1.0}
                ]
            }],
            "model": null,
            "usage_info": null,
            "document_annotation": null
        }"#;
        let doc: OcrDocument = serde_json::from_slice(json).expect("tolerant parse");
        assert_eq!(doc.pages.len(), 1);
        let blocks = doc.pages[0].blocks.as_ref().unwrap();
        assert_eq!(blocks[0].content.as_deref(), Some("hello"));
        assert!(blocks[1].content.is_none());
        assert!(doc.model.is_none());
        assert!(doc.usage_info.is_none());
    }

    #[test]
    fn missing_pages_fails_to_parse() {
        // `pages` is the required shape gate.
        let r: Result<OcrDocument, _> = serde_json::from_slice(br#"{"model":"m"}"#);
        assert!(r.is_err());
    }
}
