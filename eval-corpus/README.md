# eval-corpus/ — bulk evaluation fixtures (Git LFS)

The dedicated home for **large, bulk evaluation fixtures** — the multi-document
corpus we score the parser against, distinct from the small, hand-curated
unit-test fixtures under `blazegraph-core/test_fixtures/`.

Everything under this folder is tracked with **Git LFS** so heavy binaries
(PDFs, generated JSON, docx) never bloat the plain-git history of the open-core
repo. This keeps `git clone` fast for everyone who just wants the code.

## What belongs here

- Bulk PDF / docx source corpora for evaluation and scoring.
- Their generated derivatives (graph JSON, score snapshots) when committed.

## What does NOT belong here

- **Trust-critical golden text** — `document.bgraph.md`, `document.bgraph.json`,
  `config.yaml`, `PRODUCED_BY`. These stay in `blazegraph-core/test_fixtures/golden/`
  in **plain git**: the byte-honest golden freeze must diff *real* bytes, never an
  LFS pointer. (See `test_fixtures/README.md` and the `.gitattributes` rationale.)
- Small hand-authored unit-test fixtures — those stay in `test_fixtures/`.

## Activation (one-time, when the first corpus lands)

The `.gitattributes` LFS rules for `eval-corpus/**` are staged but **commented
out**, because git-lfs is not yet initialized in this repo and an armed
`filter=lfs` attribute with no lfs installed is a foot-gun. To turn it on:

```bash
git lfs install                       # once per machine
# then uncomment the `eval-corpus/**` block in ../.gitattributes:
#   eval-corpus/**/*.pdf   filter=lfs diff=lfs merge=lfs -text
#   eval-corpus/**/*.json  filter=lfs diff=lfs merge=lfs -text
#   eval-corpus/**/*.docx  filter=lfs diff=lfs merge=lfs -text
git add .gitattributes
```

Existing fixtures elsewhere are **never** retrofitted into LFS — that would be a
history rewrite, and the golden anchors must stay byte-plain. New corpus blobs
added here *after* activation land in LFS from birth.

Until then this folder is reserved and documented, so the convention is settled
before the bytes arrive — no bleeding into plain git later.
