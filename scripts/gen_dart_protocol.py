"""Generate the Dart bindings of the core protocol schema, or check the committed ones.

The Rust side generates its bindings at build time; the Dart side commits them, so no contributor
needs `protoc` to build the application, and CI checks them byte for byte (ADR-0004, rule 2).
This script runs `protoc` with `protoc-gen-dart` and nothing else.

`protoc` is found through PROTOC or PATH, the plugin through PROTOC_GEN_DART or PATH
(`dart pub global activate protoc_plugin`). Without them `--check` reports itself skipped.

Usage: python scripts/gen_dart_protocol.py [--check]
"""

from __future__ import annotations

import filecmp
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PROTO_DIR = ROOT / "crates" / "yata-protocol" / "proto"
SCHEMAS = ("core.proto",)
OUT_DIR = ROOT / "app" / "lib" / "gen" / "proto"
# Exit code for "the tools are missing": preflight reports the check skipped, not failed.
SKIPPED = 3


def _tool(env: str, name: str) -> str | None:
    return os.environ.get(env) or shutil.which(name)


def generate(out: Path) -> subprocess.CompletedProcess[str] | None:
    protoc = _tool("PROTOC", "protoc")
    plugin = _tool("PROTOC_GEN_DART", "protoc-gen-dart")
    if protoc is None or plugin is None:
        return None
    out.mkdir(parents=True, exist_ok=True)
    cmd = [protoc, f"--plugin=protoc-gen-dart={plugin}", f"--dart_out={out}", f"-I{PROTO_DIR}", *SCHEMAS]
    return subprocess.run(cmd, cwd=PROTO_DIR, capture_output=True, text=True, encoding="utf-8", errors="replace")


def _files(d: Path) -> list[str]:
    return sorted(p.relative_to(d).as_posix() for p in d.rglob("*") if p.is_file())


def check() -> int:
    with tempfile.TemporaryDirectory() as tmp:
        proc = generate(Path(tmp))
        if proc is None:
            print("protoc or protoc-gen-dart not found; nothing checked")
            return SKIPPED
        if proc.returncode != 0:
            print(proc.stdout + proc.stderr)
            return 1
        fresh, committed = _files(Path(tmp)), _files(OUT_DIR) if OUT_DIR.exists() else []
        bad = [f"{f}: generated but not committed" for f in fresh if f not in committed]
        bad += [f"{f}: committed but no longer generated" for f in committed if f not in fresh]
        bad += [
            f"{f}: differs from what protoc generates"
            for f in fresh
            if f in committed and not filecmp.cmp(Path(tmp) / f, OUT_DIR / f, shallow=False)
        ]
    for line in bad:
        print(f"app/lib/gen/proto/{line}")
    return 1 if bad else 0


def main(argv: list[str]) -> int:
    if "--check" in argv:
        return check()
    if OUT_DIR.exists():
        shutil.rmtree(OUT_DIR)
    proc = generate(OUT_DIR)
    if proc is None:
        print("protoc and protoc-gen-dart are required: set PROTOC and PROTOC_GEN_DART, or put them on PATH")
        return 2
    print(proc.stdout + proc.stderr, end="")
    return proc.returncode


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
