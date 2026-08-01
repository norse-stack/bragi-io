"""Bragi Python SDK — parse PDFs into typed semantic document graphs.

Usage::

    import bragi as bg

    # Local mode (no account needed)
    graph = bg.parse_pdf("document.pdf")

    # API mode
    bg.configure(api_key="blaze_prod_XXX...")
    graph = bg.parse_pdf("document.pdf")

    # Async mode (self-hosted or API)
    graph = await bg.parse_pdf_async("document.pdf")
"""

from __future__ import annotations

from bragi._config import configure, get_config
from bragi.errors import (
    BragiAuthError,
    BragiCreditsError,
    BragiError,
    BragiNotFoundError,
    BragiProcessingError,
)
from bragi.types import (
    Bragi,
    BookmarkData,
    BookmarkSection,
    BoundingBox,
    DepthDistribution,
    DocumentInfo,
    DocumentMetadata,
    DocumentNode,
    DocxMetadata,
    ExternalRef,
    ExternalRefTarget,
    HistogramBin,
    InternalRef,
    InternalRefTarget,
    MdMetadata,
    NodeContent,
    NodeLocation,
    NodeTypeDistribution,
    ParseProvenance,
    PdfMetadata,
    PhysicalLocation,
    SemanticLocation,
    StructuralProfile,
    StyleMetadata,
    TargetPoint,
    TokenDistribution,
    TokenHistogram,
)

__all__ = [
    # Public API functions
    "configure",
    "parse_pdf",
    "parse_pdf_async",
    # Top-level type
    "Bragi",
    "ParseProvenance",
    # Node types
    "DocumentNode",
    "NodeLocation",
    "SemanticLocation",
    "PhysicalLocation",
    "BoundingBox",
    "NodeContent",
    "StyleMetadata",
    # Reference types
    "InternalRef",
    "InternalRefTarget",
    "ExternalRef",
    "ExternalRefTarget",
    "TargetPoint",
    # Document info types
    "DocumentInfo",
    "DocumentMetadata",
    "PdfMetadata",
    "MdMetadata",
    "DocxMetadata",
    "BookmarkData",
    "BookmarkSection",
    # Structural profile types
    "StructuralProfile",
    "TokenDistribution",
    "TokenHistogram",
    "HistogramBin",
    "NodeTypeDistribution",
    "DepthDistribution",
    # Errors
    "BragiError",
    "BragiAuthError",
    "BragiCreditsError",
    "BragiProcessingError",
    "BragiNotFoundError",
]

from importlib.metadata import PackageNotFoundError, version as _pkg_version

try:
    # Single source of truth: the installed package metadata (pyproject
    # [project] version, bundle-bumped via `make bump-version`). Reading it
    # here means __version__ can never drift from the manifest.
    __version__ = _pkg_version("bragi-io")
except PackageNotFoundError:  # source checkout without a pip install
    __version__ = "0.0.0+unknown"


def parse_pdf(
    path: str,
    *,
    config: str | None = None,
) -> Bragi:
    """Parse a PDF and return a typed document graph.

    Uses the configured mode:
    - If ``api_key`` is set via :func:`configure`: sends the PDF to the API.
    - Otherwise: runs the ``bragi`` binary locally.

    Args:
        path: Path to a PDF file.
        config: Path to a config YAML file (local mode only).

    Returns:
        A :class:`Bragi` with fully typed nodes.

    Raises:
        BragiAuthError: If the API key is invalid (API mode).
        BragiCreditsError: If credits are exhausted (API mode).
        BragiProcessingError: If processing fails.
        BragiNotFoundError: If the CLI binary is not found (local mode).
    """
    cfg = get_config()
    if cfg.is_http_mode:
        from bragi.client import _sync_parse_pdf

        return _sync_parse_pdf(path, cfg)
    else:
        from bragi.local import _local_parse_pdf

        return _local_parse_pdf(path, config_path=config)


async def parse_pdf_async(
    path: str,
) -> Bragi:
    """Parse a PDF asynchronously via the API.

    Requires an API key to be configured via :func:`configure`.

    Args:
        path: Path to a PDF file.

    Returns:
        A :class:`Bragi` with fully typed nodes.

    Raises:
        BragiAuthError: If the API key is invalid.
        BragiCreditsError: If credits are exhausted.
        BragiProcessingError: If processing fails.
        BragiError: If no API key is configured.
    """
    cfg = get_config()
    if not cfg.is_http_mode:
        raise BragiError(
            "parse_pdf_async requires a host or API key. Call bg.configure(host=...) or bg.configure(api_key=...) first."
        )
    from bragi.client import _async_parse_pdf

    return await _async_parse_pdf(path, cfg)
