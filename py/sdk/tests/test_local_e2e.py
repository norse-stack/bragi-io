"""End-to-end local-mode test: drive the REAL CLI binary + Tika/JVM.

This is the test that validates the SDK's core value proposition — local PDF
parsing via the auto-located ``bragi`` binary and the Tika JVM bridge.
Unlike ``test_local.py`` (which mocks ``subprocess``), this runs the actual
pipeline against the golden ``attention.pdf`` and deserializes the real output.

Pre-B7 (no published GitHub release), it points ``BRAGI_CLI_PATH`` at the
locally-built ``target/release/bragi`` (search-order step 1 in
``_download.py``) and ``JAVA_HOME`` at an available JRE — no download needed.
Skips cleanly when those aren't present (e.g. CI without a prior `make build-cli`).
"""

from __future__ import annotations

import os
import shutil
from pathlib import Path

import pytest

from bragi.local import _local_parse_pdf
from bragi.types import BlazeGraph

_HERE = Path(__file__).resolve().parent
_SUBMODULE_ROOT = _HERE.parent.parent  # …/bragi-io/
_CLI_BIN = _SUBMODULE_ROOT / "target" / "release" / "bragi"
_TIKA_JAR = (
    _SUBMODULE_ROOT / "crates" / "core" / "deps" / "tika" / "jni-jars" / "blazing-tika-jni.jar"
)
_ATTENTION_PDF = (
    _SUBMODULE_ROOT
    / "crates"
    / "core"
    / "test_fixtures"
    / "golden"
    / "1.0.0"
    / "attention"
    / "attention.pdf"
)


def _resolve_jre() -> str | None:
    """Locate a JRE directory (JAVA_HOME shape) for the ``--jre-path`` flag."""
    jh = os.environ.get("JAVA_HOME")
    if jh and Path(jh).is_dir():
        return jh
    sdkman = Path.home() / ".sdkman" / "candidates" / "java" / "current"
    if sdkman.is_dir():
        return str(sdkman)
    java = shutil.which("java")
    if java:
        # <jre>/bin/java → <jre>
        return str(Path(java).resolve().parent.parent)
    return None


_JRE = _resolve_jre()

pytestmark = pytest.mark.skipif(
    not (_CLI_BIN.exists() and _TIKA_JAR.exists() and _ATTENTION_PDF.exists() and _JRE),
    reason="e2e needs a locally-built CLI + Tika JAR + a JRE (run `make build-cli`)",
)


def test_local_parse_pdf_end_to_end(monkeypatch, tmp_path) -> None:
    """A real Tika-backed parse of attention.pdf through the SDK's local path."""
    monkeypatch.setenv("BRAGI_CLI_PATH", str(_CLI_BIN))
    monkeypatch.setenv("JAVA_HOME", _JRE)
    monkeypatch.setenv("PREPROCESSOR_JRE_PATH", _JRE)
    monkeypatch.setenv("PREPROCESSOR_JAR_PATH", str(_TIKA_JAR))
    # Isolate the CLI's cache so the run is fresh (Tika actually executes) and
    # leaves no artifacts in the working tree.
    monkeypatch.setenv("BRAGI_CACHE_DIR", str(tmp_path / "cache"))

    graph = _local_parse_pdf(str(_ATTENTION_PDF))

    assert isinstance(graph, BlazeGraph)
    assert graph.schema_version == "1.0.0"
    assert len(graph.nodes) > 0
    # PDF-via-Tika invariants (not byte-pinned to the committed fixture, since a
    # fresh Tika parse may differ from the C2-cached golden bytes).
    assert graph.flow_type == "Fixed"
    assert any(n.location.physical is not None for n in graph.nodes)
    assert graph.document_info.document_metadata.pdf is not None
    assert graph.document_info.document_metadata.pdf.page_count
    assert graph.parse_provenance is not None
    assert graph.parse_provenance.source_format == "pdf"
    # CR-92: the emitted graph is content-only — no filename anywhere.
    assert "source_filename" not in graph.to_json()
