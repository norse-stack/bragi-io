# Bragi

**Bragi turns a document into a bgraph** — one structured, addressable graph of its sections, paragraphs, and content, with the coordinates to point back at the page each piece came from.

The bgraph is the thing you keep. The parser, the CLI, and the SDKs are just how you get one.

![PDF, DOCX and Markdown converge through Bragi into one bgraph, serialized as bgraph.md and bgraph.json](https://cdn.jsdelivr.net/gh/norse-stack/bragi-io@main/docs/assets/convergence.svg)

Two convergences meet on that one picture, and both are real:

- **Many inputs, one graph.** PDF, DOCX, and Markdown all land in the same bgraph.
- **Many doors, one graph.** The CLI, the Rust and Python SDKs, and a self-hosted server are front doors onto the same pure function: bytes in, bgraph out. Where the parse runs is a swap, not a rewrite.

**Want to see what a bgraph can do?** [Try the live demo](https://bragi-io.com/app/) — drop in a PDF and explore the graph it produces.

---

## A real bgraph

Everything below is one parse of one file — *Attention Is All You Need*, the canonical example — with the default config:

```bash
bragi parse -i attention.pdf -o attention.bgraph.json
```

```
✅ Graph: 179 nodes
```

That graph is **179 nodes**: 1 document root, 30 sections, 147 paragraphs, 1 margin — 10,012 tokens, tree depth 5. Every node looks like this (abridged):

```json
{
  "id": "7962788f-d2e7-50bc-8359-c47c4b37c03d",
  "node_type": "Section",
  "location": {
    "semantic": {
      "path": "2",
      "depth": 1,
      "breadcrumbs": ["Attention Is All You Need", "Attention Is All You Need"]
    },
    "physical": {
      "page": 1,
      "bounding_box": { "x": 211.5, "y": 149.1, "width": 188.4, "height": 16.5 }
    }
  },
  "content": { "text": "Attention Is All You Need" },
  "token_count": 6,
  "parent": "6b149bb8-87e8-5e54-8e0f-b5fae9cba8f6",
  "children": ["ab92b870-4886-5a62-98bf-8be145f22d82", "47b86b72-a5b1-54b3-b4cc-6f24b31b6fd6", "..."]
}
```

Two locations on every node — a **semantic** one (where it sits in the tree) and a **physical** one (where it sits on the page). A human says "the title, page 1"; a machine says `path: "2", page: 1, bbox: {x: 211.5, y: 149.1}`. Both point at the same content, and the physical coordinate lets you ground an answer back on the original page.

---

## Markdown is canonical, JSON is the superpower

One graph, two serializations — not two formats:

```bash
bragi parse -i attention.pdf -o attention.bgraph.json          # bgraph.json  (default output)
bragi parse -i attention.pdf -f bgraph-md -o attention.bgraph.md   # bgraph.md
```

- **`bgraph.md` is canonical.** It's Markdown — you can open it, read it, and diff it like any document. The structure rides in the syntax, with each node's metadata in a fenced block beside its text.
- **`bgraph.json` is the escape hatch.** When a machine wants the whole graph — every node, every bounding box, every token count — the JSON hands it over whole.

Neither is more true than the other; they invert to the same graph.

---

## The names are stable

A bgraph is worth saving because its pieces keep their names. A node's id is derived from its content and its place in the tree — not from which version parsed it, or when. So you can reparse after an edit and tell a real change from a re-emit:

![Reparsing after a one-paragraph edit: only the edited node's id changes; every other node keeps its id](https://cdn.jsdelivr.net/gh/norse-stack/bragi-io@main/docs/assets/bgraph-edit-diff.svg)

Edit one paragraph, run `bragi parse` again: only the edited paragraph gets a new id. Every other section and paragraph keeps its id byte-for-byte. That edit-locality is what lets another tool hold a reference to a node and trust it still points at the same thing.

The same graph aims to be **portable** (a `bgraph.md` written by one version reads back in another, ids intact) and **self-verifiable** (it carries a `bgraph_sha256` that proves the serialization round-trips). These are hard promises and we're still early — so rather than ask for your trust, we make the guarantee *checkable*: every graph carries a `schema_version`, and any structural change in the parser moves it, so drift is something you can *see* coming, not something that surprises you. If a new parser version changes the output in a way you don't want, pin the previous one and nothing moves under you. That's what makes a bgraph something you can build on, not a receipt you read once and throw away.

---

## Install

**Rust CLI** — installs a binary named `bragi`:

```bash
cargo install bragi-io
```

**Python:**

```bash
pip install bragi-io
```

On first use for a PDF, the CLI fetches a Java runtime (used for PDF text extraction) and caches it; later runs reuse it. DOCX and Markdown parse in pure Rust, no runtime needed.

---

## Parse a document

**CLI** — output goes to a file with `-o`; input format is detected from the extension (`.pdf`, `.docx`, `.md`, `.bgraph.md`):

```bash
bragi parse -i document.pdf -o document.bgraph.json
```

**Python** — the SDK returns a fully typed graph:

```python
import bragi

bgraph = bragi.parse_pdf("document.pdf")

print(f"{len(bgraph.nodes)} nodes, {len(bgraph.sections)} sections")

for section in bgraph.sections:
    print(section.content.text)
    print(section.location.physical.page)
```

**Rust** — embed the parser directly:

```rust
use bragi_io_core::{DocumentProcessor, ParsingConfig};

let processor = DocumentProcessor::new(ParsingConfig::default());
let bgraph = processor.process_pdf("document.pdf")?;
```

---

## Output formats

`bragi parse -f <format>` picks the serialization. The two that matter are the default JSON graph and the canonical Markdown:

| `-f` value | What it emits |
|------------|---------------|
| `bgraph` *(default)* | the `bgraph.json` serialization — the full graph |
| `bgraph-md` | the `bgraph.md` serialization — canonical Markdown with node fences |
| `sequential`, `flat`, `markdown` | flatter projections — ordered JSON segments, JSON text chunks, or plain Markdown |

`markdown` (plain Markdown) only applies to reflow documents — on a PDF that carries page-anchored nodes (headers, footers, margins) it errors; use `bgraph-md` for PDFs.

`bragi strip` turns a `bgraph.md` back into plain Markdown (fences removed, metadata optionally lifted to frontmatter). Run `bragi parse --help` for the full flag surface.

---

## Run it as a service

A small FastAPI server wraps the same binary, for parsing over HTTP:

```bash
make serve      # http://localhost:8080
```
```bash
curl -X POST http://localhost:8080/v1/parse/pdf --data-binary @attention.pdf
# {"success": true, "graph": { "schema_version": "1.0.0", "nodes": [ ... ] }}
```

Point the Python SDK at it and your parsing code doesn't change:

```python
bragi.configure(url="http://localhost:8080")
bgraph = await bragi.parse_pdf_async("document.pdf")
```

The server ships in a container that bundles the binary and its runtime — see the [Docker guide](docs/guides/03-docker.md).

**Scaling or automating a GraphRAG pipeline?** The hosted API runs this same parse behind an API key — same bgraph out, nothing changes in your code but the endpoint. See the [full product lineup](https://bragi-io.com).

---

## The pieces

```
bragi-io/
├── crates/core/   # the parsing library — bragi-io-core on crates.io
├── crates/cli/    # the bragi command-line tool — bragi-io on crates.io
├── py/sdk/        # the typed Python SDK — bragi-io on PyPI
├── py/server/     # the self-hosted FastAPI server
└── docs/          # guides + reference
```

## Documentation

- [Quickstart](docs/guides/01-quickstart.md) — parse your first document
- [Python SDK guide](docs/guides/02-python-sdk.md) — typed access, tree navigation, rendering
- [Docker guide](docs/guides/03-docker.md) — run the self-hosted server
- [Schema reference](docs/reference/02-schema-reference.md) — the `bgraph.json` fields
- [Configuration reference](docs/reference/03-config-reference.md) — tuning parse rules

## Open

Bragi core is open, under both licenses at your option:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

An open format needs an open producer: the guarantee you build on is that you can always make and read a bgraph yourself.
