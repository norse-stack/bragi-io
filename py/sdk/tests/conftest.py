"""Shared test fixtures for bragi tests."""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from bragi.types import BlazeGraph

_FIXTURES_DIR = Path(__file__).parent / "fixtures"

# The blessed 1.0.0 graph.json, regenerated from the golden `attention.pdf`
# via `make sync-python-fixture` (mirrors the core golden family). The single
# source of truth for the SDK's type tests — if the wire shape moves, this
# fixture (and these tests) move with it.
_GRAPH_FIXTURE = _FIXTURES_DIR / "attention_graph.json"


@pytest.fixture
def attention_raw() -> dict:
    """Load the raw attention_graph.json as a dict."""
    return json.loads(_GRAPH_FIXTURE.read_text(encoding="utf-8"))


@pytest.fixture
def attention_graph(attention_raw: dict) -> BlazeGraph:
    """Load the attention graph as a fully typed BlazeGraph."""
    return BlazeGraph.from_dict(attention_raw)
