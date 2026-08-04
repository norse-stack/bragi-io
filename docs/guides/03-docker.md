# Docker Guide

Run the Bragi parse server as a container — one HTTP door onto the same engine that produces a **bgraph**. The container bundles the `bragi` binary and its runtime, so there's no Rust toolchain or Java install to manage.

---

## Quick Start

### With docker-compose (recommended)

```bash
docker-compose up -d
```

The server starts on `http://localhost:8080`. Verify it's running:

```bash
curl http://localhost:8080/health
# {"status": "ok"}
```

### With docker run

```bash
docker build -t bragi-io .
docker run -d -p 8080:8080 bragi-io
```

---

## Parse via the Python SDK

Once the server is running, point the SDK at it:

```python
import bragi as bg

bg.configure(url="http://localhost:8080")
graph = await bg.parse_pdf_async("document.pdf")

for section in graph.sections:
    print(section.content.text)
```

This is the self-hosted tier — async processing without needing a hosted API key. Same output as local mode, same `Bragi` object.

---

## Parse via curl

```bash
curl -X POST http://localhost:8080/v1/parse/pdf \
  -F "file=@document.pdf" \
  -o bgraph.json
# {"success": true, "graph": { "schema_version": "1.0.0", "nodes": [ ... ] }}
```

### Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `file` | multipart | required | The PDF to parse |
| `config` | string | none | Path to a config YAML (inside the container) |
| `output_format` | string | `"graph"` | One of: `graph`, `sequential`, `flat` |

The server returns the `bgraph.json` graph. It does not emit the `bgraph.md` serialization — for that, run the CLI with `-f bgraph-md` (see the [Quickstart](./01-quickstart.md)).

---

## One-off CLI parses

You can also use the container for one-off CLI parses without starting the server:

```bash
docker run --rm -v $(pwd):/data bragi-io \
  bragi parse -i /data/document.pdf -o /data/bgraph.json
```

---

## What's in the container

The Docker image bundles:

- **bragi** — the compiled Rust binary
- **JRE** (BellSoft Liberica OpenJDK 21) — for Apache Tika PDF extraction
- **Apache Tika** — PDF text extraction via JNI
- **FastAPI + uvicorn** — the processing server
- **Fonts** (URW Base35, Liberation, DejaVu) — critical for accurate bounding box calculations on PDFs with non-embedded fonts

The image is built in two stages: Rust compilation, then a slim runtime image.

---

## Custom config

To use a custom config file, mount it into the container:

```bash
docker run -d -p 8080:8080 \
  -v $(pwd)/my-config.yaml:/config/my-config.yaml \
  bragi-io
```

Then reference it in your API call:

```bash
curl -X POST http://localhost:8080/v1/parse/pdf \
  -F "file=@document.pdf" \
  -F "config=/config/my-config.yaml"
```

---

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level for the CLI (`debug`, `info`, `warn`, `error`) |
| `BRAGI_CLI_PATH` | `/app/bin/bragi` | Path to the CLI binary |
| `BRAGI_JAR_PATH` | `/app/bin/blazing-tika-jni.jar` | Path to the Tika JAR |
| `BRAGI_CONFIG_PATH` | `/app/bin/config.yaml` | Default config file |

---

## Health check

The container includes a built-in health check on the `/health` endpoint. Docker will mark the container as healthy once the server is ready to accept requests.

```bash
docker inspect --format='{{.State.Health.Status}}' <container_id>
```
