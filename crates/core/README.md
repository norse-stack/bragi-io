# bragi-io-core

The core library behind **Bragi**. It turns a document into a **bgraph** — one structured, addressable graph of its sections, paragraphs, and content, with the coordinates to point back at the page each piece came from. This crate is the engine; the [`bragi-io`](https://crates.io/crates/bragi-io) CLI and the Bragi server are thin shells around it.

![PDF, DOCX and Markdown converge through Bragi into one bgraph, serialized as bgraph.md and bgraph.json](https://cdn.jsdelivr.net/gh/norse-stack/bragi-io@main/docs/assets/convergence.svg)

PDF, DOCX, and Markdown all converge to the same graph, serialized two ways: `bgraph.md` (canonical, human-readable) and `bgraph.json` (the machine escape hatch, `SortedDocumentGraph`).

## What a bgraph looks like

One parse of the canonical example — *Attention Is All You Need* — is **179 nodes** (1 document, 30 sections, 147 paragraphs, 1 margin). Each node carries a semantic location (its `path` in the tree) and, for PDFs, a physical one (page + bounding box):

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

let (graph, provenance) = processor.process_document("attention.pdf")?;

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
