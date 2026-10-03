//! CR-99 — furniture repetition guard for the OCR channel.
//!
//! Mistral OCR-4 labels the top/bottom text block of a page `header` /
//! `footer` **by position, regardless of content** — and "was at the
//! page top" is not "is running chrome". Body prose that touches the
//! page edge (one-off scholarly footnotes above all, plus cover-page
//! blocks) arrives supplier-labeled as furniture, and routine consumer
//! hygiene (`strip --node-types header,footer`) then silently deletes
//! it: ~10KB of real prose across the compliance shelf at CR-99 time,
//! footnote bodies first among them. (F8's original volume claim —
//! ~93–99% of header-node bytes as misfiled recitals — did not survive
//! re-measurement and was retracted at CR-99 resolution; the guard's
//! justification is the semantics, not the volume.)
//!
//! The guard is DT-08's aggregate-repetition principle applied to
//! supplier labels instead of geometry: real running furniture repeats
//! across pages (near-verbatim modulo page numbers); content that
//! merely touches the page edge does not. A label that appears once has
//! no aggregate behind it, so it does not license a "chrome" claim.
//!
//! This module is the pure decision core (the CR-98 shape):
//! [`keep_mask`] maps one furniture pool's normalized texts + pages to
//! a keep/demote verdict per block, or `None` when the document is
//! below the small-doc floor — in which case the caller must leave the
//! projection exactly as S1 built it, byte-for-byte. Headers and
//! footers are separate pools (separate phenomena — a running head and
//! a running foot repeat independently); the caller invokes once per
//! pool.

use std::collections::{HashMap, HashSet};

/// K — a furniture label survives only when its normalized text recurs
/// on at least this many **distinct pages** (CR-99 straw man). Doubles
/// as the small-doc floor: below this page count, recurrence detection
/// is hollow (DT-08's sample-vs-instance argument — with fewer pages
/// than K the threshold is unreachable and every real footer would be
/// wrongly promoted to body), so the guard stands down and supplier
/// labels are trusted as-is.
///
/// Hardcoded for CR-99 but config-shaped: if this ever needs tuning it
/// moves to the parse config as an `ocr:` knob, next to
/// [`super::numbering::MAX_OUTLINE_DEPTH`].
pub(super) const FURNITURE_MIN_RECURRENCE_PAGES: u32 = 3;

/// Normalize a furniture block's text for recurrence comparison: strip
/// every numeric char and all whitespace. Page numbers and their
/// carriers collapse (`"Page 3 of 10"` / `"Page 4 of 10"` → `"Pageof"`;
/// a bare `"7"` → `""`), so paginated chrome recurs under one key. Case
/// and punctuation are kept — the claim is *near-verbatim* repetition,
/// not fuzzy similarity.
pub(super) fn normalize_furniture(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_numeric() && !c.is_whitespace())
        .collect()
}

/// The CR-99 decision function for one furniture pool (headers OR
/// footers). `normalized` and `pages` are the pool's
/// [`normalize_furniture`]-normalized texts and 1-indexed page numbers,
/// in emission order (parallel slices). `page_count` is the document's
/// page count.
///
/// Returns the verdict per block — `true` = keep the furniture label,
/// `false` = demote to body — or `None` when `page_count` is below
/// [`FURNITURE_MIN_RECURRENCE_PAGES`] (the small-doc floor): the caller
/// must then change nothing.
///
/// Recurrence is counted over **distinct pages**: the same text twice
/// on one page is one observation, not two.
pub(super) fn keep_mask(
    normalized: &[String],
    pages: &[u32],
    page_count: u32,
) -> Option<Vec<bool>> {
    debug_assert_eq!(normalized.len(), pages.len());
    if page_count < FURNITURE_MIN_RECURRENCE_PAGES {
        return None;
    }
    let mut pages_seen: HashMap<&str, HashSet<u32>> = HashMap::new();
    for (text, page) in normalized.iter().zip(pages) {
        pages_seen.entry(text).or_default().insert(*page);
    }
    Some(
        normalized
            .iter()
            .map(|text| {
                pages_seen[text.as_str()].len() as u32 >= FURNITURE_MIN_RECURRENCE_PAGES
            })
            .collect(),
    )
}

// =============================================================================
// Tests.
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn norm(texts: &[&str]) -> Vec<String> {
        texts.iter().map(|t| normalize_furniture(t)).collect()
    }

    #[test]
    fn normalization_collapses_page_numbers_and_whitespace() {
        assert_eq!(normalize_furniture("Page 3 of 10"), "Pageof");
        assert_eq!(normalize_furniture("Page 4 of 10"), "Pageof");
        assert_eq!(normalize_furniture("7"), "");
        assert_eq!(
            normalize_furniture("Official  Journal of the European Union 4.5.2016"),
            "OfficialJournaloftheEuropeanUnion.."
        );
        // Case and punctuation are significant — near-verbatim, not fuzzy.
        assert_ne!(
            normalize_furniture("Official Journal"),
            normalize_furniture("official journal")
        );
    }

    #[test]
    fn below_floor_guard_stands_down() {
        // 2-page doc: with K=3 the threshold is unreachable — every real
        // footer would be demoted. The guard must return None.
        let texts = norm(&["1", "2"]);
        assert_eq!(keep_mask(&texts, &[1, 2], 2), None);
        assert_eq!(keep_mask(&[], &[], 1), None);
    }

    #[test]
    fn recurring_text_keeps_label_transient_text_demotes() {
        // Masthead on 3 pages + a one-off recital: keep, keep, keep, demote.
        let texts = norm(&[
            "Official Journal of the EU",
            "Official Journal of the EU",
            "Official Journal of the EU",
            "(9) Harmonised rules applicable to the placing on the market",
        ]);
        let mask = keep_mask(&texts, &[1, 2, 3, 2], 5).expect("above floor");
        assert_eq!(mask, vec![true, true, true, false]);
    }

    #[test]
    fn page_number_only_footers_collapse_to_one_recurring_key() {
        let texts = norm(&["2", "3", "4", "5"]);
        let mask = keep_mask(&texts, &[2, 3, 4, 5], 5).expect("above floor");
        assert!(mask.iter().all(|&keep| keep));
    }

    #[test]
    fn same_page_repetition_counts_once() {
        // Three observations, but only two distinct pages — below K.
        let texts = norm(&["running head", "running head", "running head"]);
        let mask = keep_mask(&texts, &[1, 1, 2], 4).expect("above floor");
        assert_eq!(mask, vec![false, false, false]);
    }

    #[test]
    fn exactly_k_distinct_pages_is_kept() {
        let texts = norm(&["chrome", "chrome", "chrome"]);
        let mask = keep_mask(&texts, &[1, 2, 3], 3).expect("floor is inclusive");
        assert_eq!(mask, vec![true, true, true]);
    }
}
