//! CR-98 — outline numbering normalization for the OCR channel.
//!
//! Mistral OCR-4 infers heading levels **per page with no cross-page
//! memory**, so the same-rank chapters of a document can land on
//! different markdown levels page to page (attention fixture: chapters
//! 1–3 as `#` on page 1, chapters 4–6 as `##` on pages 5–7). The level
//! sets overlap across true ranks, so the hierarchy is not recoverable
//! from the levels at all — but where section titles carry a coherent
//! numbering scheme (`N` / `N.M` / `N.M.K`, or letter-led appendices
//! `A.` / `A.1`), the numbering's component count IS the rank.
//!
//! This module is the pure decision core of that post-pass:
//! [`normalized_levels`] maps the document's Section titles (plus their
//! S1-inferred levels) to numbering-rank levels, or `None` when the
//! coherence gate fails — in which case the caller must leave the
//! projection exactly as S1 built it, byte-for-byte.
//!
//! The numbering pattern reuses the semantics of the CR-97 WP2 picker
//! port (`bragi-api` `picker/scoring.rs`, `_tokens` numbering-strip
//! regex): star runs stripped first, then one anchored leading-numbering
//! match; a single uppercase letter is numbering, multi-letter acronyms
//! ("API", "RFC") are content.

use std::sync::LazyLock;

use regex::Regex;

/// Maximum outline depth after normalization — numbering deeper than
/// this folds to it (children swallowed upward, same spirit as the
/// GraphSanity depth/section-height caps in the PDF arm). Also caps the
/// preserved S1 level of the leading unnumbered title.
///
/// Hardcoded for CR-98 but config-shaped: if this ever needs tuning it
/// moves to the parse config (`config.rs`) as an `ocr:` knob, next to
/// the PDF arm's structural caps.
pub(super) const MAX_OUTLINE_DEPTH: u32 = 4;

/// Coherence gate (CR-98): the fraction of the document's Section titles
/// that must carry the numbering scheme before normalization applies.
/// Below this, OCR's per-page levels are left exactly as-is — the gate
/// is what keeps "2026 Outlook" from becoming chapter 2026 in a mostly
/// unnumbered document, and keeps unnumbered documents byte-identical
/// to pre-CR-98 output.
pub(super) const NUMBERING_COHERENCE_THRESHOLD: f64 = 2.0 / 3.0;

/// Markdown-emphasis strip (`\*+` → ""), run before the numbering match —
/// same order as the picker's `_tokens` (a bold-wrapped `**3.2 Foo**`
/// still reads as numbered).
static STAR_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*+").unwrap());

/// Leading section-numbering match: `"1."`, `"1.2.3"`, `"A."`, `"B.1."` …
/// followed by at least one whitespace char (numbering must introduce a
/// title, not BE the title). Single uppercase letters are numbering;
/// multi-letter caps (acronyms like "API", "RFC") are not. Same pattern
/// as `bragi-api` `picker/scoring.rs::NUMBERING_RE` (CR-97 WP2).
static NUMBERING_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\d+(?:\.\d+)*\.?|[A-Z](?:\.\d+(?:\.\d+)*)?\.?)\s+").unwrap()
});

/// Parse a Section title's leading numbering into its rank (component
/// count): `"4 Why Self-Attention"` → 1, `"3.2 Attention"` → 2,
/// `"3.2.1 Scaled Dot-Product Attention"` → 3, `"A. Appendix"` → 1,
/// `"A.1 Detail"` → 2. `None` for unnumbered titles (including
/// acronym-led ones like `"RFC 9000"`). Uncapped — the cap is applied at
/// level-assignment time in [`normalized_levels`].
pub(super) fn numbering_depth(title: &str) -> Option<u32> {
    let stripped = STAR_RE.replace_all(title, "");
    let matched = NUMBERING_RE.find(&stripped)?;
    let numbering = matched.as_str().trim_end().trim_end_matches('.');
    Some(numbering.split('.').count() as u32)
}

/// The CR-98 decision function. `titles` and `s1_levels` are the
/// document's Section titles and their S1-inferred levels, in emission
/// order (parallel slices).
///
/// Returns the normalized level per Section, or `None` when the
/// coherence gate fails (fewer than [`NUMBERING_COHERENCE_THRESHOLD`] of
/// the titles carry numbering) — the caller must then change nothing.
///
/// Level assignment when the gate passes:
/// - numbered title → its numbering rank, capped at [`MAX_OUTLINE_DEPTH`]
///   (deeper numbering folds to the cap, children swallowed upward);
/// - any unnumbered title (Abstract, References, Acknowledgements — and
///   the leading document title) attaches at depth 1. Pinning the title
///   to the top is a presentation heuristic, not an invariant (CR-98
///   decision note): not every document opens with its title; the true
///   one-doc-one-title invariant lives in `DocumentMetadata`, which the
///   S2 native-arm graft supplies. The outline entry is presentation.
///
/// Degrades toward flat, never panics, never exceeds the cap.
pub(super) fn normalized_levels(titles: &[&str], s1_levels: &[u32]) -> Option<Vec<u32>> {
    debug_assert_eq!(titles.len(), s1_levels.len());
    if titles.is_empty() {
        return None;
    }
    let depths: Vec<Option<u32>> = titles.iter().map(|t| numbering_depth(t)).collect();
    let numbered = depths.iter().flatten().count();
    if (numbered as f64) < NUMBERING_COHERENCE_THRESHOLD * (titles.len() as f64) {
        return None;
    }
    Some(
        depths
            .iter()
            .map(|depth| match depth {
                Some(rank) => (*rank).min(MAX_OUTLINE_DEPTH),
                // Unnumbered titles — the leading document title included —
                // attach at depth 1 (see the doc comment: presentation
                // heuristic, not an invariant).
                None => 1,
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

    // --- scheme parse -------------------------------------------------

    #[test]
    fn decimal_numbering_depth_is_component_count() {
        assert_eq!(numbering_depth("4 Why Self-Attention"), Some(1));
        assert_eq!(numbering_depth("3.2 Attention"), Some(2));
        assert_eq!(numbering_depth("3.2.1 Scaled Dot-Product Attention"), Some(3));
        assert_eq!(numbering_depth("1.2.3.4.5 Very Deep"), Some(5));
    }

    #[test]
    fn trailing_dot_does_not_add_a_component() {
        assert_eq!(numbering_depth("1. Introduction"), Some(1));
        assert_eq!(numbering_depth("3.2. Attention"), Some(2));
    }

    #[test]
    fn letter_led_appendix_numbering() {
        assert_eq!(numbering_depth("A. Transformer Details"), Some(1));
        assert_eq!(numbering_depth("A.1 Attention Heads"), Some(2));
        assert_eq!(numbering_depth("B.2.3 Ablation Grid"), Some(3));
        // Bare single letter + space is numbering too (picker `_tokens`
        // semantics — `[A-Z]` with the dot optional).
        assert_eq!(numbering_depth("B Hyperparameters"), Some(1));
    }

    #[test]
    fn acronyms_and_words_are_not_numbering() {
        assert_eq!(numbering_depth("API Overview"), None);
        assert_eq!(numbering_depth("RFC 9000"), None);
        assert_eq!(numbering_depth("Abstract"), None);
        assert_eq!(numbering_depth("References"), None);
        // Lowercase letter + dot is not numbering (picker semantics).
        assert_eq!(numbering_depth("a. lowercase lead"), None);
    }

    #[test]
    fn numbering_must_introduce_a_title_not_be_it() {
        // Pure-numbering titles don't match (the pattern requires
        // trailing whitespace + text, same as the picker strip).
        assert_eq!(numbering_depth("3.2"), None);
        assert_eq!(numbering_depth("7."), None);
        assert_eq!(numbering_depth(""), None);
    }

    #[test]
    fn star_wrap_is_stripped_before_the_match() {
        assert_eq!(numbering_depth("**3.2 Attention**"), Some(2));
        assert_eq!(numbering_depth("**A. Appendix**"), Some(1));
    }

    // --- coherence gate -----------------------------------------------

    #[test]
    fn gate_passes_at_two_thirds_exactly() {
        let titles = ["1 Intro", "2 Body", "References"];
        let levels = normalized_levels(&titles, &[1, 2, 2]).expect("2/3 passes");
        assert_eq!(levels, vec![1, 1, 1]);
    }

    #[test]
    fn gate_fails_below_two_thirds() {
        // 1 of 3 numbered — untouched.
        assert_eq!(normalized_levels(&["1 Intro", "Body", "References"], &[1, 2, 2]), None);
        // 0 numbered — the unnumbered-document case.
        assert_eq!(
            normalized_levels(&["Overview", "Methods", "Results"], &[1, 2, 2]),
            None
        );
    }

    #[test]
    fn empty_outline_is_untouched() {
        assert_eq!(normalized_levels(&[], &[]), None);
    }

    #[test]
    fn year_led_title_in_unnumbered_doc_never_becomes_a_chapter() {
        // "2026 Outlook" parses as numbering in isolation, but the gate
        // keeps the document untouched.
        assert_eq!(numbering_depth("2026 Outlook"), Some(1));
        assert_eq!(
            normalized_levels(
                &["2026 Outlook", "Executive Summary", "Market Trends", "Appendix"],
                &[1, 2, 2, 2],
            ),
            None
        );
    }

    // --- level assignment ---------------------------------------------

    #[test]
    fn leading_unnumbered_title_pins_to_depth_one() {
        let titles = ["My Paper Title", "1 Introduction", "2 Background", "2.1 Prior Work"];
        let levels = normalized_levels(&titles, &[2, 1, 2, 2]).expect("3/4 passes");
        assert_eq!(levels, vec![1, 1, 1, 2]);
    }

    #[test]
    fn unnumbered_sections_attach_at_depth_one() {
        // Attention-shaped: title + Abstract + numbered chapters +
        // References. 6 numbered of 9 = exactly 2/3 — gate passes;
        // title, Abstract and References all land at 1.
        let titles = [
            "Title", "Abstract", "1 Intro", "2 Body", "2.1 Sub", "3 More", "3.1 Sub", "3.2 Sub",
            "References",
        ];
        let levels =
            normalized_levels(&titles, &[2, 3, 1, 1, 3, 2, 3, 3, 2]).expect("6/9 passes");
        assert_eq!(levels, vec![1, 1, 1, 1, 2, 1, 2, 2, 1]);
    }

    #[test]
    fn numbered_leading_title_takes_its_rank() {
        let titles = ["1 Introduction", "2 Background", "2.1 Prior Work"];
        let levels = normalized_levels(&titles, &[1, 2, 2]).expect("all numbered");
        assert_eq!(levels, vec![1, 1, 2]);
    }

    #[test]
    fn cap_folds_deep_numbering_to_max_depth() {
        let titles = ["1 A", "1.1 B", "1.1.1 C", "1.1.1.1 D", "1.1.1.1.1 E", "1.1.1.1.1.1 F"];
        let levels = normalized_levels(&titles, &[1; 6]).expect("all numbered");
        assert_eq!(levels, vec![1, 2, 3, 4, 4, 4]);
        assert!(levels.iter().all(|&l| l <= MAX_OUTLINE_DEPTH));
    }

    #[test]
    fn leading_title_depth_is_independent_of_its_s1_level() {
        // Whatever level OCR gave the title — inflated, zero, anything —
        // the pin to depth 1 wins.
        let titles = ["Deep Title", "1 A", "2 B", "3 C"];
        assert_eq!(
            normalized_levels(&titles, &[6, 1, 1, 1]).expect("3/4 passes"),
            vec![1, 1, 1, 1]
        );
        assert_eq!(
            normalized_levels(&titles, &[0, 1, 1, 1]).expect("3/4 passes"),
            vec![1, 1, 1, 1]
        );
    }
}
