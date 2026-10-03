# Quickstart

Parse your first document into a **bgraph** — one structured, addressable graph of its sections, paragraphs, and content, with the coordinates to point back at the page each piece came from.

The bgraph is the thing you keep. The CLI and SDK below are just how you get one.

---

## What Bragi does

PDF, DOCX, and Markdown all converge to the same graph, serialized two ways:

- **`bgraph.md`** — canonical Markdown you can read and diff.
- **`bgraph.json`** — the machine escape hatch: every node, bounding box, and token count.

Instead of flat text, you get a navigable tree of typed nodes, each with a semantic location (its place in the tree) and, for PDFs, a physical one (page + bounding box).

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

> On first use for a PDF, the CLI fetches a Java runtime (used for PDF text extraction) and caches it. DOCX, Markdown, and OCR JSON parse in pure Rust, no runtime needed.

---

## Parse a document

### CLI

Input format is detected from the extension (`.pdf`, `.docx`, `.md`, `.bgraph.md`, `.json`):

```bash
bragi parse -i document.pdf -o document.bgraph.json
```

A `.json` input is an OCR response payload (currently the Mistral OCR shape): the page blocks become the graph, equations land as `Equation` nodes, and `--companion-pdf original.pdf` grafts the original PDF's document metadata (title, author, …) into the result.

Want the canonical Markdown serialization instead?

```bash
bragi parse -i document.pdf -f bgraph-md -o document.bgraph.md
```

### Python

```python
import bragi

bgraph = bragi.parse_pdf("document.pdf")

print(bgraph)                # <BragiGraph: 179 nodes, schema v1.2.0>
print(len(bgraph.sections))

for section in bgraph.sections:
    print(section.content.text)
```

The CLI's default output and the SDK's graph are the same `bgraph.json`.

---

## Understand the output

Everything below is one parse of the canonical example — *Attention Is All You Need* — with the default config. That graph is **179 nodes**: 1 document root, 30 sections, 147 paragraphs, 1 margin (10,012 tokens, tree depth 5).

### The graph

```json
{
  "schema_version": "1.2.0",
  "bgraph_sha256": "f6d2fcf0d4b647b973d83197aefc3f7c8dd948ef7f259bb3eda2a7a8b2470299",
  "parse_provenance": { "bragi_version": "0.7.0", "source_format": "pdf", "...": "..." },
  "nodes": [ ... ],
  "document_info": { ... },
  "structural_profile": { ... }
}
```

| Field | Description |
|-------|-------------|
| `schema_version` | The bgraph format version (`"1.2.0"`). Check it to detect shape changes. |
| `bgraph_sha256` | An integrity hash proving this serialization round-trips back to the same graph. |
| `parse_provenance` | The (version, source, config) triple that reproduces this parse. |
| `nodes` | Every node in the document tree, sorted by reading order. |
| `document_info` | Metadata *about* the document (title, outline, flow type). |
| `structural_profile` | Node counts, token distributions, and depth stats. |

### A node

The tree's root is a `Document` node; its descendants are `Section` and `Paragraph` (and, for PDFs, the occasional `Margin`). A section looks like this:

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
  "children": ["ab92b870-4886-5a62-98bf-8be145f22d82", "..."]
}
```

### The location model

Every node has a **location** with two parts:

- **Semantic** (always present) — `path` (hierarchical position, `"2.4.1"`), `depth`, and `breadcrumbs` (a human-readable trail).
- **Physical** (PDFs) — `page` and `bounding_box` (`x`, `y`, `width`, `height` in PDF points).

This is the human-AI bridge: a person says "the title, page 1"; a machine says `path: "2", page: 1, bbox: {x: 211.5, y: 149.1}`. Both point at the same content, and the physical coordinate lets you ground an answer back on the original page (highlights, jump-to-content, overlays).

---

## The names are stable

A node's id is derived from its content and its place in the tree — not from when you parsed it. Reparse after editing one paragraph and only that paragraph's id changes; every other node keeps its id. That edit-locality is what lets you cache, diff, and build on a bgraph over time.

---

## Navigate the tree

Every node has `parent` and `children` (node ids). Walk it with plain JSON:

```python
import json

with open("document.bgraph.json") as f:
    bgraph = json.load(f)

nodes = {n["id"]: n for n in bgraph["nodes"]}
root = nodes[bgraph["document_info"]["root_id"]]

for child_id in root["children"]:
    child = nodes[child_id]
    if child["node_type"] == "Section":
        print(child["content"]["text"], "→ path", child["location"]["semantic"]["path"])
```

Or use the Python SDK for typed access:

```python
bgraph = bragi.parse_pdf("document.pdf")

for section in bgraph.sections:
    print(section.content.text)
    print(section.location.semantic.breadcrumbs)
    print(section.render(bgraph))
```

---

## Configuration

The default config works for most documents. For a specific document type, pass a YAML config that tunes section detection, spatial clustering, and size limits:

```bash
bragi parse -i contract.pdf -c my-config.yaml -o contract.bgraph.json
```

Build one config per document category (legal contracts, papers from one journal) and reuse it across that group. See the [Configuration Reference](../reference/03-config-reference.md).

---

## Run it as a service

A small server wraps the same binary, for parsing over HTTP:

```bash
make serve   # http://localhost:8080
```

Point the SDK at it and your code doesn't change:

```python
bragi.configure(url="http://localhost:8080")
bgraph = await bragi.parse_pdf_async("document.pdf")
```

See the [Docker Guide](./03-docker.md) to run the containerized server.

---

## What's next

- **[Python SDK Guide](./02-python-sdk.md)** — typed access, the three run modes, rendering
- **[Schema Reference](../reference/02-schema-reference.md)** — the `bgraph.json` fields
- **[Configuration Reference](../reference/03-config-reference.md)** — tune parsing for your document type
