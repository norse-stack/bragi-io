"""Typed data model for Blazegraph document graphs.

All dataclasses mirror the Rust types in ``blazegraph-core/src/types.rs``
(schema version 1.0.0). Designed for full IDE autocomplete.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional


# ---------------------------------------------------------------------------
# Leaf types
# ---------------------------------------------------------------------------


@dataclass
class BoundingBox:
    """Physical bounding box on a PDF page (PDF points, origin top-left)."""

    x: float
    y: float
    width: float
    height: float

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "BoundingBox":
        return cls(x=d["x"], y=d["y"], width=d["width"], height=d["height"])


@dataclass
class SemanticLocation:
    """Where a node sits in the document tree."""

    path: str
    depth: int
    breadcrumbs: List[str]

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "SemanticLocation":
        return cls(path=d["path"], depth=d["depth"], breadcrumbs=list(d["breadcrumbs"]))


@dataclass
class PhysicalLocation:
    """Where a node appears on the physical page."""

    page: int
    bounding_box: BoundingBox

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "PhysicalLocation":
        return cls(
            page=d["page"],
            bounding_box=BoundingBox.from_dict(d["bounding_box"]),
        )


@dataclass
class NodeLocation:
    """Combined semantic + physical location."""

    semantic: SemanticLocation
    physical: Optional[PhysicalLocation]

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "NodeLocation":
        phys = d.get("physical")
        return cls(
            semantic=SemanticLocation.from_dict(d["semantic"]),
            physical=PhysicalLocation.from_dict(phys) if phys else None,
        )


@dataclass
class NodeContent:
    """Node text content."""

    text: str

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "NodeContent":
        return cls(text=d["text"])


@dataclass
class StyleMetadata:
    """Verbatim style projection for a node (CR-45 / DT-03).

    Mirrors Rust ``StyleMetadata``. Present on every node key (``None`` when
    style was not requested/available — the dominant case for the Document
    root). ``font_class`` is always present; the rest are best-effort.
    """

    font_class: str
    font_size: Optional[float] = None
    is_bold: bool = False
    is_italic: bool = False
    font_family: Optional[str] = None
    foreground_color: Optional[str] = None
    background_color: Optional[str] = None

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "StyleMetadata":
        return cls(
            font_class=d["font_class"],
            font_size=d.get("font_size"),
            is_bold=d.get("is_bold", False),
            is_italic=d.get("is_italic", False),
            font_family=d.get("font_family"),
            foreground_color=d.get("foreground_color"),
            background_color=d.get("background_color"),
        )


@dataclass
class TargetPoint:
    """Resolved destination point on a target page (top-origin coords, CR-62)."""

    x: Optional[float] = None
    y: Optional[float] = None

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "TargetPoint":
        return cls(x=d.get("x"), y=d.get("y"))


@dataclass
class InternalRefTarget:
    """Target of an intra-document reference (CR-62).

    Flattens the Rust ``InternalRefTarget`` tagged union: ``kind`` is
    ``"named"`` (``name`` populated) or ``"page"``. ``page`` / ``point`` are
    the resolved destination when Tika's name-tree walk produced one.
    """

    kind: str
    name: Optional[str] = None
    page: Optional[int] = None
    point: Optional[TargetPoint] = None

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "InternalRefTarget":
        pt = d.get("point")
        return cls(
            kind=d["kind"],
            name=d.get("name"),
            page=d.get("page"),
            point=TargetPoint.from_dict(pt) if pt else None,
        )


@dataclass
class ExternalRefTarget:
    """Target of an out-of-document reference (CR-62). ``kind`` is ``"uri"``."""

    kind: str
    url: Optional[str] = None

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "ExternalRefTarget":
        return cls(kind=d["kind"], url=d.get("url"))


@dataclass
class InternalRef:
    """A reference from a node to a location within the same document (CR-62)."""

    text: str
    target: InternalRefTarget
    source_page: Optional[int] = None
    source_bbox: Optional[BoundingBox] = None

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "InternalRef":
        bbox = d.get("source_bbox")
        return cls(
            text=d["text"],
            target=InternalRefTarget.from_dict(d["target"]),
            source_page=d.get("source_page"),
            source_bbox=BoundingBox.from_dict(bbox) if bbox else None,
        )


@dataclass
class ExternalRef:
    """A reference from a node to a location outside the document (CR-62)."""

    text: str
    target: ExternalRefTarget
    source_page: Optional[int] = None
    source_bbox: Optional[BoundingBox] = None

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "ExternalRef":
        bbox = d.get("source_bbox")
        return cls(
            text=d["text"],
            target=ExternalRefTarget.from_dict(d["target"]),
            source_page=d.get("source_page"),
            source_bbox=BoundingBox.from_dict(bbox) if bbox else None,
        )


# ---------------------------------------------------------------------------
# DocumentNode
# ---------------------------------------------------------------------------


@dataclass
class DocumentNode:
    """A single node in the document graph."""

    id: str
    node_type: str
    location: NodeLocation
    text_order: Optional[int]
    content: NodeContent
    token_count: int
    parent: Optional[str]
    children: List[str]
    style_info: Optional[StyleMetadata] = None
    internal_refs: List[InternalRef] = field(default_factory=list)
    external_refs: List[ExternalRef] = field(default_factory=list)

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "DocumentNode":
        style = d.get("style_info")
        return cls(
            id=d["id"],
            node_type=d["node_type"],
            location=NodeLocation.from_dict(d["location"]),
            text_order=d.get("text_order"),
            content=NodeContent.from_dict(d["content"]),
            token_count=d["token_count"],
            parent=d.get("parent"),
            children=list(d.get("children", [])),
            style_info=StyleMetadata.from_dict(style) if style else None,
            internal_refs=[
                InternalRef.from_dict(r) for r in (d.get("internal_refs") or [])
            ],
            external_refs=[
                ExternalRef.from_dict(r) for r in (d.get("external_refs") or [])
            ],
        )

    # -- Tree navigation helpers --

    def get_parent(self, graph: "BlazeGraph") -> Optional["DocumentNode"]:
        """Return the parent node, or ``None`` for the root."""
        if self.parent is None:
            return None
        return graph.get_node(self.parent)

    def get_children(self, graph: "BlazeGraph") -> List["DocumentNode"]:
        """Return resolved child nodes in text order."""
        return [graph.get_node(cid) for cid in self.children]

    # -- Render --

    def render(
        self,
        graph: "BlazeGraph",
        *,
        breadcrumbs: bool = False,
        node_types: bool = False,
    ) -> str:
        """Render this node and all descendants as human-readable text.

        Args:
            graph: The parent ``BlazeGraph`` (needed for child lookup).
            breadcrumbs: Show breadcrumb trail on section/document nodes.
            node_types: Show ``[Type]`` prefix on each node.
        """
        parts: List[str] = []
        self._render_into(parts, graph, breadcrumbs=breadcrumbs, node_types=node_types)
        return "\n\n".join(parts)

    def _render_into(
        self,
        parts: List[str],
        graph: "BlazeGraph",
        *,
        breadcrumbs: bool,
        node_types: bool,
    ) -> None:
        """Recursively collect rendered text fragments."""
        header = self._render_header(breadcrumbs=breadcrumbs, node_types=node_types)
        if header:
            parts.append(header)

        for child in self.get_children(graph):
            child._render_into(parts, graph, breadcrumbs=breadcrumbs, node_types=node_types)

    def _render_header(self, *, breadcrumbs: bool, node_types: bool) -> str:
        """Build the header string for this single node."""
        is_structural = self.node_type in ("Section", "Document")
        use_breadcrumbs = breadcrumbs and is_structural

        if use_breadcrumbs and node_types:
            # [Section | crumb > crumb]
            trail = " > ".join(self.location.semantic.breadcrumbs)
            return f"[{self.node_type} | {trail}]"
        elif use_breadcrumbs:
            # [crumb > crumb]
            trail = " > ".join(self.location.semantic.breadcrumbs)
            return f"[{trail}]"
        elif node_types:
            # [Section] text
            return f"[{self.node_type}] {self.content.text}"
        else:
            # plain text
            return self.content.text


# ---------------------------------------------------------------------------
# DocumentInfo sub-types
# ---------------------------------------------------------------------------


@dataclass
class PdfMetadata:
    """PDF-channel metadata namespace (CR-57). Populated for PDF sources."""

    version: Optional[str] = None
    producer: Optional[str] = None
    creator_tool: Optional[str] = None
    publisher: Optional[str] = None
    page_count: Optional[int] = None
    encrypted: Optional[bool] = None
    has_marked_content: Optional[bool] = None
    modified: Optional[str] = None
    extras: Dict[str, Any] = field(default_factory=dict)

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "PdfMetadata":
        return cls(
            version=d.get("version"),
            producer=d.get("producer"),
            creator_tool=d.get("creator_tool"),
            publisher=d.get("publisher"),
            page_count=d.get("page_count"),
            encrypted=d.get("encrypted"),
            has_marked_content=d.get("has_marked_content"),
            modified=d.get("modified"),
            extras=dict(d.get("extras", {})),
        )


@dataclass
class MdMetadata:
    """Markdown-channel metadata namespace (CR-57): frontmatter slots."""

    draft: Optional[bool] = None
    tags: List[str] = field(default_factory=list)
    categories: List[str] = field(default_factory=list)
    extras: Dict[str, Any] = field(default_factory=dict)

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "MdMetadata":
        return cls(
            draft=d.get("draft"),
            tags=list(d.get("tags", [])),
            categories=list(d.get("categories", [])),
            extras=dict(d.get("extras", {})),
        )


@dataclass
class DocxMetadata:
    """DOCX-channel metadata namespace (CR-57): core.xml + app.xml properties."""

    application: Optional[str] = None
    app_version: Optional[str] = None
    pages: Optional[int] = None
    words: Optional[int] = None
    characters: Optional[int] = None
    lines: Optional[int] = None
    paragraphs: Optional[int] = None
    company: Optional[str] = None
    manager: Optional[str] = None
    template: Optional[str] = None
    total_time: Optional[int] = None
    doc_security: Optional[int] = None
    last_modified_by: Optional[str] = None
    revision: Optional[str] = None
    modified: Optional[str] = None
    extras: Dict[str, Any] = field(default_factory=dict)

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "DocxMetadata":
        return cls(
            application=d.get("application"),
            app_version=d.get("app_version"),
            pages=d.get("pages"),
            words=d.get("words"),
            characters=d.get("characters"),
            lines=d.get("lines"),
            paragraphs=d.get("paragraphs"),
            company=d.get("company"),
            manager=d.get("manager"),
            template=d.get("template"),
            total_time=d.get("total_time"),
            doc_security=d.get("doc_security"),
            last_modified_by=d.get("last_modified_by"),
            revision=d.get("revision"),
            modified=d.get("modified"),
            extras=dict(d.get("extras", {})),
        )


@dataclass
class DocumentMetadata:
    """Document metadata: universal extracted fields + channel namespaces (CR-57).

    Only one of ``pdf`` / ``md`` / ``docx`` is populated per document — the one
    matching the source channel.
    """

    title: Optional[str] = None
    author: Optional[str] = None
    description: Optional[str] = None
    language: Optional[str] = None
    created: Optional[str] = None
    pdf: Optional[PdfMetadata] = None
    md: Optional[MdMetadata] = None
    docx: Optional[DocxMetadata] = None

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "DocumentMetadata":
        pdf = d.get("pdf")
        md = d.get("md")
        docx = d.get("docx")
        return cls(
            title=d.get("title"),
            author=d.get("author"),
            description=d.get("description"),
            language=d.get("language"),
            created=d.get("created"),
            pdf=PdfMetadata.from_dict(pdf) if pdf else None,
            md=MdMetadata.from_dict(md) if md else None,
            docx=DocxMetadata.from_dict(docx) if docx else None,
        )


@dataclass
class BookmarkSection:
    """A single entry in the PDF outline / bookmark tree (flattened, ordered)."""

    title: str
    order: int
    level: int = 1

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "BookmarkSection":
        return cls(title=d["title"], order=d["order"], level=d.get("level", 1))


@dataclass
class BookmarkData:
    """Document outline (PDF bookmarks) — an ordered, level-tagged section list."""

    sections: List[BookmarkSection] = field(default_factory=list)

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "BookmarkData":
        return cls(
            sections=[BookmarkSection.from_dict(s) for s in d.get("sections", [])]
        )


@dataclass
class DocumentInfo:
    """Document-level metadata — information *about* the document.

    ``flow_type`` lives here (schema 0.8.0+): ``"Fixed"`` for physical/reflow
    documents (PDF), ``"Free"`` for reflowable sources (markdown, docx).
    """

    root_id: str
    document_metadata: DocumentMetadata
    kind: str = "document"
    flow_type: str = "Fixed"
    outline_data: Optional[BookmarkData] = None
    topology: Optional[str] = None

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "DocumentInfo":
        outline = d.get("outline_data")
        return cls(
            root_id=d["root_id"],
            document_metadata=DocumentMetadata.from_dict(d.get("document_metadata", {})),
            kind=d.get("kind", "document"),
            flow_type=d.get("flow_type", "Fixed"),
            outline_data=BookmarkData.from_dict(outline) if outline else None,
            topology=d.get("topology"),
        )


# ---------------------------------------------------------------------------
# StructuralProfile sub-types
# ---------------------------------------------------------------------------


@dataclass
class HistogramBin:
    """A single bin in a token distribution histogram."""

    range_start: int
    range_end: int
    count: int
    token_sum: int

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "HistogramBin":
        return cls(
            range_start=d["range_start"],
            range_end=d["range_end"],
            count=d["count"],
            token_sum=d["token_sum"],
        )


@dataclass
class TokenHistogram:
    """Token count distribution for a set of nodes."""

    bins: List[HistogramBin] = field(default_factory=list)
    total_count: int = 0
    total_tokens: int = 0
    mean: float = 0.0
    median: float = 0.0
    mode: Optional[int] = None
    variance: float = 0.0

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "TokenHistogram":
        return cls(
            bins=[HistogramBin.from_dict(b) for b in d.get("bins", [])],
            total_count=d.get("total_count", 0),
            total_tokens=d.get("total_tokens", 0),
            mean=d.get("mean", 0.0),
            median=d.get("median", 0.0),
            mode=d.get("mode"),
            variance=d.get("variance", 0.0),
        )


@dataclass
class TokenDistribution:
    """Per-type and overall token distributions."""

    by_node_type: Dict[str, TokenHistogram] = field(default_factory=dict)
    overall: Optional[TokenHistogram] = None

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "TokenDistribution":
        by_type = {
            k: TokenHistogram.from_dict(v) for k, v in d.get("by_node_type", {}).items()
        }
        overall_raw = d.get("overall")
        return cls(
            by_node_type=by_type,
            overall=TokenHistogram.from_dict(overall_raw) if overall_raw else None,
        )


@dataclass
class NodeTypeDistribution:
    """Node type counts and percentages."""

    counts: Dict[str, int] = field(default_factory=dict)
    percentages: Dict[str, float] = field(default_factory=dict)

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "NodeTypeDistribution":
        return cls(
            counts=dict(d.get("counts", {})),
            percentages=dict(d.get("percentages", {})),
        )


@dataclass
class DepthDistribution:
    """Tree depth statistics."""

    max_depth: int = 0
    depth_counts: Dict[str, int] = field(default_factory=dict)
    avg_depth: float = 0.0

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "DepthDistribution":
        return cls(
            max_depth=d.get("max_depth", 0),
            depth_counts=dict(d.get("depth_counts", {})),
            avg_depth=d.get("avg_depth", 0.0),
        )


@dataclass
class StructuralProfile:
    """Statistical properties of the document graph.

    ``flow_type`` moved to :class:`DocumentInfo` (schema 0.8.0); read it from
    ``graph.document_info.flow_type``, not here.
    """

    document_type: str = "Generic"
    total_nodes: int = 0
    total_tokens: int = 0
    token_distribution: Optional[TokenDistribution] = None
    node_type_distribution: Optional[NodeTypeDistribution] = None
    depth_distribution: Optional[DepthDistribution] = None

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "StructuralProfile":
        td = d.get("token_distribution")
        ntd = d.get("node_type_distribution")
        dd = d.get("depth_distribution")
        return cls(
            document_type=d.get("document_type", "Generic"),
            total_nodes=d.get("total_nodes", 0),
            total_tokens=d.get("total_tokens", 0),
            token_distribution=TokenDistribution.from_dict(td) if td else None,
            node_type_distribution=NodeTypeDistribution.from_dict(ntd) if ntd else None,
            depth_distribution=DepthDistribution.from_dict(dd) if dd else None,
        )


# ---------------------------------------------------------------------------
# ParseProvenance
# ---------------------------------------------------------------------------


@dataclass
class ParseProvenance:
    """Provenance for the parse that produced this graph.

    Rides *beside* the graph on the wrapper (Block A), not on ``document_info``.
    CR-92: content-only — no ``source_filename`` (a transport/session detail).
    """

    blazegraph_version: str
    source_format: str
    source_sha256: str
    config_hash: str

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "ParseProvenance":
        return cls(
            blazegraph_version=d["blazegraph_version"],
            source_format=d["source_format"],
            source_sha256=d["source_sha256"],
            config_hash=d["config_hash"],
        )


# ---------------------------------------------------------------------------
# BlazeGraph — top-level return type
# ---------------------------------------------------------------------------


@dataclass
class BlazeGraph:
    """Top-level wrapper for a parsed document graph.

    This is the return type for :func:`blazegraphio.parse_pdf` and
    :func:`blazegraphio.parse_pdf_async`. Mirrors the Rust
    ``SortedDocumentGraph`` wrapper (schema 1.0.0).
    """

    schema_version: str
    nodes: List[DocumentNode]
    document_info: DocumentInfo
    structural_profile: StructuralProfile
    graph_sha256: str = ""
    created_at: str = ""
    parse_provenance: Optional[ParseProvenance] = None
    _raw: Dict[str, Any] = field(default_factory=dict, repr=False)
    _index: Dict[str, DocumentNode] = field(default_factory=dict, repr=False)

    def __post_init__(self) -> None:
        # Build ID → node index for fast lookup
        if not self._index:
            self._index = {node.id: node for node in self.nodes}

    def __repr__(self) -> str:
        return f"<BlazeGraph: {len(self.nodes)} nodes, schema v{self.schema_version}>"

    # -- Filtered accessors --

    @property
    def sections(self) -> List[DocumentNode]:
        """All Section nodes."""
        return [n for n in self.nodes if n.node_type == "Section"]

    @property
    def paragraphs(self) -> List[DocumentNode]:
        """All Paragraph nodes."""
        return [n for n in self.nodes if n.node_type == "Paragraph"]

    @property
    def root(self) -> DocumentNode:
        """The Document root node."""
        return self._index[self.document_info.root_id]

    @property
    def flow_type(self) -> str:
        """Convenience: the document's flow type (``"Fixed"`` / ``"Free"``)."""
        return self.document_info.flow_type

    # -- Lookup helpers --

    def get_node(self, node_id: str) -> DocumentNode:
        """Look up a node by UUID.

        Raises:
            KeyError: If the node ID is not found.
        """
        return self._index[node_id]

    def nodes_by_page(self, page: int) -> List[DocumentNode]:
        """Return all nodes on a specific page number."""
        return [
            n
            for n in self.nodes
            if n.location.physical is not None and n.location.physical.page == page
        ]

    # -- Render --

    def render(
        self,
        *,
        breadcrumbs: bool = False,
        node_types: bool = False,
    ) -> str:
        """Render the full document as human-readable text."""
        return self.root.render(self, breadcrumbs=breadcrumbs, node_types=node_types)

    # -- Serialization --

    def to_dict(self) -> Dict[str, Any]:
        """Return the raw dictionary (the original JSON)."""
        return self._raw

    def to_json(self) -> str:
        """Return the graph as a JSON string."""
        import json

        return json.dumps(self._raw, indent=2)

    # -- Construction --

    @classmethod
    def from_dict(cls, d: Dict[str, Any]) -> "BlazeGraph":
        """Construct a ``BlazeGraph`` from a raw dictionary (parsed JSON).

        The dictionary should have the ``SortedDocumentGraph`` shape:
        ``schema_version``, ``nodes``, ``document_info``, ``structural_profile``,
        plus the optional wrapper fields ``graph_sha256`` / ``created_at`` /
        ``parse_provenance``.
        """
        nodes = [DocumentNode.from_dict(n) for n in d["nodes"]]
        prov = d.get("parse_provenance")
        return cls(
            schema_version=d["schema_version"],
            nodes=nodes,
            document_info=DocumentInfo.from_dict(d["document_info"]),
            structural_profile=StructuralProfile.from_dict(d["structural_profile"]),
            graph_sha256=d.get("graph_sha256", ""),
            created_at=d.get("created_at", ""),
            parse_provenance=ParseProvenance.from_dict(prov) if prov else None,
            _raw=d,
        )
