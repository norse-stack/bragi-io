# Test Fixtures

Small, hand-curated fixtures for `blazegraph-core`. Tests load pre-generated
snapshots and assert stability at the pipeline edges — **no JVM required**.

Bulk evaluation corpora (large multi-document PDF/docx sets we score the parser
against) do **not** live here — they live under [`eval-corpus/`](../../eval-corpus/),
tracked with Git LFS. See [Where new corpus goes](#where-new-corpus-goes).

## Structure

```
test_fixtures/
├── pdfs/                          ← Fixture PDFs (committed to plain git)
│   ├── claude_shannon_paper.pdf      Small academic paper (~358KB)
│   └── elements_of_euclid.pdf        Large book (~1.8MB)
├── snapshots/                     ← Generated pipeline snapshots (committed)
│   ├── claude_shannon_paper/
│   │   ├── stage1a_xhtml.html        Tika XHTML output        ┐ back `tika_boundary`
│   │   ├── stage1b_text_elements.json                         │  (Boundary 1)
│   │   ├── summary.json              byte + element counts    ┘
│   │   └── stage3_graph.json         final graph — backs markdown_emit / markdown_roundtrip
│   ├── elements_of_euclid/           same shape
│   ├── c1-xhtml/<sha256>.xhtml      ← Golden-family cache tier C1 (Block D)
│   └── c2-preprocessor/<sha256>.json ← Golden-family cache tier C2 (Block D)
├── markdown/                       ← Generic-markdown parse fixtures (`generic_markdown_tests`)
├── docx/structured.docx            ← docx-body `#[cfg(test)]` fixture
├── golden/                         ← Block D reconstruction anchors (committed, plain git)
│   └── 1.0.0/attention/
│       ├── attention.pdf             The source document
│       ├── config.yaml               The exact config the family binds to
│       ├── document.bgraph.md        The frozen 1.0.0 emit (the anchor)
│       └── PRODUCED_BY               git HEAD sha at freeze time (codebase_sha binding)
└── README.md
```

## The Sandwich Model

Fixture tests stabilize the pipeline's **boundaries**, not its middle:

```
Boundary 1 (stable):  PDF → Tika → XHTML → TextElements
                      Guarded by `tika_boundary` — only moves if the Tika version does.

Middle (flexible):    TextElements → Rules → ParsedElements
                      Where we iterate. NOT snapshot-tested.

Boundary 2 (stable):  ParsedElements → Graph → bgraph.{md,json}
                      The schema contract for API customers — now owned by the
                      golden freeze family + the json-wire freeze, NOT stage snapshots.
```

> **CR-93 (2026-07-18):** Boundary 2 used to be guarded here by stage-snapshot
> modules (`schema_contract`, `graph_structure`, `breadcrumbs`). The B6 json-wire
> golden freeze now covers that ground at higher fidelity — byte-exact, plus an
> explicit schema-contract boundary proof — so those modules were retired.
> `tika_boundary` stays: the golden freeze replays the committed C2 cache and
> **skips Tika by design**, so it *structurally cannot* catch a Tika regression —
> which makes `tika_boundary` the **only** guard on that boundary.

## What the tests cover

| Module (file) | Tests | Guards |
|---|---|---|
| `tika_boundary` (`pipeline_tests.rs`) | 4 | XHTML byte + text-element counts per fixture — the sole Tika-drift guard |
| `golden_freeze_tests.rs` | 14 | Byte-freeze of `bgraph.md` + the json wire, json↔md identity, schema-contract boundary proof |
| `generic_markdown_tests.rs` | 14 | Generic-markdown parse paths on non-golden docs |
| `markdown_roundtrip_tests.rs` | 18 | Round-trip on the shannon/euclid graphs (`stage3_graph.json`) |
| `markdown_emit_tests.rs` | 2 | Emit stability on the same graphs |
| docx `#[cfg(test)]` (`preprocessors/docx/`) | — | docx-body parse (`docx/structured.docx`) |

No JVM: `tika_boundary` and the golden freeze both replay committed snapshots /
the C2 cache.

## Config

The stage snapshots were generated with the standard processing config
(`crates/cli/configs/processing/config.yaml`) — spatial clustering + paragraph
merging, the same pipeline configuration used in production. Without it, text
element counts are ~30× higher (raw Tika output, unmerged).

## Golden freeze family (Block D — the cold-tier reconstruction anchor)

Separate from the stage snapshots above. `golden/1.0.0/attention/` freezes one
real document's emitted **bgraph.md** at schema `1.0.0` as a *reconstruction
anchor*: the durable artifact a future version-pinned binary can use to prove it
still reproduces this edition. Tests live in `tests/golden_freeze_tests.rs`.

The family:

| Artifact | Role |
|----------|------|
| `golden/1.0.0/attention/attention.pdf` | The source document. |
| `golden/1.0.0/attention/config.yaml` | The exact config the family binds to (`config_hash` is stamped in the md). `dump_analytics: false` so the replay writes no sidecars. |
| `golden/1.0.0/attention/document.bgraph.md` | The frozen `1.0.0` emit — style-bearing (`--include-style-info`) so it self-verifies. |
| `golden/1.0.0/attention/PRODUCED_BY` | `blazegraph-io` git HEAD sha at freeze time — the `codebase_sha` binding. A sidecar, **not** a serialized-artifact field. |
| `snapshots/c1-xhtml/<sha>.xhtml`, `snapshots/c2-preprocessor/<sha>.json` | The committed cache tiers. `<sha>` is the SHA-256 of `attention.pdf`. |

### How the freeze test works (JVM-free)

`golden_freeze_tests.rs` replays the deterministic identity path
(`process_document_with_cache` → `build_graph_deterministic`, a pure function of
`PreprocessorOutput` + config) from the committed **C2 preprocessor cache**. A C2
cache hit skips Tika *and* the XHTML parse entirely — the test's stub
preprocessor panics if either is reached, so a JVM invocation is a loud failure,
not a silent slow pass.

- **Test A — reproduction:** regenerate the md from C2 (JVM-free) and assert it
  is **byte-identical** to the frozen `document.bgraph.md`.
- **Test B — roundtrip:** `parse_markdown` the frozen md and assert `Verified`.

```bash
cargo test -p bragi-io-core --test golden_freeze_tests   # or: make golden-test
```

### Re-freeze intentionally (a change legitimately moved the output)

JVM-free — regenerates the md + refreshes `PRODUCED_BY` from the committed C2:

```bash
BLESS_GOLDEN=1 cargo test -p bragi-io-core --test golden_freeze_tests
```

### Rebuild the whole family from the PDF (needs the JVM)

Runs a **clean, fresh Tika parse** (`--fresh-from c0`) — the family is never
seeded from a stale cache — rebuilds C1→C2, emits the style-bearing md, and
records the sha:

```bash
make golden-generate   # submodule Makefile; needs JRE + the Tika JAR
```

> **C3 note:** C3 is the cached **output** — the config-keyed `DocumentGraph`,
> from which `bgraph.md`/`.json` are serialized on demand (it is the first
> config-dependent tier; C0–C2 are config-independent intermediates). The
> pipeline does not yet *write* C3 (`store_graph_output` has no caller in
> `process_document_with_cache`); CR-89 corrected the read-gate, and wiring the
> writer + API delivery is the **C3 output-cache feature CR**. The golden family
> needs only C2 — the freeze replays via `FreshFrom::C3`, which skips the C3
> read and always rebuilds — so it commits no `c3-graph/` tier. When the writer
> lands, `golden-generate` can populate the slot.

## Where new corpus goes

- **Small, hand-authored unit fixtures** (a new boundary PDF, a markdown/docx
  case) → here, in plain git.
- **Trust-critical golden text** (`document.bgraph.md`/`.json`, `config.yaml`,
  `PRODUCED_BY`) → `golden/`, in **plain git** — the byte-honest freeze must diff
  *real* bytes, never an LFS pointer.
- **Bulk evaluation corpora** (large multi-doc PDF/docx sets + their generated
  derivatives) → [`eval-corpus/`](../../eval-corpus/), tracked with **Git LFS**
  (active). Keeps `git clone` fast and never bloats the open-core history — see
  `eval-corpus/README.md` for the policy.

Existing fixtures here are **never** retrofitted into LFS — that would be a
history rewrite, and the golden anchors must stay byte-plain.

## Git notes

`pdfs/` and `snapshots/` are committed to plain git. `blazegraph-io/.gitignore`
carries a `*.json` rule with an exception for `test_fixtures/**/*.json`, so the
snapshot JSON is tracked despite the global ignore.
