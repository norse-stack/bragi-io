# bragi-io

The **Bragi** command-line tool. It turns a document into a **bgraph** — one structured, addressable graph of its sections, paragraphs, and content, with the coordinates to point back at the page each piece came from. The bgraph is the product; this CLI is one way to get one.

`cargo install bragi-io` installs a binary named `bragi`.

![PDF, DOCX and Markdown converge through Bragi into one bgraph, serialized as bgraph.md and bgraph.json](https://cdn.jsdelivr.net/gh/norse-stack/bragi-io@main/docs/assets/convergence.svg)

PDF, DOCX, and Markdown all converge to the same graph, emitted two ways: `bgraph.md` (canonical, human-readable) and `bgraph.json` (the machine escape hatch).

## Install

```bash
cargo install bragi-io
```

On first use for a PDF, `bragi` fetches a Java runtime (used for PDF text extraction) and caches it; later runs reuse it. DOCX and Markdown parse in pure Rust, no runtime needed.

## A real parse

One parse of the canonical example — *Attention Is All You Need* — with the default config:

```bash
bragi parse -i attention.pdf -o attention.bgraph.json
```

```
✅ Graph: 179 nodes
```

That's **179 nodes** (1 document, 30 sections, 147 paragraphs, 1 margin). Each node carries a semantic location (its `path` in the tree) and, for PDFs, a physical one (page + bounding box):

```json
{
  "id": "7962788f-d2e7-50bc-8359-c47c4b37c03d",
  "node_type": "Section",
  "location": {
    "semantic": { "path": "2", "depth": 1, "breadcrumbs": ["Attention Is All You Need", "Attention Is All You Need"] },
    "physical": { "page": 1, "bounding_box": { "x": 211.5, "y": 149.1, "width": 188.4, "height": 16.5 } }
  },
  "content": { "text": "Attention Is All You Need" },
  "token_count": 6
}
```

## Two serializations

One graph, emitted two ways:

```bash
bragi parse -i attention.pdf -o attention.bgraph.json            # bgraph.json  (default output)
bragi parse -i attention.pdf -f bgraph-md -o attention.bgraph.md     # bgraph.md
```

- **`bgraph.md`** is canonical Markdown — readable and diffable, with each node's metadata in a fenced block beside its text.
- **`bgraph.json`** is the full graph — every node, bounding box, and token count.

Both invert to the same graph, and a bgraph carries a `bgraph_sha256` that proves the serialization round-trips.

## Usage

```bash
# Input format is detected from the extension (.pdf, .docx, .md, .bgraph.md)
bragi parse -i document.pdf -o document.bgraph.json

# Canonical Markdown instead of JSON
bragi parse -i document.pdf -f bgraph-md -o document.bgraph.md

# Tune parsing with a config (PDF channel)
bragi parse -i contract.pdf -c my-config.yaml -o contract.bgraph.json

# Turn a bgraph.md back into plain Markdown
bragi strip -i document.bgraph.md -o document.md
```

### Output formats

`-f, --output-format` selects the serialization:

| Value | Emits |
|-------|-------|
| `bgraph` *(default)* | the `bgraph.json` graph serialization |
| `bgraph-md` | the `bgraph.md` canonical Markdown serialization |
| `sequential`, `flat`, `markdown` | flatter projections — ordered JSON segments, JSON text chunks, or plain Markdown |

`markdown` (plain Markdown) is the inverse of Markdown *input* — it only applies to reflow documents. On a PDF that carries page-anchored nodes (headers, footers, margins) it errors; use `bgraph-md` for PDFs.

Run `bragi parse --help` for every flag (config, caching, JRE path, style info, and more).

## Stable names

A node's id is derived from its content and its place in the tree — not from which version parsed it. Reparse after editing one paragraph and only that paragraph's id changes; every other node keeps its id. That edit-locality is what makes a bgraph safe to save and build on.

## As a library

To embed the parser in a Rust program, depend on [`bragi-io-core`](https://crates.io/crates/bragi-io-core) instead of shelling out to this CLI.

## Documentation

- [Quickstart](https://github.com/norse-stack/bragi-io/blob/main/docs/guides/01-quickstart.md)
- [Schema reference](https://github.com/norse-stack/bragi-io/blob/main/docs/reference/02-schema-reference.md) — the `bgraph.json` fields
- [Configuration reference](https://github.com/norse-stack/bragi-io/blob/main/docs/reference/03-config-reference.md)

## License

Licensed under either of Apache License, Version 2.0 or MIT license, at your option.
