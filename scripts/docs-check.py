#!/usr/bin/env python3
"""docs-check — assert the published docs say what the code does.

`make test` has never read a word of docs/. These four cheap gates cover the
truth gaps CR-94 §G named — each checkable against source, none reachable from
the hermetic suite:

  1. env vars  — every variable the docker guide documents that the Dockerfile
                 ALSO sets must carry the Dockerfile's default. (A var the guide
                 documents but the Dockerfile does not set — RUST_LOG, a
                 framework default — and a var the Dockerfile sets but the guide
                 omits — JAVA_HOME, internal — are reported, not failed.)
  2. routes    — every /v1/* and /health path the docs cite must be a real route
                 in py/server/main.py.
  3. sdk types — the python-sdk "Type Reference" table must list EXACTLY the
                 SDK's public data types (bragi.__all__ minus functions/errors).
  4. links     — every relative markdown link must resolve. The relative
                 structure is identical in the mono and the projected repo, so a
                 link that resolves from the doc's own directory resolves in both.

Exit 0 = every gate green. 1 = at least one truth gap. 2 = could not run.
"""

import ast
import re
import sys
from pathlib import Path

BRAGI = Path(__file__).resolve().parent.parent
DOCKERFILE = BRAGI / "Dockerfile"
MAIN = BRAGI / "py/server/main.py"
SDK_INIT = BRAGI / "py/sdk/src/bragi/__init__.py"
DOCS = BRAGI / "docs"
ALL_DOCS = sorted(DOCS.glob("guides/*.md")) + sorted(DOCS.glob("reference/*.md"))


def rel(p):
    return p.relative_to(BRAGI)


# --- 1. env vars ⇄ Dockerfile ----------------------------------------------

def check_env():
    print("\n── env vars ⇄ Dockerfile")
    dock = {}
    for line in DOCKERFILE.read_text().splitlines():
        m = re.match(r"\s*ENV\s+([A-Z_][A-Z0-9_]*)=(\S+)", line)
        if m:
            dock[m.group(1)] = m.group(2)

    doc = {}
    for line in (DOCS / "guides/03-docker.md").read_text().splitlines():
        # env table rows: | `VAR` | `default` | desc |
        m = re.match(r"\|\s*`([A-Z_][A-Z0-9_]*)`\s*\|\s*`?([^|`]+?)`?\s*\|", line)
        if m:
            doc[m.group(1)] = m.group(2).strip()

    mismatched = [(v, doc[v], dock[v]) for v in sorted(doc) if v in dock and doc[v] != dock[v]]
    for v, d, k in mismatched:
        print("  ✗ %-18s doc says %s, Dockerfile sets %s" % (v, d, k))
    for v in sorted(set(doc) - set(dock)):
        print("  · %-18s documented, not set by the Dockerfile (framework default)" % v)
    for v in sorted(set(dock) - set(doc)):
        print("  · %-18s set by the Dockerfile, not documented (internal)" % v)
    ok = not mismatched
    print("  %s %d documented, %d in Dockerfile, %d mismatch"
          % ("✓" if ok else "✗", len(doc), len(dock), len(mismatched)))
    return ok


# --- 2. routes ⇄ FastAPI app ------------------------------------------------

def check_routes():
    print("\n── routes ⇄ py/server/main.py")
    real = set()
    for line in MAIN.read_text().splitlines():
        m = re.search(r"@app\.\w+\(\s*[\"']([^\"']+)[\"']", line)
        if m:
            real.add(m.group(1))

    cited = {}
    for doc in ALL_DOCS:
        for m in re.finditer(r"/(?:v1|health)[A-Za-z0-9_/-]*", doc.read_text()):
            cited.setdefault(m.group(0), set()).add(doc.name)

    missing = sorted(p for p in cited if p not in real)
    for p in sorted(cited):
        where = ", ".join(sorted(cited[p]))
        if p in real:
            print("  ✓ %-20s cited in %s" % (p, where))
        else:
            print("  ✗ %-20s cited in %s — NO such route (real: %s)"
                  % (p, where, ", ".join(sorted(real))))
    ok = not missing
    print("  %s %d route(s) cited, %d not in the app" % ("✓" if ok else "✗", len(cited), len(missing)))
    return ok


# --- 3. sdk Type Reference ⇄ __all__ ---------------------------------------

def check_types():
    print("\n── sdk Type Reference ⇄ bragi.__all__")
    tree = ast.parse(SDK_INIT.read_text())
    names = []
    for node in ast.walk(tree):
        if isinstance(node, ast.Assign) and any(
                isinstance(t, ast.Name) and t.id == "__all__" for t in node.targets):
            names = [e.value for e in node.value.elts if isinstance(e, ast.Constant)]
    # Data types: CapWords, not an *Error, not a snake_case function.
    types = {n for n in names if n[:1].isupper() and not n.endswith("Error")}

    documented, in_table = set(), False
    for line in (DOCS / "guides/02-python-sdk.md").read_text().splitlines():
        if line.strip().startswith("## "):
            in_table = "Type Reference" in line
        if in_table:
            m = re.match(r"\|\s*`([A-Za-z_]\w*)`\s*\|", line)
            if m:
                documented.add(m.group(1))

    phantom = sorted(documented - types)      # documented, not a real export
    undocumented = sorted(types - documented)  # a real type, missing from the table
    for n in phantom:
        print("  ✗ %-22s in the table, NOT in __all__" % n)
    for n in undocumented:
        print("  ✗ %-22s exported, MISSING from the table" % n)
    ok = not phantom and not undocumented
    print("  %s %d documented, %d exported types, %d phantom, %d undocumented"
          % ("✓" if ok else "✗", len(documented), len(types), len(phantom), len(undocumented)))
    return ok


# --- 4. relative links resolve ---------------------------------------------

def strip_fenced_code(text):
    """Blank out fenced code blocks, keeping line count so nothing shifts.

    A doc that *shows* markdown syntax — an image reference in a schema
    example, say — is not linking to anything. Scanning inside fences
    made every such example a phantom broken link.
    """
    out, fenced = [], False
    for line in text.split("\n"):
        if line.lstrip().startswith("```"):
            fenced = not fenced
            out.append("")
            continue
        out.append("" if fenced else line)
    return "\n".join(out)


def check_links():
    print("\n── relative links resolve")
    broken = []
    total = 0
    for doc in ALL_DOCS:
        for m in re.finditer(r"\[[^\]]*\]\(([^)]+)\)", strip_fenced_code(doc.read_text())):
            target = m.group(1).strip()
            if "://" in target or target.startswith(("#", "mailto:")):
                continue
            path = target.split("#", 1)[0]
            if not path:
                continue
            total += 1
            if not (doc.parent / path).exists():
                broken.append((doc, target))
    for doc, target in broken:
        print("  ✗ %s → %s (does not resolve)" % (rel(doc), target))
    ok = not broken
    print("  %s %d relative link(s), %d broken" % ("✓" if ok else "✗", total, len(broken)))
    return ok


def main():
    for p in (DOCKERFILE, MAIN, SDK_INIT, DOCS):
        if not p.exists():
            print("docs-check: missing %s" % rel(p), file=sys.stderr)
            return 2
    print("docs-check — %s" % rel(DOCS))
    results = {
        "env vars": check_env(),
        "routes": check_routes(),
        "sdk types": check_types(),
        "links": check_links(),
    }
    print("\n" + "─" * 50)
    failed = [k for k, ok in results.items() if not ok]
    for k, ok in results.items():
        print("  %s %s" % ("✓" if ok else "✗", k))
    if failed:
        print("\ndocs-check FAILED: %s" % ", ".join(failed))
        return 1
    print("\ndocs-check ok — every gate green.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
