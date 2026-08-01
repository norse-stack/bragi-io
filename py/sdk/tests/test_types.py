"""Deserialization of the 1.0.0 graph.json fixtures into typed BlazeGraph.

Ground truth: `attention_graph.json` (PDF, Fixed flow) + `demo_md_graph.json`
(markdown, Free flow), both regenerated from the core golden family via
`make sync-python-fixture`.
"""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from bragi.types import (
    BlazeGraph,
    BookmarkData,
    BoundingBox,
    DocumentInfo,
    DocumentNode,
    ExternalRef,
    InternalRef,
    NodeContent,
    NodeLocation,
    ParseProvenance,
    PhysicalLocation,
    SemanticLocation,
    StructuralProfile,
    StyleMetadata,
)

_FIXTURES_DIR = Path(__file__).parent / "fixtures"


@pytest.fixture
def demo_md_graph() -> BlazeGraph:
    """The Free-flow markdown fixture (regression-guards flow_type)."""
    raw = json.loads((_FIXTURES_DIR / "demo_md_graph.json").read_text(encoding="utf-8"))
    return BlazeGraph.from_dict(raw)


class TestBlazeGraphDeserialization:
    """Deserialize the real attention fixture and verify all typed fields."""

    def test_top_level_fields(self, attention_graph: BlazeGraph) -> None:
        assert attention_graph.schema_version == "1.0.0"
        assert len(attention_graph.nodes) > 0
        assert isinstance(attention_graph.document_info, DocumentInfo)
        assert isinstance(attention_graph.structural_profile, StructuralProfile)

    def test_wrapper_identity_fields(self, attention_graph: BlazeGraph) -> None:
        # 1.0.0 wrapper carries graph_sha256 (64 hex) + created_at.
        assert len(attention_graph.graph_sha256) == 64
        assert attention_graph.created_at  # non-empty ISO timestamp

    def test_parse_provenance(self, attention_graph: BlazeGraph) -> None:
        prov = attention_graph.parse_provenance
        assert isinstance(prov, ParseProvenance)
        assert prov.source_format == "pdf"
        assert len(prov.source_sha256) == 64
        # CR-92: provenance is content-only — no source_filename attribute.
        assert not hasattr(prov, "source_filename")

    def test_repr(self, attention_graph: BlazeGraph) -> None:
        r = repr(attention_graph)
        assert "BlazeGraph" in r
        assert "nodes" in r
        assert "v1.0.0" in r

    def test_node_count(self, attention_graph: BlazeGraph) -> None:
        # attention.pdf golden fixture has 179 nodes.
        assert len(attention_graph.nodes) == 179

    def test_root_node(self, attention_graph: BlazeGraph) -> None:
        root = attention_graph.root
        assert root.node_type == "Document"
        assert root.parent is None
        assert len(root.children) > 0

    def test_root_id_matches_document_info(self, attention_graph: BlazeGraph) -> None:
        root = attention_graph.root
        assert root.id == attention_graph.document_info.root_id

    def test_document_node_fields(self, attention_graph: BlazeGraph) -> None:
        node = attention_graph.nodes[0]
        assert isinstance(node, DocumentNode)
        assert isinstance(node.id, str)
        assert isinstance(node.node_type, str)
        assert isinstance(node.location, NodeLocation)
        assert isinstance(node.content, NodeContent)
        assert isinstance(node.token_count, int)
        assert isinstance(node.children, list)

    def test_semantic_location(self, attention_graph: BlazeGraph) -> None:
        node = attention_graph.nodes[0]
        sem = node.location.semantic
        assert isinstance(sem, SemanticLocation)
        assert isinstance(sem.path, str)
        assert isinstance(sem.depth, int)
        assert isinstance(sem.breadcrumbs, list)
        assert all(isinstance(b, str) for b in sem.breadcrumbs)

    def test_physical_location_present(self, attention_graph: BlazeGraph) -> None:
        """At least some nodes should have physical locations (it's a PDF)."""
        nodes_with_phys = [
            n for n in attention_graph.nodes if n.location.physical is not None
        ]
        assert len(nodes_with_phys) > 0

        phys = nodes_with_phys[0].location.physical
        assert isinstance(phys, PhysicalLocation)
        assert isinstance(phys.page, int)
        assert phys.page >= 1
        assert isinstance(phys.bounding_box, BoundingBox)
        assert isinstance(phys.bounding_box.x, float)
        assert isinstance(phys.bounding_box.y, float)
        assert isinstance(phys.bounding_box.width, float)
        assert isinstance(phys.bounding_box.height, float)

    def test_physical_location_null_for_root(self, attention_graph: BlazeGraph) -> None:
        root = attention_graph.root
        assert root.location.physical is None

    def test_style_info(self, attention_graph: BlazeGraph) -> None:
        """PDF nodes carry style_info (CR-86); the root does not."""
        assert attention_graph.root.style_info is None
        styled = [n for n in attention_graph.nodes if n.style_info is not None]
        assert len(styled) > 0
        style = styled[0].style_info
        assert isinstance(style, StyleMetadata)
        assert isinstance(style.font_class, str)
        assert style.font_family is not None

    def test_internal_refs(self, attention_graph: BlazeGraph) -> None:
        """attention.pdf has intra-document citation links (CR-62)."""
        ref_nodes = [n for n in attention_graph.nodes if n.internal_refs]
        assert len(ref_nodes) > 0
        ref = ref_nodes[0].internal_refs[0]
        assert isinstance(ref, InternalRef)
        assert ref.target.kind in ("named", "page")

    def test_external_refs(self, attention_graph: BlazeGraph) -> None:
        ref_nodes = [n for n in attention_graph.nodes if n.external_refs]
        assert len(ref_nodes) > 0
        ref = ref_nodes[0].external_refs[0]
        assert isinstance(ref, ExternalRef)
        assert ref.target.kind == "uri"

    def test_refs_default_empty(self, attention_graph: BlazeGraph) -> None:
        # Nodes without refs get empty lists, never None.
        for n in attention_graph.nodes:
            assert isinstance(n.internal_refs, list)
            assert isinstance(n.external_refs, list)

    def test_sections_filter(self, attention_graph: BlazeGraph) -> None:
        sections = attention_graph.sections
        assert len(sections) > 0
        assert all(s.node_type == "Section" for s in sections)

    def test_paragraphs_filter(self, attention_graph: BlazeGraph) -> None:
        paragraphs = attention_graph.paragraphs
        assert len(paragraphs) > 0
        assert all(p.node_type == "Paragraph" for p in paragraphs)

    def test_get_node(self, attention_graph: BlazeGraph) -> None:
        first = attention_graph.nodes[0]
        fetched = attention_graph.get_node(first.id)
        assert fetched is first

    def test_get_node_missing(self, attention_graph: BlazeGraph) -> None:
        with pytest.raises(KeyError):
            attention_graph.get_node("nonexistent-id")

    def test_nodes_by_page(self, attention_graph: BlazeGraph) -> None:
        page1 = attention_graph.nodes_by_page(1)
        assert len(page1) > 0
        for n in page1:
            assert n.location.physical is not None
            assert n.location.physical.page == 1

    def test_tree_navigation_parent(self, attention_graph: BlazeGraph) -> None:
        non_root = [n for n in attention_graph.nodes if n.parent is not None]
        assert len(non_root) > 0
        node = non_root[0]
        parent = node.get_parent(attention_graph)
        assert parent is not None
        assert parent.id == node.parent

    def test_tree_navigation_children(self, attention_graph: BlazeGraph) -> None:
        root = attention_graph.root
        children = root.get_children(attention_graph)
        assert len(children) == len(root.children)
        for child, child_id in zip(children, root.children):
            assert child.id == child_id

    def test_root_parent_is_none(self, attention_graph: BlazeGraph) -> None:
        root = attention_graph.root
        assert root.get_parent(attention_graph) is None

    # -- DocumentInfo / metadata (CR-57 namespaces) --

    def test_document_kind(self, attention_graph: BlazeGraph) -> None:
        assert attention_graph.document_info.kind == "document"

    def test_document_metadata_pdf_namespace(self, attention_graph: BlazeGraph) -> None:
        """page_count moved into the pdf namespace (CR-57)."""
        meta = attention_graph.document_info.document_metadata
        assert meta.title == "Attention Is All You Need"
        assert meta.pdf is not None
        assert isinstance(meta.pdf.page_count, int)
        assert meta.pdf.page_count > 0
        assert meta.md is None and meta.docx is None
        assert isinstance(meta.pdf.extras, dict)

    def test_outline_data(self, attention_graph: BlazeGraph) -> None:
        outline = attention_graph.document_info.outline_data
        assert isinstance(outline, BookmarkData)
        assert len(outline.sections) > 0
        first = outline.sections[0]
        assert isinstance(first.title, str)
        assert isinstance(first.order, int)
        assert first.level >= 1

    # -- flow_type: THE fix (moved to document_info; was silently "Fixed") --

    def test_flow_type_on_document_info(self, attention_graph: BlazeGraph) -> None:
        assert attention_graph.document_info.flow_type == "Fixed"
        assert attention_graph.flow_type == "Fixed"  # convenience accessor
        # It must NOT live on structural_profile anymore.
        assert not hasattr(attention_graph.structural_profile, "flow_type")

    def test_flow_type_free_for_markdown(self, demo_md_graph: BlazeGraph) -> None:
        """Regression guard: a Free doc must report Free, not the old silent Fixed."""
        assert demo_md_graph.flow_type == "Free"
        assert demo_md_graph.document_info.flow_type == "Free"

    def test_markdown_has_no_physical_locations(self, demo_md_graph: BlazeGraph) -> None:
        assert all(n.location.physical is None for n in demo_md_graph.nodes)

    def test_markdown_provenance(self, demo_md_graph: BlazeGraph) -> None:
        assert demo_md_graph.parse_provenance.source_format == "markdown"

    # -- StructuralProfile --

    def test_structural_profile(self, attention_graph: BlazeGraph) -> None:
        sp = attention_graph.structural_profile
        assert sp.total_nodes > 0
        assert sp.total_tokens > 0
        assert isinstance(sp.document_type, str)

    def test_structural_profile_distributions(self, attention_graph: BlazeGraph) -> None:
        sp = attention_graph.structural_profile
        assert sp.node_type_distribution is not None
        assert "Paragraph" in sp.node_type_distribution.counts
        assert sp.depth_distribution is not None
        assert sp.depth_distribution.max_depth > 0

    # -- Serialization --

    def test_to_dict_roundtrip(
        self, attention_graph: BlazeGraph, attention_raw: dict
    ) -> None:
        assert attention_graph.to_dict() is attention_raw

    def test_to_json(self, attention_graph: BlazeGraph) -> None:
        j = attention_graph.to_json()
        parsed = json.loads(j)
        assert parsed["schema_version"] == "1.0.0"
        assert len(parsed["nodes"]) == 179


class TestNodeContent:
    """Verify node content text is accessible."""

    def test_content_text_accessible(self, attention_graph: BlazeGraph) -> None:
        for node in attention_graph.nodes[:5]:
            assert isinstance(node.content.text, str)

    def test_section_has_meaningful_text(self, attention_graph: BlazeGraph) -> None:
        sections = attention_graph.sections
        assert any("Attention" in s.content.text for s in sections)
