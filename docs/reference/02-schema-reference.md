# bgraph.json Schema Reference

Complete field-by-field documentation of the Bragi output format (`bgraph.json`).

**Schema version:** `1.1.0`
**Source of truth:** [`types.rs`](../../crates/core/src/types.rs)

All examples are from processing Claude Shannon's *A Mathematical Theory of Communication* (55 pages).

---

## Top-Level: SortedDocumentGraph

The root object of the `bgraph.json` output.

```json
{
  "schema_version": "1.1.0",
  "bgraph_sha256": "f6d2fcf0…",
  "created_at": "1970-01-01T00:00:00Z",
  "parse_provenance": { ... },
  "nodes": [ ... ],
  "document_info": { ... },
  "structural_profile": { ... }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `schema_version` | string | Output format version. Currently `"1.1.0"`. Check this to detect schema changes. |
| `bgraph_sha256` | string (hex) | SHA-256 content address of the canonical graph. Identical inputs produce an identical hash; `created_at` is excluded so the address stays stable. |
| `created_at` | string (ISO 8601) | When the graph was generated. A side-effect field — **not** part of `bgraph_sha256`. |
| `parse_provenance` | object | How the graph was produced. See [ParseProvenance](#parseprovenance). |
| `nodes` | array | All nodes in the document tree, sorted by `text_order`. |
| `document_info` | object | Document-level metadata and structure. Not a node — information *about* the document. |
| `structural_profile` | object | Statistical properties of the graph (node counts, token distributions, depth). |

---

## ParseProvenance

How the graph was produced — the inputs that determine it.

```json
{
  "bragi_version": "0.6.0",
  "source_format": "pdf",
  "source_sha256": "bdfaa68d…",
  "config_hash": "6daf2782…"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `bragi_version` | string | Version of Bragi that produced this graph. |
| `source_format` | string | Source format: `"pdf"`, `"markdown"`, `"docx"`, or `"ocr"`. |
| `source_sha256` | string (hex) | SHA-256 of the source document bytes. |
| `config_hash` | string (hex) | SHA-256 of the effective parsing config. |

---

## DocumentNode

Every element in the `nodes` array is a `DocumentNode`.

```json
{
  "id": "17113498-be4b-4bb5-88dd-80978ee00266",
  "node_type": "Section",
  "location": {
    "semantic": { "path": "2", "depth": 1, "breadcrumbs": ["shannon1948.dvi", "A Mathematical Theory of Communication"] },
    "physical": { "page": 1, "bounding_box": { "x": 181.0, "y": 127.9, "width": 265.5, "height": 8.0 } }
  },
  "text_order": 1,
  "content": { "text": "A Mathematical Theory of Communication" },
  "token_count": 9,
  "parent": "f49c0604-3794-41aa-a7d3-20e4a194a7eb",
  "children": ["d1ea207e-...", "480f520b-...", "..."]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | string (UUID) | Unique identifier for this node. |
| `node_type` | string | One of: `"Document"`, `"Section"`, `"Paragraph"`, `"Margin"`, `"Header"`, `"Footer"`, `"CodeBlock"`, `"List"`, `"Blockquote"`, `"Table"`, `"Equation"`. Treat as an open set — minor schema versions may add types. |
| `location` | object | Where this node exists — both in the tree and on the page. See [NodeLocation](#nodelocation). |
| `text_order` | integer? | Sequential reading order (0-indexed). `null` for the Document root. |
| `content` | object | The node's text content. See [NodeContent](#nodecontent). |
| `token_count` | integer | Pre-calculated token count for the node's text. Useful for RAG chunk sizing. |
| `parent` | string? (UUID) | Parent node ID. `null` for the Document root. |
| `children` | array (UUID[]) | Child node IDs, ordered by `text_order`. Empty for leaf nodes. |

### Node Types

| Type | Description | Typical depth | Has children? |
|------|-------------|---------------|---------------|
| `Document` | Root node. One per graph. | 0 | Yes — sections and top-level paragraphs |
| `Section` | Detected heading or structural division. | 1+ | Yes — paragraphs and nested sections |
| `Paragraph` | Merged, semantically coherent text block. | 2+ | No (leaf) |
| `Margin` | Page-anchored marginal text (page numbers, running heads) captured off the body. | 2+ | No (leaf) |
| `Header` | Page header (repeated content). | 2+ | No (leaf) |
| `Footer` | Page footer (repeated content). | 2+ | No (leaf) |
| `CodeBlock` | One code block, held verbatim (fence delimiters and language tag preserved). | 2+ | No (leaf) |
| `List` | One list block, held verbatim (bullets/numbering preserved). | 2+ | No (leaf) |
| `Blockquote` | One blockquote, held verbatim (`>` markers preserved). | 2+ | No (leaf) |
| `Table` | One table, held verbatim (pipe syntax preserved). | 2+ | No (leaf) |
| `Equation` | One display-math block, verbatim LaTeX as delivered by the source (schema 1.1.0). | 2+ | No (leaf) |

Each source channel produces a subset of this union: PDF emits `Section`/`Paragraph`/`Margin`, Markdown and DOCX add the verbatim block types, and OCR emits the widest set including `Header`/`Footer`/`Equation`. Filter on the types you care about rather than assuming which appear.

---

## NodeLocation

Every node has a location with two components: where it sits in the document tree (semantic) and where it appears on the physical page (physical).

```json
{
  "semantic": {
    "path": "2.3",
    "depth": 2,
    "breadcrumbs": ["shannon1948.dvi", "A Mathematical Theory of Communication"]
  },
  "physical": {
    "page": 1,
    "bounding_box": { "x": 91.9, "y": 585.9, "width": 427.5, "height": 164.2 }
  }
}
```

### SemanticLocation

Always present. Computed from the tree structure.

| Field | Type | Description |
|-------|------|-------------|
| `path` | string | Hierarchical position. `"2.3"` means 3rd child of 2nd top-level element. Empty string for root. |
| `depth` | integer | Tree depth. `0` = document root, `1` = top-level section, `2` = content within section. |
| `breadcrumbs` | string[] | Human-readable trail from root to this node. |

**Path notation:** The path is a dot-separated string of 1-indexed child positions. `"2.3.1"` means: the 1st child of the 3rd child of the 2nd child of root.

### PhysicalLocation

Present for PDFs. `null` for reflow formats (Markdown, DOCX — future).

| Field | Type | Description |
|-------|------|-------------|
| `page` | integer | Page number (1-indexed). |
| `bounding_box` | object | Position on the page in PDF coordinate space. |

### BoundingBox

Coordinates are in PDF points (1 point = 1/72 inch). Origin is top-left of the page.

| Field | Type | Description |
|-------|------|-------------|
| `x` | float | Horizontal position from left edge. |
| `y` | float | Vertical position from top edge. |
| `width` | float | Width of the bounding region. |
| `height` | float | Height of the bounding region. |

**The human-AI bridge:** Semantic and physical locations together let you map between tree positions and page coordinates. A human says "page 3, top paragraph" and a machine could programmatically use `path: "2.5", page: 3, bbox: {x: 72, y: 89}` — both pointing at the same content.

---

## NodeContent

```json
{
  "text": "The fundamental problem of communication is that of reproducing at one point either exactly or approximately a message selected at another point..."
}
```

| Field | Type | Description |
|-------|------|-------------|
| `text` | string | The node's text content, trimmed of leading/trailing whitespace. |

The `content` object is extensible. Future versions may add type-specific fields (e.g., `heading_level` for sections, `table_data` for tables).

---

## DocumentInfo

Document-level metadata. Not a node in the tree — information *about* the document.

```json
{
  "document_info": {
    "root_id": "f49c0604-3794-41aa-a7d3-20e4a194a7eb",
    "kind": "document",
    "document_metadata": { ... },
    "outline_data": { ... },
    "flow_type": "Fixed"
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `root_id` | string (UUID) | References the `Document` node in the `nodes` array — the tree root. |
| `kind` | string | The graph kind. Currently `"document"`. |
| `document_metadata` | object | Metadata extracted from the source format. See [DocumentMetadata](#documentmetadata). |
| `outline_data` | object? | The document outline — detected sections / PDF bookmarks. |
| `flow_type` | string | `"Fixed"` (PDF — has physical layout) or `"Free"` (Markdown, DOCX — reflows). Read the flow type from here. |

### DocumentMetadata

Universal fields sit at the top; format-specific metadata lives in a channel namespace (`pdf`, `md`, `docx`, or `ocr`) matching the source. At most one namespace is present, and it is absent entirely when the source carried no format-specific metadata. All fields are pass-through — Bragi doesn't infer or modify them.

```json
{
  "title": "shannon1948.dvi",
  "author": null,
  "description": null,
  "language": null,
  "created": "1998-07-16T10:14:40Z",
  "pdf": {
    "version": "1.2",
    "producer": "Acrobat Distiller Command 3.01 for Solaris 2.3 and later (SPARC)",
    "creator_tool": "dvipsk 5.58f Copyright 1986, 1994 Radical Eye Software",
    "publisher": null,
    "page_count": 55,
    "encrypted": false,
    "has_marked_content": false,
    "modified": null,
    "extras": { "pdf:hasXMP": "false", "...": "..." }
  }
}
```

**Universal fields:**

| Field | Type | Description |
|-------|------|-------------|
| `title` | string? | Document title. May be a filename if no title is set. |
| `author` | string? | Document author. |
| `description` | string? | Document description. |
| `language` | string? | Language tag (e.g., `"en"`, `"de"`). |
| `created` | string? | Creation timestamp (ISO 8601). |
| `pdf` / `md` / `docx` / `ocr` | object? | The channel namespace matching the source. At most one is present; absent when the source has no format-specific metadata. |

**`pdf` namespace:**

| Field | Type | Description |
|-------|------|-------------|
| `version` | string? | PDF specification version (e.g., `"1.2"`, `"1.7"`). |
| `producer` | string? | The PDF producer. |
| `creator_tool` | string? | The tool that created the document. |
| `publisher` | string? | Publisher, if present. |
| `page_count` | integer? | Number of pages. |
| `encrypted` | boolean? | Whether the PDF is encrypted. |
| `has_marked_content` | boolean? | Whether the PDF has tagged/marked content (accessibility structure). |
| `modified` | string? | Last modification timestamp (ISO 8601). |
| `extras` | object | Raw pass-through of remaining Tika metadata keys (string → string). |

**`ocr` namespace:**

| Field | Type | Description |
|-------|------|-------------|
| `model` | string? | The OCR model that produced the source payload. |
| `pages_processed` | integer? | Pages processed, as reported by the OCR run. |
| `doc_size_bytes` | integer? | Size of the original document the OCR run read, in bytes. |
| `dpi` | integer? | Raster resolution the pixel bounding boxes were reported at. |
| `companion_pdf_sha256` | string? (hex) | SHA-256 of the companion PDF whose native metadata was grafted in via `--companion-pdf`. Absent on single-arm parses. |
| `extras` | object | Raw pass-through of remaining payload fields. |

---

## StructuralProfile

Statistical properties of the document graph. Deterministic — computed mechanically from the tree structure.

```json
{
  "structural_profile": {
    "document_type": "Generic",
    "total_nodes": 94,
    "total_tokens": 35647,
    "token_distribution": { ... },
    "node_type_distribution": { ... },
    "depth_distribution": { ... }
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `document_type` | string | Currently defaults to `"Generic"` for all documents. |
| `total_nodes` | integer | Total nodes in the graph. |
| `total_tokens` | integer | Sum of all node token counts. |
| `token_distribution` | object | Token count histograms. |
| `node_type_distribution` | object | Node type counts and percentages. |
| `depth_distribution` | object | Tree depth statistics. |

> `created_at` is a top-level field (see [Top-Level](#top-level-sorteddocumentgraph)); `flow_type` lives in [`document_info`](#documentinfo).

### NodeTypeDistribution

```json
{
  "counts": { "Document": 1, "Section": 6, "Paragraph": 87 },
  "percentages": { "Document": 1.06, "Section": 6.38, "Paragraph": 92.55 }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `counts` | object | Node count per type. |
| `percentages` | object | Percentage of total nodes per type. |

### DepthDistribution

```json
{
  "max_depth": 3,
  "depth_counts": { "0": 1, "1": 6, "2": 87 },
  "avg_depth": 2.37
}
```

| Field | Type | Description |
|-------|------|-------------|
| `max_depth` | integer | Deepest level in the tree. |
| `depth_counts` | object | Node count at each depth level. |
| `avg_depth` | float | Average node depth. |

### TokenDistribution

Per-type and overall token count histograms. Useful for understanding chunk size distribution (e.g., for RAG).

```json
{
  "by_node_type": {
    "Paragraph": {
      "bins": [
        { "range_start": 0, "range_end": 100, "count": 30, "token_sum": 1500 },
        { "range_start": 100, "range_end": 200, "count": 25, "token_sum": 3750 },
        ...
      ],
      "total_count": 87,
      "total_tokens": 35000,
      "mean": 402.3,
      "median": 350.0,
      "mode": 100,
      "variance": 45000.0
    }
  },
  "overall": { ... }
}
```

**TokenHistogram:**

| Field | Type | Description |
|-------|------|-------------|
| `bins` | array | Bucketed distribution of token counts. |
| `total_count` | integer | Number of nodes in this histogram. |
| `total_tokens` | integer | Sum of all token counts. |
| `mean` | float | Mean token count per node. |
| `median` | float | Median token count. |
| `mode` | integer? | Most frequent bin's `range_start`. `null` if empty. |
| `variance` | float | Variance of token counts. |

**HistogramBin:**

| Field | Type | Description |
|-------|------|-------------|
| `range_start` | integer | Inclusive lower bound of this bin. |
| `range_end` | integer | Exclusive upper bound of this bin. |
| `count` | integer | Number of nodes with token counts in this range. |
| `token_sum` | integer | Total tokens across all nodes in this bin. |

---

## Navigating the Tree

The graph is a tree rooted at the `Document` node. Every node has `parent` and `children` fields that reference other nodes by ID.

### Index-based lookup

```python
nodes = {n["id"]: n for n in graph["nodes"]}

# Get any node by ID
node = nodes["17113498-be4b-4bb5-88dd-80978ee00266"]

# Walk children
for child_id in node["children"]:
    child = nodes[child_id]
```

### Find the root

```python
root = next(n for n in graph["nodes"] if n["node_type"] == "Document")
# Or use document_info:
root = nodes[graph["document_info"]["root_id"]]
```

### Filter by type

```python
sections = [n for n in graph["nodes"] if n["node_type"] == "Section"]
paragraphs = [n for n in graph["nodes"] if n["node_type"] == "Paragraph"]
```

### Get text by page

```python
page_3_nodes = [
    n for n in graph["nodes"]
    if n["location"]["physical"] and n["location"]["physical"]["page"] == 3
]
```

---

## Schema Versioning

The `schema_version` field (currently `"1.1.0"`) follows semver:

- **Major** (X.0.0): Breaking changes to existing fields
- **Minor** (0.X.0): New fields added (backwards compatible)
- **Patch** (0.0.X): Bug fixes to field values

`1.1.0` added the `Equation` node type and the `ocr` metadata namespace — additive, so `1.0.0` consumers keep working if they tolerate unknown node types and metadata keys.

Always check `schema_version` before parsing to handle schema evolution gracefully.
