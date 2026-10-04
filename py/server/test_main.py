"""Tests for the self-hosted parse server's request handling.

They need no parser binary: refused requests never reach it, and the
accepted case runs against a stand-in executable that records what it was
given.

    make test-server
"""

from __future__ import annotations

import hashlib
import json
import stat
import sys
from pathlib import Path

import pytest
from fastapi.testclient import TestClient

import main

PDF = b"%PDF-1.7\n1 0 obj\n<<>>\nendobj\n%%EOF\n"

client = TestClient(main.app)


def test_a_multipart_upload_is_refused_with_400() -> None:
    # What `curl -F file=@doc.pdf` sends: the PDF wrapped in a form envelope.
    response = client.post("/v1/parse/pdf", files={"file": ("doc.pdf", PDF, "application/pdf")})
    assert response.status_code == 400
    assert response.json() == {
        "success": False,
        "error": {
            "code": "bad_request",
            "message": (
                "Request body must be a raw PDF (send the bytes with Content-Type: "
                "application/pdf, not multipart/form-data)"
            ),
        },
    }


def test_a_body_that_is_not_a_pdf_is_refused_with_400() -> None:
    response = client.post("/v1/parse/pdf", content=b"hello, world")
    assert response.status_code == 400
    assert response.json()["error"]["code"] == "bad_request"


def test_an_empty_body_is_refused_with_400() -> None:
    response = client.post("/v1/parse/pdf", content=b"")
    assert response.status_code == 400
    assert response.json() == {
        "success": False,
        "error": {"code": "bad_request", "message": "No PDF data provided"},
    }


@pytest.fixture
def stand_in_cli(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> Path:
    """An executable that answers `parse -i IN ... -o OUT` by writing the
    sha256 of IN's bytes to OUT as JSON."""
    script = tmp_path / "bragi"
    script.write_text(
        f"#!{sys.executable}\n"
        "import hashlib, json, sys\n"
        "args = sys.argv[1:]\n"
        "src = args[args.index('-i') + 1]\n"
        "out = args[args.index('-o') + 1]\n"
        "with open(src, 'rb') as f:\n"
        "    digest = hashlib.sha256(f.read()).hexdigest()\n"
        "with open(out, 'w') as f:\n"
        "    json.dump({'input_sha256': digest}, f)\n"
    )
    script.chmod(script.stat().st_mode | stat.S_IXUSR)
    monkeypatch.setattr(main, "CLI_PATH", str(script))
    return script


def test_a_raw_pdf_body_reaches_the_parser_byte_for_byte(stand_in_cli: Path) -> None:
    response = client.post(
        "/v1/parse/pdf", content=PDF, headers={"Content-Type": "application/pdf"}
    )
    assert response.status_code == 200, response.text
    body = response.json()
    assert body["success"] is True
    assert body["graph"] == {"input_sha256": hashlib.sha256(PDF).hexdigest()}
    json.dumps(body)  # the envelope is plain JSON
