"""FastAPI server wrapping the bragi binary.

Self-hosted parse API matching the hosted endpoint contract at
`api.bragi-io.com`: a **raw request body** in, the **official response
envelope** out. No auth, no billing — just document parsing.

Request/response contract (matches the hosted `/v1/parse/pdf` and the Python
SDK's remote-mode client, `bragi.client._handle_response`):
  * body    → the raw PDF bytes (`--data-binary @doc.pdf`), NOT multipart.
  * ?format= → `json` (default, returns the graph) or `md`/`bgraph-md`/`bgraph`
               (returns the canonical bgraph.md text in `bgraph_md`).
  * success → 200 `{"success": true, "graph": <SortedDocumentGraph>}`
              or   `{"success": true, "bgraph_md": "<markdown>"}`
  * error   → 4xx/5xx `{"success": false, "error": {"code": "...", "message": "..."}}`

The hosted response also carries `billing` and `pipeline_diagnostics`; those are
hosted-only (this server has no billing) and are omitted here.
"""

from __future__ import annotations

import json
import os
import subprocess
import tempfile
from pathlib import Path

from fastapi import FastAPI, Query, Request
from fastapi.responses import JSONResponse

app = FastAPI(
    title="Bragi API — Self-Hosted",
    description="Local document parsing API powered by bragi",
    version="0.5.0",
)

CLI_PATH = os.environ.get("BRAGI_CLI_PATH", "/app/bin/bragi")
JAR_PATH = os.environ.get("BRAGI_JAR_PATH", "/app/bin/blazing-tika-jni.jar")
DEFAULT_CONFIG_PATH = os.environ.get("BRAGI_CONFIG_PATH")

# Mirrors the hosted API's `wants_markdown()`: these `?format=` values return the
# canonical bgraph.md text in `bgraph_md`; anything else (incl. `json`) returns
# the graph JSON in `graph`.
_MARKDOWN_FORMATS = {"md", "bgraph-md", "bgraph"}


def _error(status_code: int, code: str, message: str) -> JSONResponse:
    """Official error envelope: `{"success": false, "error": {code, message}}`."""
    return JSONResponse(
        status_code=status_code,
        content={"success": False, "error": {"code": code, "message": message}},
    )


@app.get("/health")
async def health() -> dict:
    return {"name": "Bragi API — Self-Hosted", "status": "healthy", "version": app.version}


@app.post("/v1/parse/pdf")
async def parse_pdf(
    request: Request,
    format: str | None = Query(None, description="json (default) | md | bgraph-md | bgraph"),
) -> JSONResponse:
    # Raw request body — matches the hosted API (`pdf_data: Bytes`), not multipart.
    pdf_bytes = await request.body()
    if not pdf_bytes:
        return _error(400, "bad_request", "No PDF data provided")

    want_md = (format or "json") in _MARKDOWN_FORMATS

    tmp_dir = None
    try:
        tmp_dir = tempfile.mkdtemp(prefix="bragi_")
        input_path = Path(tmp_dir) / "input.pdf"
        input_path.write_bytes(pdf_bytes)

        if want_md:
            out_format, output_path = "bgraph-md", Path(tmp_dir) / "output.bgraph.md"
        else:
            out_format, output_path = "bgraph", Path(tmp_dir) / "output.json"

        cmd = [
            CLI_PATH,
            "parse",
            "-i", str(input_path),
            "--jar-path", JAR_PATH,
            "-f", out_format,
            "-o", str(output_path),
        ]
        if DEFAULT_CONFIG_PATH:
            cmd.extend(["--config", DEFAULT_CONFIG_PATH])

        result = subprocess.run(cmd, capture_output=True, text=True, timeout=300)

        if result.returncode != 0:
            return _error(
                500,
                "processing_error",
                f"CLI processing failed: {result.stderr.strip() or result.stdout.strip()}",
            )

        if not output_path.exists():
            return _error(500, "processing_error", "CLI completed but no output file was produced")

        if want_md:
            return JSONResponse(content={"success": True, "bgraph_md": output_path.read_text()})

        graph = json.loads(output_path.read_text())
        return JSONResponse(content={"success": True, "graph": graph})

    except subprocess.TimeoutExpired:
        return _error(500, "processing_error", "Processing timed out after 300 seconds")
    except Exception as exc:  # noqa: BLE001 — surface any unexpected failure as a 500
        return _error(500, "processing_error", f"Unexpected error: {exc}")
    finally:
        if tmp_dir:
            import shutil

            shutil.rmtree(tmp_dir, ignore_errors=True)
