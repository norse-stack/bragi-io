# bragi-io-core

Core library for semantic document graph processing. Parses PDFs into structured, queryable graphs with bounding boxes.

This is the library crate that powers [`bragi-io`](https://crates.io/crates/bragi-io) (the CLI) and the [Bragi API](https://bragi-io.com).

## Usage

```rust
use bragi_io_core::{DocumentProcessor, ParsingConfig};

let config = ParsingConfig::default();
let processor = DocumentProcessor::new(config);
let graph = processor.process_pdf("document.pdf")?;

for node in &graph.nodes {
    println!("{}: {}", node.node_type, node.content.text);
    if let Some(physical) = &node.location.physical {
        println!("  Page {}, bbox: {:?}", physical.page, physical.bounding_box);
    }
}
```

## Features

- `jni-backend` (default) — Uses JNI to call Apache Tika for PDF text extraction. Requires a JRE (the CLI auto-downloads one; if using the library directly, provide your own).

## When to use this vs the CLI

Use **bragi-io-core** when you want to embed the parser in your own Rust application. Use **bragi-io** (the CLI crate) for command-line usage.

## Documentation

- [Schema Reference](https://github.com/norse-stack/bragi-io/blob/main/docs/reference/02-schema-reference.md) — Full `bgraph.json` field documentation
- [Configuration Reference](https://github.com/norse-stack/bragi-io/blob/main/docs/reference/03-config-reference.md) — Tuning for your document type

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.
