#!/usr/bin/env python3
"""Build the papers under papers/ (ADR-0023): Markdown -> Pandoc -> Typst -> PDF.

Each paper directory holds `paper.md` (English, canonical) and may hold `paper.zh.md`
(Chinese). For each source this script

1. runs Pandoc (verified with 3.11) to Typst, giving `body.typ` / `body.zh.typ`;
2. maps the symbol names Pandoc emits for pre-0.13 Typst to current ones;
3. turns theorem, proof and figure paragraphs into the environments of
   `papers/template/env.typ` / `env.zh.typ`;
4. compiles `main.typ` / `main.zh.typ` with Typst to `<paper>.pdf` / `<paper>.zh.pdf`.

The pipeline follows KCN-judu/BDL_FV `paper/*/build.sh`. Fonts beyond Typst's own are
found through `TYPST_FONT_PATHS` (see `papers/README.md`).

    python scripts/build_papers.py            # regenerate bodies and PDFs
    python scripts/build_papers.py --check    # fail if a committed body is stale

`body*.typ` is generated and never edited by hand. The script only orchestrates tools and
rewrites text; it holds no project logic (ADR-0013).
"""

from __future__ import annotations

import glob
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PAPERS = ROOT / "papers"

# Pandoc's pre-0.13 Typst symbol names and their current names (as BDL_FV's build.sh).
SYMBOLS = [
    (r"bracket\.l\.double", "bracket.l.stroked"),
    (r"bracket\.r\.double", "bracket.r.stroked"),
    (r"angle\.l", "chevron.l"),
    (r"angle\.r", "chevron.r"),
    (r"gt\.tri", "gt.closed"),
    (r"tack\.r([_^])", r"scripts(tack.r)\1"),
]

KINDS_EN = "Theorem|Proposition|Lemma|Corollary|Definition|Remark|Example"
KINDS_ZH = "定理|命题|引理|推论|定义|注记|例"
# The full-width parentheses and full stop of Chinese theorem heads and captions.
FW_OPEN, FW_CLOSE, FW_STOP = chr(0xFF08), chr(0xFF09), chr(0xFF0E)


def find_pandoc() -> str | None:
    """Pandoc on PATH, from `PANDOC`, or unpacked under C:/dev (the Windows convention here)."""
    env = os.environ.get("PANDOC")
    if env and Path(env).exists():
        return env
    found = shutil.which("pandoc")
    if found:
        return found
    candidates = sorted(glob.glob("C:/dev/pandoc-*/pandoc.exe"))
    return candidates[-1] if candidates else None


def words(m: re.Match[str]) -> str:
    return f'{m.group(1)}("{m.group(2).replace(" ", "")}")'


def postprocess(text: str, zh: bool) -> str:
    """Pandoc's Typst into the paper environments."""
    for old, new in SYMBOLS:
        text = re.sub(old, new, text)
    text = re.sub(r"\b(sans|upright|italic|bold)\(([A-Za-z](?: [A-Za-z])+)\)", words, text)
    # Pandoc fixes column widths from the Markdown dash rules, which the formatter pads; let
    # Typst size the columns to their content instead.
    text = re.sub(
        r"columns: \(([0-9.%, ]+)\)",
        lambda m: f"columns: {len(m.group(1).rstrip(', ').split(','))}",
        text,
    )
    # A citation is a label reference; in Chinese text the next character would join the label,
    # so every citation becomes an explicit call.
    text = re.sub(r"@([a-z]+\d{4}[a-z]*)", r"#cite(<\1>)", text)
    # Pandoc writes a heading's label and the next paragraph without a blank line between.
    text = re.sub(r"^(<[^>\n]+>)\n(?=\S)", r"\1\n\n", text, flags=re.M)
    out: list[str] = []
    for para in text.split("\n\n"):
        if zh:
            m = re.match(rf"#strong\[({KINDS_ZH})(?:\s+(\d+))?{FW_OPEN}(.*?){FW_CLOSE}\]\s*(.*)", para, flags=re.S)
        else:
            m = re.match(rf"#strong\[({KINDS_EN})(?:\s+(\d+))?\s+\((.*?)\)\.\]\s*(.*)", para, flags=re.S)
        if m:
            kind, num, name, body = m.groups()
            out.append(f'#thm("{kind}", "{num or ""}")[{name}][{body}]')
            continue
        head = "证明." if zh else "Proof."
        m = re.match(
            rf"#emph\[{re.escape(head)}\]\s*(.*?)\s*\$square(?:\.[a-z.]+)?\$\s*$",
            para,
            flags=re.S,
        )
        if m:
            out.append(f"#proof[{m.group(1)}]")
            continue
        m = re.match(rf"#emph\[((?:Figure|图) ?\d+[.{FW_STOP}].*)\]\s*$", para, flags=re.S)
        if m:
            out.append(f"#figcaption[{m.group(1)}]")
            continue
        out.append(para)
    env = "env.zh.typ" if zh else "env.typ"
    return f'#import "../template/{env}": *\n\n' + "\n\n".join(out)


def body_of(pandoc: str, source: Path, zh: bool) -> str:
    # The source has one H1, the title, so that it lints as one document; the shift turns it
    # into metadata (the layout prints the title) and the H2 sections into numbered sections.
    proc = subprocess.run(
        [
            pandoc,
            str(source),
            "-f",
            # A line break between CJK characters is not a space.
            "markdown+east_asian_line_breaks" if zh else "markdown",
            "-t",
            "typst",
            "--shift-heading-level-by=-1",
        ],
        capture_output=True,
        text=True,
        encoding="utf-8",
        check=True,
    )
    return postprocess(proc.stdout.replace("\r\n", "\n"), zh)


def jobs() -> list[tuple[Path, Path, Path, Path]]:
    """(source, body, main, pdf) for every paper source with a Typst layout. A paper without
    one, such as a frozen Markdown manuscript, is not built."""
    out: list[tuple[Path, Path, Path, Path]] = []
    for d in sorted(p for p in PAPERS.iterdir() if p.is_dir() and p.name not in ("template", "archive")):
        for src, body, main, pdf in (
            ("paper.md", "body.typ", "main.typ", f"{d.name}.pdf"),
            ("paper.zh.md", "body.zh.typ", "main.zh.typ", f"{d.name}.zh.pdf"),
        ):
            if (d / src).exists() and (d / main).exists():
                out.append((d / src, d / body, d / main, d / pdf))
    return out


def main(argv: list[str]) -> int:
    check = "--check" in argv
    pandoc = find_pandoc()
    if pandoc is None:
        print("pandoc not found (PATH, PANDOC, or C:/dev/pandoc-*); nothing built", file=sys.stderr)
        return 0 if check else 2
    stale: list[str] = []
    for source, body, main_typ, pdf in jobs():
        zh = source.name.endswith(".zh.md")
        text = body_of(pandoc, source, zh)
        current = body.read_text(encoding="utf-8") if body.exists() else ""
        if current != text:
            stale.append(str(body.relative_to(ROOT)))
            if not check:
                body.write_text(text, encoding="utf-8", newline="\n")
        if check:
            continue
        typst = shutil.which("typst")
        if typst is None:
            print("typst not found; bodies regenerated, PDFs not compiled", file=sys.stderr)
            return 2
        subprocess.run([typst, "compile", "--root", str(PAPERS), str(main_typ), str(pdf)], check=True)
        print(f"built {pdf.relative_to(ROOT)}")
    if check and stale:
        print("stale generated body; run `just papers`:\n  " + "\n  ".join(stale), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
