"""CLI subprocess wrapper for local-mode PDF processing."""

from __future__ import annotations

import json
import subprocess
import tempfile
from pathlib import Path

from bragi._download import find_or_download_cli, get_jre_dir
from bragi.errors import BragiProcessingError
from bragi.types import BragiGraph


def _local_parse_pdf(
    path: str,
    *,
    config_path: str | None = None,
) -> BragiGraph:
    """Parse a PDF using the bragi binary.

    Args:
        path: Path to the PDF file.
        config_path: Optional path to a config YAML file.

    Returns:
        A :class:`BragiGraph` with fully typed nodes.

    Raises:
        BragiProcessingError: If the CLI exits with an error.
        BragiNotFoundError: If the CLI binary cannot be found.
        FileNotFoundError: If the PDF file does not exist.
    """
    pdf_path = Path(path)
    if not pdf_path.exists():
        raise FileNotFoundError(f"PDF not found: {path}")

    cli_path = find_or_download_cli()
    jre_dir = get_jre_dir()

    with tempfile.TemporaryDirectory() as tmpdir:
        output_path = Path(tmpdir) / "graph.json"

        cmd = [
            str(cli_path),
            "parse",
            "-i", str(pdf_path),
            "-f", "bgraph",
            "-o", str(output_path),
        ]
        # No usable JRE found (no JAVA_HOME, no packaged JRE): omit the flag
        # so the CLI's own auto-download runs instead of trusting an empty dir.
        if jre_dir is not None:
            cmd.extend(["--jre-path", str(jre_dir)])

        if config_path:
            cmd.extend(["--config", str(config_path)])

        print(f"Processing {pdf_path.name}... ", end="", flush=True)

        result = subprocess.run(
            cmd,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
            check=False,
        )

        if result.returncode != 0:
            stderr_text = result.stderr.decode("utf-8", errors="replace").strip()
            print("failed.")
            raise BragiProcessingError(
                f"bragi exited with code {result.returncode}: {stderr_text}"
            )

        if not output_path.exists():
            print("failed.")
            raise BragiProcessingError(
                "bragi completed but did not produce output."
            )

        print("done.")

        raw = json.loads(output_path.read_text(encoding="utf-8"))
        return BragiGraph.from_dict(raw)
