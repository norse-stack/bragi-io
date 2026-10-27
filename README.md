# Bragi

**Bragi turns a document into a bgraph** — one structured, addressable graph of its sections, paragraphs, and content, with the coordinates to point back at the page each piece came from.

The bgraph is the thing you keep. The parser, the CLI, and the SDKs are just how you get one.

![Three containers, one tree: the same chapter as a PDF, a DOCX and a Markdown file, parsed into one heading tree whose nodes carry content-derived ids](https://cdn.jsdelivr.net/gh/norse-stack/bragi-io@main/docs/assets/convergence.svg)

Two convergences, and both are real:

- **Many inputs, one graph.** PDF, DOCX, and Markdown all land in the same bgraph. The picture is a real parse: the first six nodes of the Rust Book's ownership chapter, with their ids.
- **Many doors, one graph.** The CLI, the Rust and Python SDKs, and a self-hosted server are front doors onto the same pure function: bytes in, bgraph out. Where the parse runs is a swap, not a rewrite.

**Want to see what a bgraph can do?** [Try the live demo](https://bragi-io.com/app/) — drop in a PDF and explore the graph it produces.

---

## A real bgraph

Everything below is one parse of one file — *From Local to Global: A GraphRAG Approach to Query-Focused Summarization* (Edge et al., Microsoft, arXiv:2404.16130, CC BY 4.0), the canonical example ([sources](crates/core/test_fixtures/pdfs/SOURCES.md)) — with the default config:

```bash
bragi parse -i graphrag.pdf -o graphrag.bgraph.json
```

```
Parsing PDF graphrag.pdf
Graph: 368 nodes
Wrote bgraph JSON to graphrag.bgraph.json
```

That graph is **368 nodes**: 1 document root, 54 sections, 312 paragraphs, 1 margin — 21,601 tokens, tree depth 5. Every node looks like this (abridged):

```json
{
  "id": "97488a75-0154-50d2-a24a-66b52e793154",
  "node_type": "Section",
  "location": {
    "semantic": {
      "path": "1",
      "depth": 1,
      "breadcrumbs": ["From Local to Global: A GraphRAG Approach to Query-Focused Summarization", "From Local to Global: A GraphRAG Approach to Query-Focused Summarization"]
    },
    "physical": {
      "page": 1,
      "bounding_box": { "x": 121.7, "y": 100.1, "width": 368.7, "height": 36.5 }
    }
  },
  "content": { "text": "From Local to Global: A GraphRAG Approach to Query-Focused Summarization" },
  "token_count": 17,
  "parent": "042eaee9-abb9-5230-903e-2d67ff3a2918",
  "children": ["0696d509-1b6e-50a5-97e6-09da4e414e2a", "92daf96a-3dd4-5244-adb7-c7a0e5964037", "..."]
}
```

Two locations on every node — a **semantic** one (where it sits in the tree) and a **physical** one (where it sits on the page). A human says "the title, page 1"; a machine says `path: "1", page: 1, bbox: {x: 121.7, y: 100.1}`. Both point at the same content, and the physical coordinate lets you ground an answer back on the original page.

---

## Markdown is canonical, JSON is the superpower

One graph, two serializations — not two formats:

```bash
bragi parse -i graphrag.pdf -o graphrag.bgraph.json          # bgraph.json  (default output)
bragi parse -i graphrag.pdf -f bgraph-md -o graphrag.bgraph.md   # bgraph.md
```

- **`bgraph.md` is canonical.** It's Markdown — you can open it, read it, and diff it like any document. The structure rides in the syntax, with each node's metadata in a fenced block beside its text.
- **`bgraph.json` is the escape hatch.** When a machine wants the whole graph — every node, every bounding box, every token count — the JSON hands it over whole.

Neither is more true than the other; they invert to the same graph.

---

## The names are stable

A bgraph is worth saving because its pieces keep their names. A node's id is derived from its content and its place in the tree — not from which version parsed it, or when. So you can reparse after an edit and tell a real change from a re-emit:

![Edit one paragraph, re-parse: the ids hold. The same six nodes before and after one line of the second paragraph was changed; only that paragraph's id moved](https://cdn.jsdelivr.net/gh/norse-stack/bragi-io@main/docs/assets/bgraph-edit-diff.svg)

Edit one paragraph, run `bragi parse` again: only the edited paragraph gets a new id. Every other section and paragraph keeps its id byte-for-byte (the chapter above has over a hundred nodes; one moved). That edit-locality is what lets another tool hold a reference to a node and trust it still points at the same thing.

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

On first use for a PDF, the CLI fetches a Java runtime (used for PDF text extraction) and caches it; later runs reuse it. DOCX, Markdown, and OCR JSON parse in pure Rust, no runtime needed.

---

## Parse a document

**CLI** — output goes to a file with `-o`; input format is detected from the extension (`.pdf`, `.docx`, `.md`, `.bgraph.md`, `.json` for OCR payloads):

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
use bragi_io_core::DocumentProcessor;
use std::path::Path;

// The PDF channel runs Tika over JNI, so the processor needs a JRE and the
// Tika jar. `bragi parse` downloads and locates both; when embedding, pass
// the paths yourself (the jar ships in the repository under `crates/core/deps/tika/jni-jars/`).
let mut processor = DocumentProcessor::new_cli_jni(Path::new(jre_dir), Path::new(tika_jar))?;
let (graph, provenance) = processor.process_document("document.pdf")?;
println!("{} nodes, parsed by bragi {}", graph.nodes.len(), provenance.bragi_version);
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
curl -X POST http://localhost:8080/v1/parse/pdf --data-binary @graphrag.pdf
# {"success": true, "graph": { "schema_version": "1.2.0", "nodes": [ ... ] }}
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

The licences cover the code, not the names: see [TRADEMARKS.md](TRADEMARKS.md). For bug reports, feature requests and the current policy on pull requests, see [CONTRIBUTING.md](CONTRIBUTING.md).

An open format needs an open producer: the guarantee you build on is that you can always make and read a bgraph yourself.
