"""HTTP client for the Bragi API (sync + async)."""

from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

import httpx

from bragi.errors import (
    BragiAuthError,
    BragiCreditsError,
    BragiError,
    BragiProcessingError,
)
from bragi.types import Bragi

if TYPE_CHECKING:
    from bragi._config import _Config

_PARSE_PATH = "/v1/parse/pdf"
_TIMEOUT = 300.0  # 5 minutes — PDF processing can be slow


def _build_headers(cfg: "_Config") -> dict[str, str]:
    """Build request headers. Authorization is omitted if no API key is set."""
    if cfg.api_key:
        return {"Authorization": f"Bearer {cfg.api_key}"}
    return {}


def _handle_response(response: httpx.Response) -> Bragi:
    """Parse an API response into a Bragi, raising on errors."""
    if response.status_code == 401:
        body = response.json()
        msg = body.get("error", {}).get("message", "Unauthorized")
        raise BragiAuthError(msg)

    if response.status_code == 402:
        body = response.json()
        msg = body.get("error", {}).get("message", "Insufficient credits")
        raise BragiCreditsError(msg)

    if response.status_code >= 500:
        try:
            body = response.json()
            msg = body.get("error", {}).get("message", "Processing error")
        except Exception:
            msg = f"Server error (HTTP {response.status_code})"
        raise BragiProcessingError(msg)

    if response.status_code != 200:
        raise BragiError(f"Unexpected HTTP {response.status_code}: {response.text}")

    body = response.json()
    if not body.get("success"):
        error_info = body.get("error", {})
        msg = error_info.get("message", "Unknown error") if isinstance(error_info, dict) else str(error_info)
        raise BragiProcessingError(msg)

    return Bragi.from_dict(body["graph"])


def _sync_parse_pdf(path: str, cfg: "_Config") -> Bragi:
    """Send a PDF to the API using httpx sync client."""
    pdf_path = Path(path)
    if not pdf_path.exists():
        raise FileNotFoundError(f"PDF not found: {path}")

    url = cfg.resolved_url.rstrip("/") + _PARSE_PATH
    headers = _build_headers(cfg)

    with httpx.Client(timeout=_TIMEOUT) as client:
        with open(pdf_path, "rb") as f:
            files = {"file": (pdf_path.name, f, "application/pdf")}
            response = client.post(url, headers=headers, files=files)

    return _handle_response(response)


async def _async_parse_pdf(path: str, cfg: "_Config") -> Bragi:
    """Send a PDF to the API using httpx async client."""
    pdf_path = Path(path)
    if not pdf_path.exists():
        raise FileNotFoundError(f"PDF not found: {path}")

    url = cfg.resolved_url.rstrip("/") + _PARSE_PATH
    headers = _build_headers(cfg)

    async with httpx.AsyncClient(timeout=_TIMEOUT) as client:
        with open(pdf_path, "rb") as f:
            files = {"file": (pdf_path.name, f, "application/pdf")}
            response = await client.post(url, headers=headers, files=files)

    return _handle_response(response)
