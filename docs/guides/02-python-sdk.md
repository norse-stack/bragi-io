# Python SDK Guide

The `bragi-io` Python package turns a PDF into a **bgraph** — one structured, addressable graph of its sections, paragraphs, and content — and hands it back as fully typed Python objects. The bgraph is the product; this SDK is one door onto it.

```bash
pip install bragi-io
```

**Requirements:** Python 3.9+. The only runtime dependency is `httpx`.

---

## Quick Start

```python
import bragi

bgraph = bragi.parse_pdf("attention.pdf")

print(bgraph)                # <BragiGraph: 179 nodes, schema v1.1.0>
print(len(bgraph.sections))  # 30

for section in bgraph.sections:
    print(section.content.text)
```

On first use, the SDK fetches the `bragi` binary and a Java runtime automatically, then caches them; later runs reuse them.

---

## Run modes

The SDK is one interface over one pure function — where the parse *runs* is a one-line change, and your graph code stays the same.

### Local (default)

Runs the `bragi` binary via subprocess. Synchronous — blocks until the parse is done.

```python
bgraph = bragi.parse_pdf("document.pdf")
```

### Self-hosted

Point the SDK at your own server (see the [Docker Guide](./03-docker.md)). Pass the server's base URL — including the scheme:

```python
bragi.configure(url="http://localhost:8080")
bgraph = await bragi.parse_pdf_async("document.pdf")
```

### Hosted

Point at the managed API by setting a key:

```python
bragi.configure(api_key="...")
bgraph = await bragi.parse_pdf_async("document.pdf")
```

`parse_pdf_async` requires a configured `url` or `api_key`; without one it raises. Same input, same graph, whichever mode you use.

---

## The BragiGraph object

Every `parse_pdf` call returns a `BragiGraph` — the top-level wrapper.

```python
bgraph = bragi.parse_pdf("document.pdf")

bgraph.nodes                    # list[DocumentNode] — all nodes
bgraph.sections                 # Section nodes only
bgraph.paragraphs               # Paragraph nodes only
bgraph.root                     # the Document root
bgraph.document_info            # DocumentInfo — metadata about the document
bgraph.structural_profile       # StructuralProfile — graph statistics
bgraph.schema_version           # "1.1.0"
bgraph.bgraph_sha256            # the round-trip integrity hash
bgraph.parse_provenance         # ParseProvenance — version, source, config
```

### Lookup helpers

```python
node = bgraph.get_node("7962788f-d2e7-50bc-8359-c47c4b37c03d")   # by id
page_nodes = bgraph.nodes_by_page(1)                             # all nodes on a page
```

### Serialization

```python
bgraph.to_dict()    # the raw bgraph.json dict
bgraph.to_json()    # JSON string
```

---

## Working with nodes

Each node is a `DocumentNode` with typed fields:

```python
node = bgraph.sections[0]

node.id                        # str (UUID) — content-derived, stable
node.node_type                 # str — "Document", "Section", "Paragraph", ...
node.content.text              # str — the node's text
node.token_count               # int — pre-computed token count

# Semantic location (tree position) — always present
node.location.semantic.path          # "2"
node.location.semantic.depth         # 1
node.location.semantic.breadcrumbs   # ["Attention Is All You Need", "Attention Is All You Need"]

# Physical location (page position) — present for PDFs
node.location.physical.page          # 1
node.location.physical.bounding_box  # BoundingBox(x=211.5, y=149.1, width=188.4, height=16.5)

# Tree relationships
node.parent                    # str | None — parent id
node.children                  # list[str] — child ids
```

### Tree navigation

```python
parent = node.get_parent(bgraph)      # DocumentNode | None
children = node.get_children(bgraph)  # list[DocumentNode]
```

---

## Rendering text

`render()` recursively collects text from a node and its descendants.

### Plain

```python
section = bgraph.sections[0]
print(section.render(bgraph))
```

### With breadcrumbs

```python
print(section.render(bgraph, breadcrumbs=True))
```

```
[Attention Is All You Need > Attention Is All You Need]

...the section's text, and its descendants...
```

### With node types

```python
print(section.render(bgraph, node_types=True))
```

```
[Section] Attention Is All You Need

[Paragraph] Provided proper attribution is provided...
```

### Full document

```python
print(bgraph.render())
```

---

## Error handling

```python
from bragi.errors import (
    BragiError,           # base exception
    BragiAuthError,       # bad/missing API key (hosted)
    BragiCreditsError,    # insufficient credits (hosted)
    BragiProcessingError, # the parse failed
    BragiNotFoundError,   # the bragi binary wasn't found (local)
)

try:
    bgraph = bragi.parse_pdf("document.pdf")
except BragiNotFoundError:
    print("bragi not found — it should auto-download on first run")
except BragiProcessingError as e:
    print(f"parse failed: {e}")
```

---

## Local mode details

### Runtime management

Runtime artifacts live inside the package directory (`site-packages/bragi/_runtime/`). `pip uninstall` is a clean removal.

### Binary resolution order

1. `BRAGI_CLI_PATH` environment variable (override)
2. `_runtime/bin/bragi` (package-local, from a previous download)
3. `bragi` on `PATH`, then `~/.cargo/bin/bragi` (installed via `cargo install`)
4. Download from GitHub Releases

The JRE resolves from `JAVA_HOME` if set, otherwise the package-local `_runtime/jre/` (the CLI downloads it there on first PDF parse).

### Config file (local mode only)

```python
bgraph = bragi.parse_pdf("document.pdf", config="path/to/config.yaml")
```

See the [Configuration Reference](../reference/03-config-reference.md).

---

## Node Types

The tree root is a `Document`; its descendants are `Section` and `Paragraph`, with the occasional `Margin` on PDFs.

| Type | Description | Typical depth |
|------|-------------|---------------|
| `Document` | Root node (one per graph) | 0 |
| `Section` | Heading or structural division | 1+ |
| `Paragraph` | Merged text block | 1+ |
| `Margin` | Marginal / running content (PDF) | varies |

`node_type` is a string, so you can filter on any value the parser emits. Parsing *Attention Is All You Need* produces 1 `Document`, 30 `Section`, 147 `Paragraph`, and 1 `Margin` node.

---

## Type Reference

All types are plain Python dataclasses with full IDE autocomplete. Import them from `bragi`.

| Type | Description |
|------|-------------|
| `BragiGraph` | Top-level graph wrapper — the return type of `parse_pdf` |
| `ParseProvenance` | The (version, source, config) triple that reproduces this parse |
| `DocumentNode` | A single node in the graph |
| `NodeLocation` | Combined semantic + physical location |
| `SemanticLocation` | Tree position (path, depth, breadcrumbs) |
| `PhysicalLocation` | Page position (page, bounding box) |
| `BoundingBox` | Position rectangle (x, y, width, height) |
| `NodeContent` | Node text (the `text` field) |
| `StyleMetadata` | Per-node style projection (font, size, bold/italic, colors) |
| `InternalRef` | A reference to a location within the same document |
| `InternalRefTarget` | Where an internal reference points (named / page) |
| `ExternalRef` | A reference to a location outside the document |
| `ExternalRefTarget` | Where an external reference points (a URI) |
| `TargetPoint` | Resolved destination point on a target page |
| `DocumentInfo` | Document-level metadata (root id, outline, flow type) |
| `DocumentMetadata` | Extracted fields plus per-channel namespaces |
| `PdfMetadata` | PDF-channel metadata namespace |
| `MdMetadata` | Markdown-channel metadata namespace (frontmatter) |
| `DocxMetadata` | DOCX-channel metadata namespace |
| `OcrMetadata` | OCR-channel metadata namespace (model, pages, companion PDF) |
| `BookmarkData` | Document outline (PDF bookmarks) |
| `BookmarkSection` | A single outline entry |
| `StructuralProfile` | Graph statistics |
| `TokenDistribution` | Per-type and overall token histograms |
| `TokenHistogram` | One token-count distribution |
| `HistogramBin` | One bin in a token histogram |
| `NodeTypeDistribution` | Node-type counts and percentages |
| `DepthDistribution` | Tree-depth statistics |
