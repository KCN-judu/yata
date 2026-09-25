"""Every check this repository runs, as one named list run by profile.

What `just fast` and `just check` run locally is what CI runs: each CI job is one profile of
this script (docs/project/ci.md). The requirements are docs/guides/engineering-requirements.md.

- Read-only: nothing is formatted or rewritten, except by the `fix` profile, which says so.
- Standard library only; tracked files come from `git ls-files` in a repository, otherwise from
  a walk of the tree filtered by .gitignore.
- A check whose tool is missing is skipped, not failed, except under --strict, which CI always
  uses: a CI job installs every tool its profile needs, so a skip there means the setup broke. A
  check that has nothing to check yet (no Cargo.toml, no app/) passes as "n/a" in either mode.

Usage: python scripts/preflight.py [fast|full|fix|<ci profile>|<check>...] [--verbose] [--strict]
"""

from __future__ import annotations

import fnmatch
import importlib.util
import json
import os
import re
import shutil
import subprocess
import sys
import time
import tomllib
from collections.abc import Callable
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

PRETTIER = "prettier@3.9.9"
MARKDOWNLINT = "markdownlint-cli2@0.23.3"

MAX_LINES = 1000
# path -> (line ceiling, reason). A ceiling, so an exempt file cannot keep growing.
LENGTH_ALLOWLIST: dict[str, tuple[int, str]] = {}
GENERATED = ("/gen/", "/generated/", ".pb.dart", ".pbenum.dart", ".pbjson.dart", ".pbserver.dart")

# Every workspace crate and its class (ADR-0005). A crate missing here fails `crate-graph`.
CRATE_CLASS: dict[str, str] = {
    "yata-core": "pure",
    "yata-protocol": "pure",
    "yata-store": "pure",
    "yata-daemon": "effectful",
}

# ADR-0019: SQL lives only in yata-store; rusqlite only in the daemon's executor module.
SQL_CRATE = "crates/yata-store/"
EXECUTOR = "crates/yata-daemon/src/store/executor.rs"
SQL_LITERAL = re.compile(r'"[^"\n]*\b(SELECT|INSERT|UPDATE|DELETE|CREATE|DROP|ALTER|PRAGMA|BEGIN|COMMIT)\b[^"\n]*"')
SQL_CALL = re.compile(r"\.(execute|execute_batch|prepare|prepare_cached|query_row|query_map)\s*\(")

# ADR-0023: the quality standard's formal artifacts, and the documents holding generated numbers.
LEAN_DIR = "formal/lean"
CALIBRATION = "formal/calibration/Cargo.toml"
QUALITY_DOCS = (
    "docs/spec/quality-model.md",
    "papers/quality-model-v2/paper.md",
    "papers/quality-model-v2/paper.zh.md",
)

# ADR-0011: private keys and certificates are never tracked, whatever their name.
SIGNING_MATERIAL = (".key", ".pem", ".p8", ".p12", ".pfx", ".snk", ".keystore", ".jks")

# ADR-0012: the error codes are the oneof cases of core.proto, and nothing else spells one.
CORE_PROTO = "crates/yata-protocol/proto/core.proto"
CODE_NAMESPACES = ("session", "query", "command", "job", "decode", "import", "store", "client", "internal")
CODE_LITERAL = re.compile(rf"""["']({"|".join(CODE_NAMESPACES)})\.([a-z_]+)["']""")
# Where Rust and Dart are searched for spelled-out codes, and the one file allowed to spell them.
CODE_SCAN = ("crates/", "app/lib/")
CODE_LITERAL_HOMES = (CORE_PROTO,)
# The messages whose `oneof kind` cases are the error codes: per request, per stream, client only.
CODE_MESSAGES = ("Error", "SessionFailed", "ClientFailure")
# The wording of every explanation: a cause and a remedy per code, never blank (A-Q4).
ARB_ZH = "app/lib/l10n/app_zh.arb"

# ADR-0017: the SVG profile of committed icons.
ICON_ROOT = "app/assets/icons/"
ICON_PATH = re.compile(r"^app/assets/icons/(soul-set|shikigami)/(emblem|portrait)/\d+\.svg$")
SVG_FORBIDDEN = re.compile(r"<(text|script|foreignObject|image|style|animate\w*|set|filter|use)\b|href\s*=\s*\"(?!#)")


@dataclass
class Result:
    ok: bool
    output: str = ""
    skipped: bool = False  # a tool is missing: a failure under --strict
    na: bool = False  # nothing to check yet: passes in every mode


@dataclass
class Check:
    name: str
    description: str
    run: Callable[[], Result]
    tools: tuple[str, ...]
    hint: str


# ---------------------------------------------------------------- files


def _ignore_patterns() -> list[str]:
    gi = ROOT / ".gitignore"
    if not gi.exists():
        return []
    lines = gi.read_text(encoding="utf-8").splitlines()
    return [ln.strip() for ln in lines if ln.strip() and not ln.lstrip().startswith(("#", "!"))]


def _ignored(relpath: str, is_dir: bool, patterns: list[str]) -> bool:
    name = relpath.rsplit("/", 1)[-1]
    for pat in patterns:
        dir_only = pat.endswith("/")
        p = pat.rstrip("/")
        if dir_only and not is_dir:
            continue
        if "/" in p:
            if fnmatch.fnmatch(relpath, p.lstrip("/")):
                return True
        elif fnmatch.fnmatch(name, p):
            return True
    return False


def tracked_files() -> list[str]:
    """Paths relative to the root, POSIX style: what is or would be committed."""
    if (ROOT / ".git").exists() and shutil.which("git"):
        out = subprocess.run(
            ["git", "ls-files", "--cached", "--others", "--exclude-standard"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            encoding="utf-8",
            check=True,
        ).stdout
        return sorted(ln for ln in out.splitlines() if ln and (ROOT / ln).is_file())
    patterns = [*_ignore_patterns(), ".git/"]
    found: list[str] = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        base = Path(dirpath).relative_to(ROOT).as_posix()
        base = "" if base == "." else base + "/"
        dirnames[:] = sorted(d for d in dirnames if not _ignored(base + d, True, patterns))
        found.extend(base + f for f in filenames if not _ignored(base + f, False, patterns))
    return sorted(found)


def markdown_files() -> list[str]:
    return [f for f in tracked_files() if f.endswith(".md")]


def _run(cmd: list[str], cwd: Path = ROOT) -> Result:
    exe = shutil.which(cmd[0]) or cmd[0]
    proc = subprocess.run([exe, *cmd[1:]], cwd=cwd, capture_output=True, text=True, encoding="utf-8", errors="replace")
    return Result(proc.returncode == 0, (proc.stdout + proc.stderr).strip())


def _has_rust() -> bool:
    return (ROOT / "Cargo.toml").exists()


def _has_app() -> bool:
    return (ROOT / "app" / "pubspec.yaml").exists()


def _read(f: str) -> str:
    return (ROOT / f).read_text(encoding="utf-8")


# ---------------------------------------------------------------- repository checks


def python_location() -> Result:
    bad = [f for f in tracked_files() if f.endswith(".py") and not f.startswith("scripts/")]
    return Result(not bad, "\n".join(f"{f}: Python outside scripts/ (ADR-0013)" for f in bad))


def publication() -> Result:
    """Nothing ADR-0016 or ADR-0017 keeps local is about to be committed."""
    local = ("research/", ".claude/", "local-assets/")
    bad = [f"{f}: local-only path (ADR-0016, ADR-0017)" for f in tracked_files() if f.startswith(local)]
    bad += [f"{f}: CLAUDE.md is agent tooling (ADR-0016)" for f in tracked_files() if f.endswith("CLAUDE.md")]
    bad += [
        f"{f}: signing material; the release key lives only in CI secrets (ADR-0011)"
        for f in tracked_files()
        if f.endswith(SIGNING_MATERIAL)
    ]
    bad += [
        f"{f}: raster icon in the tracked tree; project icons are SVG (ADR-0017)"
        for f in tracked_files()
        if f.startswith(ICON_ROOT) and not f.endswith(".svg")
    ]
    return Result(not bad, "\n".join(bad))


def file_length() -> Result:
    bad: list[str] = []
    for f in tracked_files():
        if not f.endswith((".rs", ".dart", ".py")) or any(g in f for g in GENERATED):
            continue
        if f.startswith("tests/") or "/tests/" in f or "/test/" in f:
            continue
        n = len(_read(f).splitlines())
        limit, reason = LENGTH_ALLOWLIST.get(f, (MAX_LINES, ""))
        if n > limit:
            note = f" (allowlisted to {limit}: {reason})" if reason else ""
            bad.append(f"{f}: {n} lines > {limit}{note}")
    return Result(not bad, "\n".join(bad))


def docs_validate() -> Result:
    return _run([sys.executable, "scripts/validate_docs.py"])


def status_shape() -> Result:
    page = ROOT / "docs" / "project" / "status.md"
    if not page.exists():
        return Result(False, "docs/project/status.md: missing")
    lines = page.read_text(encoding="utf-8").splitlines()
    bad: list[str] = []
    snap = next((i for i, ln in enumerate(lines) if ln.startswith("**Snapshot:**")), None)
    if snap is not None:
        n = 0
        while snap + n < len(lines) and lines[snap + n].strip():
            n += 1
        if n > 3:
            bad.append(f"status.md: snapshot is {n} lines > 3")
    header: list[str] = []
    for i, ln in enumerate(lines, 1):
        if not ln.startswith("|"):
            header = [] if not ln.strip() else header
            continue
        if re.match(r"^\|[\s|:-]+\|$", ln):
            continue
        cells = [c.strip() for c in ln.strip().strip("|").split("|")]
        if not header:
            header = [c.lower() for c in cells]
            continue
        row = dict(zip(header, cells, strict=False))
        bad += [f"status.md:{i}: cell of {len(c)} characters > 300" for c in cells if len(c) > 300]
        state, evidence = row.get("state", ""), row.get("evidence", "")
        if state in {"implemented", "tested"} and evidence in {"", "—"}:
            bad.append(f"status.md:{i}: '{state}' with no evidence")
    return Result(not bad, "\n".join(bad))


def docs_format() -> Result:
    return _run(["npx", "--yes", PRETTIER, "--check", *markdown_files()])


def docs_lint() -> Result:
    return _run(["npx", "--yes", MARKDOWNLINT, *markdown_files()])


def _module(name: str, *args: str) -> Result:
    """A Python tool run as `python -m`, skipped when it is not installed."""
    if importlib.util.find_spec(name) is None:
        return Result(True, f"{name} not installed (pip install -r requirements-dev.txt)", skipped=True)
    return _run([sys.executable, "-m", name, *args])


def python_lint() -> Result:
    a = _module("ruff", "check", "scripts")
    b = _module("ruff", "format", "--check", "scripts")
    return Result(a.ok and b.ok, "\n".join(x for x in (a.output, b.output) if x), a.skipped)


def python_types() -> Result:
    return _module("mypy", "--strict", "scripts")


# ---------------------------------------------------------------- Rust


def _rust(cmd: list[str]) -> Result:
    if not _has_rust():
        return Result(True, "no Cargo.toml yet", na=True)
    return _run(cmd)


def rust_format() -> Result:
    return _rust(["cargo", "fmt", "--all", "--check"])


def rust_check() -> Result:
    return _rust(["cargo", "check", "--workspace", "--all-targets", "--locked"])


def rust_clippy() -> Result:
    return _rust(["cargo", "clippy", "--workspace", "--all-targets", "--locked", "--", "-D", "warnings"])


def rust_test() -> Result:
    return _rust(["cargo", "test", "--workspace", "--locked"])


def rust_version() -> Result:
    """The pinned toolchain is the declared minimum, so every build and test runs at the MSRV."""
    if not _has_rust():
        return Result(True, "no Cargo.toml yet", na=True)
    workspace = tomllib.loads(_read("Cargo.toml"))["workspace"]
    msrv = str(workspace["package"].get("rust-version", ""))
    bad: list[str] = []
    if not re.fullmatch(r"\d+\.\d+", msrv):
        bad.append(f"Cargo.toml: workspace rust-version '{msrv}' is not <major>.<minor>")
    pin = str(tomllib.loads(_read("rust-toolchain.toml"))["toolchain"]["channel"])
    if not pin.startswith(msrv + "."):
        bad.append(f"rust-toolchain.toml: channel {pin} is not a {msrv}.x release (rust-version {msrv})")
    for member in workspace["members"]:
        package = tomllib.loads(_read(f"{member}/Cargo.toml"))["package"]
        if package.get("rust-version") != {"workspace": True}:
            bad.append(f"{member}/Cargo.toml: rust-version must be 'rust-version.workspace = true'")
    for f in tracked_files():
        if f.endswith("rust-toolchain.toml") and f != "rust-toolchain.toml":
            other = str(tomllib.loads(_read(f))["toolchain"]["channel"])
            if other != pin:
                bad.append(f"{f}: channel {other} differs from rust-toolchain.toml ({pin})")
    return Result(not bad, "\n".join(bad))


def _lake() -> str | None:
    """Lake on PATH, or where elan installs it by default."""
    found = shutil.which("lake")
    if found:
        return found
    for name in ("lake.exe", "lake"):
        candidate = Path.home() / ".elan" / "bin" / name
        if candidate.exists():
            return str(candidate)
    return None


def lean_build() -> Result:
    """Every theorem of the quality standard checks (ADR-0023)."""
    lake = _lake()
    if lake is None:
        return Result(True, "lake not installed (elan)", skipped=True)
    return _run([lake, "build"], ROOT / LEAN_DIR)


def quality_calibration() -> Result:
    """The calibration program is clean, and every number it generates matches what is committed."""
    if shutil.which("cargo") is None:
        return Result(True, "cargo not installed", skipped=True)
    manifest = ["--manifest-path", CALIBRATION]
    steps = [
        _run(["cargo", "fmt", *manifest, "--check"]),
        _run(["cargo", "clippy", *manifest, "--release", "--", "-D", "warnings"]),
        _run(["cargo", "run", *manifest, "--release", "--quiet", "--", "check", *QUALITY_DOCS]),
    ]
    return Result(all(s.ok for s in steps), "\n".join(s.output for s in steps if not s.ok and s.output))


def papers_current() -> Result:
    """Every paper's generated Typst body matches its Markdown source (ADR-0023)."""
    r = _run([sys.executable, "scripts/build_papers.py", "--check"])
    if "pandoc not found" in r.output:
        return Result(True, r.output, skipped=True)
    return r


def crate_graph() -> Result:
    """Every crate is classed; no pure crate depends on an effectful one; lints are inherited."""
    if not _has_rust():
        return Result(True, "no Cargo.toml yet", na=True)
    proc = subprocess.run(
        [shutil.which("cargo") or "cargo", "metadata", "--format-version", "1", "--no-deps"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        encoding="utf-8",
    )
    if proc.returncode != 0:
        return Result(False, proc.stderr.strip())
    meta = json.loads(proc.stdout)
    bad: list[str] = []
    members = {p["name"]: p for p in meta["packages"]}
    for name, pkg in sorted(members.items()):
        cls = CRATE_CLASS.get(name)
        if cls is None:
            bad.append(f"{name}: not classed pure or effectful in preflight CRATE_CLASS (ADR-0005)")
            continue
        for dep in pkg["dependencies"]:
            if dep["name"] in members and cls == "pure" and CRATE_CLASS.get(dep["name"]) != "pure":
                bad.append(f"{name}: pure crate depends on effectful {dep['name']} (ADR-0005)")
        manifest = Path(pkg["manifest_path"]).read_text(encoding="utf-8")
        lints = re.search(r"^\[lints\]\s*\n(.*?)(?=^\[|\Z)", manifest, re.S | re.M)
        if not lints or lints.group(1).strip() != "workspace = true":
            bad.append(f"{name}: [lints] must be exactly 'workspace = true' (ADR-0005)")
        if re.search(r"^\[lints\.", manifest, re.M):
            bad.append(f"{name}: re-declares lints (ADR-0005)")
        if cls == "pure" and not (Path(pkg["manifest_path"]).parent / "clippy.toml").exists():
            bad.append(f"{name}: pure crate without clippy.toml disallowing effects (ADR-0005)")
    return Result(not bad, "\n".join(bad))


def sql_boundary() -> Result:
    bad: list[str] = []
    for f in tracked_files():
        if not f.endswith((".rs", ".toml")):
            continue
        text = _read(f)
        if f.endswith(".toml"):
            if "rusqlite" in text and f not in ("Cargo.toml", "crates/yata-daemon/Cargo.toml"):
                bad.append(f"{f}: depends on rusqlite outside the daemon (ADR-0019)")
            continue
        if not f.startswith(SQL_CRATE):
            for n, line in enumerate(text.splitlines(), 1):
                if SQL_LITERAL.search(line):
                    bad.append(f"{f}:{n}: SQL text outside yata-store (ADR-0019)")
        if f != EXECUTOR:
            if re.search(r"\brusqlite\b", text):
                bad.append(f"{f}: uses rusqlite outside the executor module (ADR-0019)")
            if f.startswith("crates/yata-daemon/") and SQL_CALL.search(text):
                bad.append(f"{f}: runs statements outside the executor module (ADR-0019)")
    return Result(not bad, "\n".join(bad))


# ---------------------------------------------------------------- release (ADR-0011)


def release_selftest() -> Result:
    """Naming, manifest, checksums, and the refusal to sign, on fixture files; no build."""
    return _run([sys.executable, "scripts/release.py", "selftest"])


def release_package() -> Result:
    """The daemon builds in release mode, is packaged unsigned, and the package verifies."""
    if not _has_rust():
        return Result(True, "no Cargo.toml yet", na=True)
    if shutil.which("cargo") is None:
        return Result(True, "cargo not installed", skipped=True)
    out = "target/preflight/release"
    shutil.rmtree(ROOT / out, ignore_errors=True)
    made = _run([sys.executable, "scripts/release.py", "package", "--unsigned", "--out", out])
    return Result(made.ok, made.output, na=made.ok and made.output.startswith("n/a:"))


# ---------------------------------------------------------------- Flutter


def _app(cmd: list[str]) -> Result:
    if not _has_app():
        return Result(True, "no app/ yet", na=True)
    if shutil.which(cmd[0]) is None:
        return Result(True, f"{cmd[0]} not installed", skipped=True)
    return _run(cmd, ROOT / "app")


def _dart_sources() -> list[str]:
    """Hand-written Dart under app/, relative to it: generated code keeps its generator's layout."""
    return [
        f.removeprefix("app/")
        for f in tracked_files()
        if f.startswith("app/") and f.endswith(".dart") and not any(g in f for g in GENERATED)
    ]


def dart_format() -> Result:
    return _app(["dart", "format", "--page-width", "100", "--output=none", "--set-exit-if-changed", *_dart_sources()])


def _flutter(cmd: list[str]) -> Result:
    """A Flutter check, after generating the localizations: they are not committed (ADR-0012), and
    `flutter analyze` on a fresh checkout does not generate them."""
    generated = _app(["flutter", "gen-l10n"])
    if not generated.ok or generated.skipped or generated.na:
        return generated
    return _app(cmd)


def flutter_analyze() -> Result:
    return _flutter(["flutter", "analyze"])


def flutter_test() -> Result:
    return _flutter(["flutter", "test"])


def dart_layers() -> Result:
    bad: list[str] = []
    for f in tracked_files():
        if f.startswith("app/lib/ui/") and f.endswith(".dart") and re.search(r"import\s+'[^']*daemon/", _read(f)):
            bad.append(f"{f}: ui/ imports daemon/ (ADR-0012)")
    return Result(not bad, "\n".join(bad))


def dart_bindings() -> Result:
    """The committed Dart bindings are what protoc generates from the schema (ADR-0004, rule 2)."""
    if not _has_app():
        return Result(True, "no app/ yet", na=True)
    proc = subprocess.run(
        [sys.executable, "scripts/gen_dart_protocol.py", "--check"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    out = (proc.stdout + proc.stderr).strip()
    return Result(proc.returncode in (0, 3), out, skipped=proc.returncode == 3)


def _without_tests(path: str, text: str) -> str:
    """The text before a Rust file's `#[cfg(test)]`: by convention its test module closes the file."""
    return text.split("#[cfg(test)]", 1)[0] if path.endswith(".rs") else text


def _proto_message(proto: str, name: str) -> str | None:
    """The body of top-level `message <name>`, braces balanced; None when there is none."""
    m = re.search(rf"^message {name}\s*\{{", proto, re.M)
    if m is None:
        return None
    depth, i = 1, m.end()
    while depth and i < len(proto):
        depth += {"{": 1, "}": -1}.get(proto[i], 0)
        i += 1
    return proto[m.end() : i - 1]


def _oneof_kind(body: str | None) -> list[str] | None:
    """The field names of `oneof kind` in a message body; None when the body has no such oneof."""
    block = re.search(r"\boneof kind\s*\{([^{}]*)\}", body or "")
    if block is None:
        return None
    return re.findall(r"^\s*[\w.]+\s+([a-z][a-z0-9_]*)\s*=\s*\d+\s*;", block.group(1), re.M)


def core_code_problems(proto: str, sources: dict[str, str]) -> list[str]:
    """The error codes are the cases of `oneof kind` in core.proto's `Error`, `SessionFailed` and
    `ClientFailure` (ADR-0012): the case is the code, and its message is the code's debug record.
    Every case names a known namespace, no code is a case of two of them, and no source outside
    core.proto spells a code as a literal. That each code has a cause and a remedy is proved by the
    app's exhaustive switch, not here."""
    bad: list[str] = []
    seen: dict[str, str] = {}
    for message in CODE_MESSAGES:
        fields = _oneof_kind(_proto_message(proto, message))
        if fields is None:
            bad.append(f"{CORE_PROTO}: message {message} has no oneof kind")
            continue
        for field in fields:
            code = field.replace("_", ".", 1)
            if code.split(".", 1)[0] not in CODE_NAMESPACES or "." not in code:
                bad.append(f"{message}.{field}: outside the namespaces {', '.join(CODE_NAMESPACES)}")
            if code in seen:
                bad.append(f"{code}: a case of both {seen[code]} and {message}; a code has one home")
            seen.setdefault(code, message)
    for path, text in sorted(sources.items()):
        for m in CODE_LITERAL.finditer(_without_tests(path, text)):
            line = text.count("\n", 0, m.start()) + 1
            bad.append(f"{path}:{line}: '{m.group(1)}.{m.group(2)}' spelled out; derive it from the oneof case")
    return bad


def error_codes() -> Result:
    """The codes are the schema's oneof cases, and no source spells one out."""
    if not _has_app():
        return Result(True, "no app/ yet", na=True)
    sources = {
        f: _read(f)
        for f in tracked_files()
        if f.startswith(CODE_SCAN)
        and f.endswith((".rs", ".dart"))
        and not any(g in f for g in GENERATED)
        and "/tests/" not in f
        and f not in CODE_LITERAL_HOMES
    }
    found = core_code_problems(_read(CORE_PROTO), sources)
    return Result(not found, "\n".join(found))


def explanation_problems(arb: str) -> list[str]:
    """Every `cause*` message has its `remedy*`, the other way round too, and none is blank. That
    each code reaches a cause and a remedy is proved by the exhaustive switch; the text is not."""
    messages = {k: v for k, v in json.loads(arb).items() if not k.startswith("@")}
    causes = {k.removeprefix("cause") for k in messages if k.startswith("cause")}
    remedies = {k.removeprefix("remedy") for k in messages if k.startswith("remedy")}
    bad = [f"cause{k}: no remedy{k} in {ARB_ZH}" for k in sorted(causes - remedies)]
    bad += [f"remedy{k}: no cause{k} in {ARB_ZH}" for k in sorted(remedies - causes)]
    bad += [
        f"{k}: blank in {ARB_ZH}"
        for k, v in sorted(messages.items())
        if k.startswith(("cause", "remedy")) and not str(v).strip()
    ]
    return bad


def explanation_text() -> Result:
    if not _has_app():
        return Result(True, "no app/ yet", na=True)
    found = explanation_problems(_read(ARB_ZH))
    return Result(not found, "\n".join(found))


def checks_selftest() -> Result:
    """Planted defects the code checks must catch, and clean inputs they must pass."""
    cases: list[tuple[str, list[str] | None, int | None]] = []
    core = (
        "message Error {\n  oneof kind {\n    StoreNewerFormat store_newer_format = 2;\n"
        "    InternalBug internal_bug = 3;\n  }\n  string message = 1;\n}\n"
        "message StoreNewerFormat {\n  uint32 found = 1;\n}\n"
        "message SessionFailed {\n  oneof kind {\n    MalformedFrame session_malformed_frame = 2;\n  }\n"
        "  string message = 1;\n}\n"
        "message ClientFailure {\n  oneof kind {\n    Timeout client_timeout = 1;\n  }\n}\n"
    )
    clean = {"crates/x/src/a.rs": "let c = code(&kind);\n"}
    planted = {"crates/yata-core/src/a.rs": 'fn f() {}\nconst C: &str = "store.newer_format";\n'}
    in_dart = {"app/lib/ui/common/explanation.dart": "'client.timeout' => x,\n"}
    in_test = {"crates/x/src/a.rs": 'fn f() {}\n#[cfg(test)]\nmod tests { const C: &str = "store.newer_format"; }\n'}
    twice = core.replace("client_timeout = 1", "client_timeout = 1;\n    Bug internal_bug = 2")
    cases += [
        ("three oneofs and no literal pass", core_code_problems(core, clean), 0),
        ("a planted literal in yata-core fails", core_code_problems(core, planted), 1),
        ("a literal in the Dart explanations fails", core_code_problems(core, in_dart), 1),
        ("a literal in the test module passes", core_code_problems(core, in_test), 0),
        ("an unknown namespace fails", core_code_problems(core.replace("internal_bug", "widget_bug"), clean), 1),
        ("a code in two oneofs fails", core_code_problems(twice, clean), 1),
        ("a missing ClientFailure fails", core_code_problems(core.split("message ClientFailure")[0], clean), 1),
        (
            "an Error with a plain code fails",
            core_code_problems(core.replace("oneof kind", "oneof other", 1), clean),
            1,
        ),
    ]
    arb = '{"@@locale": "zh", "causeX": "a", "remedyX": "b", "@causeX": {}}'
    cases += [
        ("a paired cause and remedy pass", explanation_problems(arb), 0),
        ("a blank remedy fails", explanation_problems(arb.replace('"b"', '" "')), 1),
        ("a cause without its remedy fails", explanation_problems(arb.replace("remedyX", "remedyY")), 2),
    ]
    bad = [
        f"{name}: expected {want} problems, got {got}"
        for name, got, want in cases
        if (None if got is None else len(got)) != want
    ]
    return Result(not bad, "\n".join(bad))


def icon_profile() -> Result:
    bad: list[str] = []
    for f in tracked_files():
        if not f.startswith(ICON_ROOT) or not f.endswith(".svg"):
            continue
        if not ICON_PATH.match(f):
            bad.append(f"{f}: not <kind>/<role>/<id>.svg (ADR-0017)")
        text = _read(f)
        m = SVG_FORBIDDEN.search(text)
        if m:
            bad.append(f"{f}: '{m.group(0)}' is outside the icon SVG profile (ADR-0017)")
        vb = re.search(r'viewBox="\s*[-\d.]+\s+[-\d.]+\s+([\d.]+)\s+([\d.]+)\s*"', text)
        if not vb or vb.group(1) != vb.group(2):
            bad.append(f"{f}: needs a square viewBox (ADR-0017)")
    return Result(not bad, "\n".join(bad))


# ---------------------------------------------------------------- mutating


def fix_formatting() -> Result:
    print("fix: rewriting files with cargo fmt, dart format, prettier, markdownlint --fix, ruff format")
    steps: list[Result] = []
    if _has_rust() and shutil.which("cargo"):
        steps.append(_run(["cargo", "fmt", "--all"]))
    if _has_app() and shutil.which("dart"):
        steps.append(_run(["dart", "format", "--page-width", "100", *_dart_sources()], ROOT / "app"))
    if shutil.which("npx"):
        md = markdown_files()
        steps.append(_run(["npx", "--yes", PRETTIER, "--write", *md]))
        steps.append(_run([sys.executable, "scripts/md_normalize.py", *md]))
        steps.append(_run(["npx", "--yes", MARKDOWNLINT, "--fix", *md]))
    if importlib.util.find_spec("ruff") is not None:
        steps.append(_run([sys.executable, "-m", "ruff", "format", "scripts"]))
    return Result(all(s.ok for s in steps), "\n".join(s.output for s in steps if s.output))


CHECKS = [
    Check("python-location", "no .py outside scripts/", python_location, (), "move it to scripts/ (ADR-0013)"),
    Check("publication", "no local-only file is tracked", publication, (), "keep it in research/ or local-assets/"),
    Check("file-length", "no hand-written file over 1 000 lines", file_length, (), "split by responsibility"),
    Check("docs-validate", "engineering records are well-formed", docs_validate, (), "python scripts/validate_docs.py"),
    Check("status-shape", "status.md is scannable; claims carry evidence", status_shape, (), "shorten the cell"),
    Check("docs-format", "Prettier would change no Markdown", docs_format, ("npx",), "just fmt"),
    Check("docs-lint", "markdownlint reports nothing", docs_lint, ("npx",), "just fmt, then fix what remains"),
    Check("python-lint", "ruff clean on scripts/", python_lint, (), "ruff check --fix scripts; just fmt"),
    Check("python-types", "mypy --strict clean on scripts/", python_types, (), "annotate; narrow Any"),
    Check("rust-format", "cargo fmt would change nothing", rust_format, ("cargo",), "cargo fmt --all"),
    Check("rust-check", "the workspace builds", rust_check, ("cargo",), "cargo check --workspace --all-targets"),
    Check("rust-clippy", "clippy with -D warnings is clean", rust_clippy, ("cargo",), "fix the lint, never allow it"),
    Check("rust-test", "workspace tests pass", rust_test, ("cargo",), "cargo test --workspace"),
    Check("crate-graph", "crates classed; pure not on effectful; lints inherited", crate_graph, ("cargo",), "ADR-0005"),
    Check("rust-version", "the pinned toolchain is the declared rust-version", rust_version, (), "pin <msrv>.x"),
    Check("sql-boundary", "SQL only in yata-store; rusqlite only in the executor", sql_boundary, (), "ADR-0019"),
    Check("dart-format", "dart format would change nothing", dart_format, (), "just fmt"),
    Check("flutter-analyze", "flutter analyze is clean", flutter_analyze, (), "fix the analyzer finding"),
    Check("flutter-test", "Flutter tests pass", flutter_test, (), "cd app; flutter test"),
    Check("dart-layers", "app/lib/ui never imports app/lib/daemon", dart_layers, (), "go through state/ (ADR-0012)"),
    Check("icon-profile", "committed icons follow the SVG profile and layout", icon_profile, (), "ADR-0017"),
    Check("error-codes", "codes are core.proto oneof cases, spelled nowhere else", error_codes, (), "ADR-0012"),
    Check("explanation-text", "every cause has its remedy, none blank", explanation_text, (), "fill app_zh.arb"),
    Check(
        "checks-selftest", "the code checks fail on planted defects", checks_selftest, (), "fix the check, not the case"
    ),
    Check(
        "dart-bindings",
        "the committed Dart bindings match the schema",
        dart_bindings,
        (),
        "python scripts/gen_dart_protocol.py",
    ),
    Check("lean-build", "every Lean theorem of the quality standard checks", lean_build, (), "lake build"),
    Check(
        "quality-calibration",
        "calibration clean; generated numbers match what is committed",
        quality_calibration,
        (),
        "cargo run --manifest-path formal/calibration/Cargo.toml --release -- render <documents>",
    ),
    Check("papers-current", "every paper's Typst body matches its source", papers_current, (), "just papers"),
    Check("release-selftest", "release naming, manifest, checksums on fixtures", release_selftest, (), "ADR-0011"),
    Check(
        "release-package",
        "the daemon packages, unsigned, on this platform",
        release_package,
        (),
        "python scripts/release.py package --unsigned",
    ),
    Check("fix-formatting", "MUTATES: runs every formatter", fix_formatting, (), "fix what the formatters report"),
]
STRUCTURE = [
    "python-location",
    "publication",
    "file-length",
    "docs-validate",
    "status-shape",
    "sql-boundary",
    "dart-layers",
    "icon-profile",
    "error-codes",
    "explanation-text",
    "checks-selftest",
]
DOCS = ["docs-format", "docs-lint"]
PYTHON = ["python-lint", "python-types", "release-selftest"]
RUST_FAST = ["rust-format", "crate-graph", "rust-version", "rust-check"]
RUST_FULL = ["rust-format", "crate-graph", "rust-version", "rust-clippy", "rust-test"]
# What differs by target: cfg-gated code, paths, process spawning, the release build. Formatting,
# the crate graph and the toolchain pin are the same on every platform and are proved on Linux.
PLATFORM = ["rust-clippy", "rust-test", "release-package"]
FLUTTER = ["dart-bindings", "dart-format", "flutter-analyze", "flutter-test"]
FORMAL = ["lean-build", "quality-calibration", "papers-current"]
PROFILES = {
    "fast": STRUCTURE + DOCS + PYTHON + RUST_FAST + ["dart-format", "flutter-analyze"],
    "full": STRUCTURE + DOCS + PYTHON + RUST_FULL + FLUTTER + FORMAL,
    "platform": PLATFORM,
    "docs-ci": STRUCTURE + DOCS + PYTHON,
    "rust-ci": RUST_FULL,
    "platform-ci": PLATFORM,
    "flutter-ci": FLUTTER,
    "formal-ci": FORMAL,
    "fix": ["fix-formatting"],
}


def main(argv: list[str]) -> int:
    verbose = "--verbose" in argv or bool(os.environ.get("GITHUB_ACTIONS"))
    strict = "--strict" in argv or bool(os.environ.get("GITHUB_ACTIONS"))
    args = [a for a in argv if not a.startswith("--")] or ["fast"]
    by_name = {c.name: c for c in CHECKS}
    names: list[str] = []
    for a in args:
        if a in PROFILES:
            names += PROFILES[a]
        elif a in by_name:
            names.append(a)
        else:
            print(f"unknown profile or check '{a}'; profiles: {', '.join(PROFILES)}; checks: {', '.join(by_name)}")
            return 2
    failed: list[str] = []
    timings: list[tuple[float, str]] = []
    for name in dict.fromkeys(names):
        c = by_name[name]
        missing = [t for t in c.tools if shutil.which(t) is None]
        if missing:
            print(f"  {'FAIL' if strict else 'skip':<5} {name:<16} {', '.join(missing)} not installed")
            if strict:
                print("        hint: --strict: the job must install every tool its profile needs")
                failed.append(name)
            continue
        t0 = time.perf_counter()
        r = c.run()
        dt = time.perf_counter() - t0
        timings.append((dt, name))
        if r.skipped and strict:
            r = Result(False, r.output + "\n--strict: the job must install every tool its profile needs")
        tag = "n/a" if r.na else "skip" if r.skipped else "ok" if r.ok else "FAIL"
        print(f"  {tag:<5} {name:<16} {dt:5.2f}s  {c.description}")
        if (not r.ok or r.skipped or r.na or verbose) and r.output:
            print("\n".join("        " + ln for ln in r.output.splitlines()))
        if not r.ok:
            print(f"        hint: {c.hint}")
            failed.append(name)
    slow = [f"{n} {t:.1f}s" for t, n in sorted(timings, reverse=True) if t > 5.0]
    if slow:
        print(f"slow: {', '.join(slow)}")
    print(f"preflight: {len(failed)} failed: {', '.join(failed)}" if failed else "preflight: all passed")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
