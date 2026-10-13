# bragi-io-core

The core library behind **Bragi**. It turns a document into a **bgraph** — one structured, addressable graph of its sections, paragraphs, and content, with the coordinates to point back at the page each piece came from. This crate is the engine; the [`bragi-io`](https://crates.io/crates/bragi-io) CLI and the Bragi server are thin shells around it.

![PDF, DOCX and Markdown converge through Bragi into one bgraph, serialized as bgraph.md and bgraph.json](https://cdn.jsdelivr.net/gh/norse-stack/bragi-io@main/docs/assets/convergence.svg)

PDF, DOCX, and Markdown all converge to the same graph, serialized two ways: `bgraph.md` (canonical, human-readable) and `bgraph.json` (the machine escape hatch, `SortedDocumentGraph`).

## What a bgraph looks like

One parse of the canonical example — *From Local to Global: A GraphRAG Approach to Query-Focused Summarization* (Edge et al., Microsoft, arXiv:2404.16130, CC BY 4.0) — is **368 nodes** (1 document, 54 sections, 312 paragraphs, 1 margin). Each node carries a semantic location (its `path` in the tree) and, for PDFs, a physical one (page + bounding box):

```json
{
  "id": "97488a75-0154-50d2-a24a-66b52e793154",
  "node_type": "Section",
  "location": {
    "semantic": { "path": "1", "depth": 1, "breadcrumbs": ["From Local to Global: A GraphRAG Approach to Query-Focused Summarization", "From Local to Global: A GraphRAG Approach to Query-Focused Summarization"] },
    "physical": { "page": 1, "bounding_box": { "x": 121.7, "y": 100.1, "width": 368.7, "height": 36.5 } }
  },
  "content": { "text": "From Local to Global: A GraphRAG Approach to Query-Focused Summarization" },
  "token_count": 17
}
```

## Usage

`process_document` returns the in-memory `DocumentGraph` and the `ParseProvenance` for that run. PDF text extraction uses a bundled Java library, so the JNI backend takes a JRE and the extraction JAR:

```rust
use bragi_io_core::DocumentProcessor;
use std::path::Path;

// The JNI backend needs a JRE + the extraction JAR (used for PDF).
let mut processor = DocumentProcessor::new_cli_jni(
    Path::new("/path/to/jre"),
    Path::new("/path/to/extractor.jar"),
)?;

let (graph, provenance) = processor.process_document("graphrag.pdf")?;

println!("{} nodes, parsed by bragi {}", graph.nodes.len(), provenance.bragi_version);

// graph.nodes is a HashMap<NodeId, DocumentNode>
for node in graph.nodes.values() {
    println!("{}: {}", node.node_type, node.content.text);
    if let Some(physical) = &node.location.physical {
        println!("  page {}, bbox {:?}", physical.page, physical.bounding_box);
    }
}
```

To parse with a tuned config, use `process_document_with_config_file(input_path, config_path)`; for full control over caching and profiling, `process_document_with_cache(...)`.

## Stable names

A node's id is content-derived — a function of its content and its place in the tree, not of the parser version. Reparse after editing one paragraph and only that node's id changes; every other node keeps its id. The graph also carries a `bgraph_sha256` that proves its serialization round-trips (`SortedDocumentGraph::verify_identity`). Together that makes a bgraph portable across versions and safe to build on.

## Features

- `jni-backend` *(default)* — PDF text extraction via a JNI call into a bundled Java library. Needs a JRE at runtime (the CLI auto-downloads one; a library caller supplies the path). DOCX and Markdown channels are pure Rust and do not need it.
- `strict-identity` *(default)* — re-derive and assert graph identity on read, as a correctness belt. The library always exposes `ParseOptions.accept_drift` for programmatic callers regardless of this feature.

## When to use this vs the CLI

Reach for **bragi-io-core** to embed the parser in a Rust program. For command-line use, install [`bragi-io`](https://crates.io/crates/bragi-io) (it gives you the `bragi` binary).

## Documentation

- [Schema reference](https://github.com/norse-stack/bragi-io/blob/main/docs/reference/02-schema-reference.md) — the `bgraph.json` fields
- [Configuration reference](https://github.com/norse-stack/bragi-io/blob/main/docs/reference/03-config-reference.md)

## License

Licensed under either of Apache License, Version 2.0 or MIT license, at your option.
