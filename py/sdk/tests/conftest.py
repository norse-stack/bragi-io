"""Shared test fixtures for bragi tests."""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from bragi.types import BragiGraph

_FIXTURES_DIR = Path(__file__).parent / "fixtures"

# The blessed 1.0.0 graph.json, regenerated from the golden `graphrag.pdf`
# via `make sync-python-fixture` (mirrors the core golden family). The single
# source of truth for the SDK's type tests — if the wire shape moves, this
# fixture (and these tests) move with it.
_GRAPH_FIXTURE = _FIXTURES_DIR / "graphrag_graph.json"


@pytest.fixture
def graphrag_raw() -> dict:
    """Load the raw graphrag_graph.json as a dict."""
    return json.loads(_GRAPH_FIXTURE.read_text(encoding="utf-8"))


@pytest.fixture
def graphrag_graph(graphrag_raw: dict) -> BragiGraph:
    """Load the graphrag graph as a fully typed BragiGraph."""
    return BragiGraph.from_dict(graphrag_raw)
