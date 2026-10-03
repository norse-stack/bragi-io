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
# {"name": "Bragi API — Self-Hosted", "status": "healthy", "version": "0.6.1"}
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
import bragi

bragi.configure(url="http://localhost:8080")
bgraph = await bragi.parse_pdf_async("document.pdf")

for section in bgraph.sections:
    print(section.content.text)
```

This is the self-hosted tier — async processing without needing a hosted API key. Same output as local mode, same `BragiGraph` object.

---

## Parse via curl

```bash
curl -X POST http://localhost:8080/v1/parse/pdf \
  --data-binary @document.pdf \
  -o bgraph.json
# {"success": true, "graph": { "schema_version": "1.2.0", "nodes": [ ... ] }}
```

### Parameters

The request body is the raw PDF bytes (`--data-binary @doc.pdf`) — matching the hosted API, not a multipart form. One optional query parameter selects the output:

| Parameter | In | Default | Description |
|-----------|-----|---------|-------------|
| `format` | query | `json` | `json` returns the graph; `md` / `bgraph-md` / `bgraph` return the canonical bgraph.md text in a `bgraph_md` field |

The parse config is set once at container start via `BRAGI_CONFIG_PATH` (see [Environment variables](#environment-variables)), not per request.

By default the server returns the `bgraph.json` graph; pass `?format=md` to get the canonical `bgraph.md` text instead. (Over HTTP, `?format=bgraph` also returns the markdown — the opposite of the CLI's `-f bgraph`, which emits the JSON graph; the `?format=` selector mirrors the hosted API.)

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

The server applies the config at `BRAGI_CONFIG_PATH` to every parse. Mount your config into the container and point the env var at it:

```bash
docker run -d -p 8080:8080 \
  -v $(pwd)/my-config.yaml:/config/my-config.yaml \
  -e BRAGI_CONFIG_PATH=/config/my-config.yaml \
  bragi-io
```

Every `/v1/parse/pdf` request then parses with that config — no per-request parameter needed.

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
