"""The release skeleton: deterministic names, version metadata, a manifest, checksums, a signing slot.

ADR-0011 decides what a release carries; docs/spec/release-manifest.md states the draft manifest
this script writes. Nothing here publishes, and nothing signs yet: `package` refuses to run
without `--unsigned` until an Ed25519 signer is wired to the CI secret. The private key is never
read from the tree.

The package holds the daemon alone, because the daemon is the only program this repository
builds today; the application and the installers join it when they exist.

Usage:
  python scripts/release.py package --unsigned [--out DIR]   build, stage and verify for this host
  python scripts/release.py verify DIR                         check a staged package
  python scripts/release.py selftest                           the rules above, on fixtures
"""

from __future__ import annotations

import hashlib
import json
import platform
import re
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path
from typing import Any, Protocol

ROOT = Path(__file__).resolve().parents[1]

PRODUCT = "yata"
MANIFEST = "manifest.json"
SIGNATURE = "manifest.sig"
CHECKSUMS = "SHA256SUMS"
# The draft format; 1 is assigned when a daemon first verifies a manifest (release-manifest.md).
MANIFEST_FORMAT = 0

# ADR-0008: the platforms the application ships on, and each one's executable suffix.
PLATFORMS: dict[str, str] = {"windows-x64": ".exe", "macos-arm64": ""}
VERSION = re.compile(r"^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$")

STORE_SOURCE = "crates/yata-daemon/src/store/mod.rs"
STORE_VERSION = re.compile(r"pub const STORE_FORMAT_VERSION: u32 = (\d+);")
PROTOCOL_SOURCE = "crates/yata-protocol/src/lib.rs"
PROTOCOL_VERSION = re.compile(r"ProtocolVersion \{ major: (\d+), minor: (\d+) \}")


class ReleaseError(Exception):
    pass


class Signer(Protocol):
    """Signs the canonical manifest bytes; returns the detached signature as text."""

    def sign(self, message: bytes) -> str: ...


def signer(unsigned: bool) -> Signer | None:
    """The release signer, or None for an explicitly unsigned test package.

    ADR-0011: an Ed25519 key held only in CI secrets. Until it is wired, a signed package is
    refused rather than produced with a stand-in, so nothing unsigned can pass for a release.
    """
    if unsigned:
        return None
    raise ReleaseError("release signing is not wired yet (ADR-0011); pass --unsigned for a test package")


# ---------------------------------------------------------------- metadata


def release_name(version: str, plat: str) -> str:
    """`yata-<version>-<platform>`: the same inputs always give the same name."""
    if not VERSION.match(version):
        raise ReleaseError(f"version '{version}' is not <major>.<minor>.<patch>[-<pre>]")
    if plat not in PLATFORMS:
        raise ReleaseError(f"'{plat}' is not a supported platform ({', '.join(PLATFORMS)}; ADR-0008)")
    return f"{PRODUCT}-{version}-{plat}"


def host_platform() -> str | None:
    machine = platform.machine().lower()
    if sys.platform == "win32" and machine in {"amd64", "x86_64"}:
        return "windows-x64"
    if sys.platform == "darwin" and machine in {"arm64", "aarch64"}:
        return "macos-arm64"
    return None


def _search(path: str, pattern: re.Pattern[str]) -> re.Match[str]:
    m = pattern.search((ROOT / path).read_text(encoding="utf-8"))
    if m is None:
        raise ReleaseError(f"{path}: '{pattern.pattern}' not found; update scripts/release.py")
    return m


def metadata() -> dict[str, Any]:
    """Version metadata from the sources: the workspace version, store format, probe protocol."""
    workspace = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))["workspace"]
    protocol = _search(PROTOCOL_SOURCE, PROTOCOL_VERSION)
    return {
        "version": str(workspace["package"]["version"]),
        "store_format_version": int(_search(STORE_SOURCE, STORE_VERSION).group(1)),
        "probe_protocol_version": f"{protocol.group(1)}.{protocol.group(2)}",
    }


def source_commit() -> str:
    """HEAD, with `-dirty` when tracked files differ from it."""
    git = shutil.which("git") or "git"
    head = subprocess.run([git, "rev-parse", "HEAD"], cwd=ROOT, capture_output=True, text=True, check=True)
    status = subprocess.run(
        [git, "status", "--porcelain", "--untracked-files=no"], cwd=ROOT, capture_output=True, text=True, check=True
    )
    return head.stdout.strip() + ("-dirty" if status.stdout.strip() else "")


# ---------------------------------------------------------------- manifest and checksums


def canonical(manifest: dict[str, Any]) -> bytes:
    """The bytes that are hashed and signed: sorted keys, no whitespace, ASCII only."""
    return json.dumps(manifest, sort_keys=True, separators=(",", ":"), ensure_ascii=True).encode("ascii")


def _sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _entries(package: Path, names: list[str]) -> list[dict[str, Any]]:
    return [{"path": n, "size": (package / n).stat().st_size, "sha256": _sha256(package / n)} for n in sorted(names)]


def stage(files: dict[str, Path], meta: dict[str, Any], plat: str, commit: str, out: Path, sign: Signer | None) -> Path:
    """Copy `files` (package path -> source) into `out/<name>/` with its manifest and checksums."""
    package = out / release_name(meta["version"], plat)
    if package.exists():
        raise ReleaseError(f"{package}: already exists; staging never overwrites")
    package.mkdir(parents=True)
    for name, source in files.items():
        if "/" in name or "\\" in name or name in {MANIFEST, SIGNATURE, CHECKSUMS}:
            raise ReleaseError(f"'{name}': package files are flat and may not shadow the manifest")
        shutil.copyfile(source, package / name)
    manifest = {
        "manifest_format": MANIFEST_FORMAT,
        "product": PRODUCT,
        "platform": plat,
        "commit": commit,
        **meta,
        "files": _entries(package, list(files)),
    }
    body = canonical(manifest)
    (package / MANIFEST).write_bytes(body)
    listed = [MANIFEST]
    if sign is not None:
        (package / SIGNATURE).write_text(sign.sign(body) + "\n", encoding="ascii", newline="\n")
        listed.append(SIGNATURE)
    sums = "".join(f"{e['sha256']}  {e['path']}\n" for e in _entries(package, [*files, *listed]))
    (package / CHECKSUMS).write_text(sums, encoding="ascii", newline="\n")
    return package


def verify(package: Path) -> list[str]:
    """Every problem with a staged package; empty when it is intact."""
    bad: list[str] = []
    body = (package / MANIFEST).read_bytes()
    manifest = json.loads(body)
    if canonical(manifest) != body:
        bad.append(f"{MANIFEST}: not in canonical form")
    try:
        if package.name != release_name(str(manifest.get("version")), str(manifest.get("platform"))):
            bad.append(f"{package.name}: the name does not match the manifest's version and platform")
    except ReleaseError as e:
        bad.append(f"{MANIFEST}: {e}")
    if manifest.get("manifest_format") != MANIFEST_FORMAT:
        bad.append(f"{MANIFEST}: manifest_format {manifest.get('manifest_format')} is not {MANIFEST_FORMAT}")
    for entry in manifest.get("files", []):
        f = package / entry["path"]
        if not f.is_file():
            bad.append(f"{entry['path']}: listed but missing")
        elif f.stat().st_size != entry["size"] or _sha256(f) != entry["sha256"]:
            bad.append(f"{entry['path']}: size or SHA-256 differs from the manifest")
    listed = {e["path"] for e in manifest.get("files", [])} | {MANIFEST, SIGNATURE, CHECKSUMS}
    bad += [f"{p.name}: present but not in the manifest" for p in sorted(package.iterdir()) if p.name not in listed]
    for line in (package / CHECKSUMS).read_text(encoding="ascii").splitlines():
        digest, _, name = line.partition("  ")
        if not (package / name).is_file() or _sha256(package / name) != digest:
            bad.append(f"{CHECKSUMS}: {name} does not match")
    if (package / SIGNATURE).exists():
        bad.append(f"{SIGNATURE}: present, but signature verification is not wired yet (ADR-0011)")
    return bad


# ---------------------------------------------------------------- commands


def package_host(out: Path, unsigned: bool) -> int:
    sign = signer(unsigned)
    plat = host_platform()
    if plat is None:
        print(f"n/a: {sys.platform}/{platform.machine()} is not a supported platform (ADR-0008)")
        return 0
    cargo = shutil.which("cargo") or "cargo"
    build = subprocess.run([cargo, "build", "--release", "--locked", "-p", "yata-daemon"], cwd=ROOT)
    if build.returncode != 0:
        raise ReleaseError("cargo build --release failed")
    exe = "yata-daemon" + PLATFORMS[plat]
    package = stage({exe: ROOT / "target" / "release" / exe}, metadata(), plat, source_commit(), ROOT / out, sign)
    problems = verify(package)
    print("\n".join(problems) if problems else f"package: {package.as_posix()}")
    return 1 if problems else 0


def selftest() -> int:
    """The naming, manifest and checksum rules, on fixture files under target/preflight."""
    work = ROOT / "target" / "preflight" / "release-selftest"
    shutil.rmtree(work, ignore_errors=True)
    work.mkdir(parents=True)
    fixture = work / "fixture.bin"
    fixture.write_bytes(bytes(range(256)) * 4)
    meta = {"version": "1.2.3", "store_format_version": 1, "probe_protocol_version": "1.0"}
    failures: list[str] = []

    def expect(ok: bool, what: str) -> None:
        if not ok:
            failures.append(what)

    for plat, suffix in PLATFORMS.items():
        a = stage({"yata-daemon" + suffix: fixture}, meta, plat, "0" * 40, work / "a", None)
        b = stage({"yata-daemon" + suffix: fixture}, meta, plat, "0" * 40, work / "b", None)
        expect(a.name == f"yata-1.2.3-{plat}", f"{plat}: name {a.name}")
        expect((a / MANIFEST).read_bytes() == (b / MANIFEST).read_bytes(), f"{plat}: manifest is not deterministic")
        expect((a / CHECKSUMS).read_bytes() == (b / CHECKSUMS).read_bytes(), f"{plat}: checksums not deterministic")
        expect(verify(a) == [], f"{plat}: a fresh package does not verify: {verify(a)}")
        expect(not (a / SIGNATURE).exists(), f"{plat}: an unsigned package carries a signature")
        (b / ("yata-daemon" + suffix)).write_bytes(b"tampered")
        expect(any("differs" in p for p in verify(b)), f"{plat}: a tampered file verifies")
    for version, plat in (("1.2", "windows-x64"), ("1.2.3", "linux-x64")):
        try:
            release_name(version, plat)
            failures.append(f"{version}/{plat}: accepted")
        except ReleaseError:
            pass
    try:
        signer(unsigned=False)
        failures.append("a signed package was produced without a signer")
    except ReleaseError:
        pass
    try:
        stage({"yata-daemon": fixture}, meta, "macos-arm64", "0" * 40, work / "a", None)
        failures.append("staging overwrote an existing package")
    except ReleaseError:
        pass
    shutil.rmtree(work, ignore_errors=True)
    print("\n".join(failures) if failures else "release selftest: all passed")
    return 1 if failures else 0


def main(argv: list[str]) -> int:
    try:
        match argv:
            case ["package", "--unsigned"]:
                return package_host(Path("target/release-stage"), unsigned=True)
            case ["package", "--unsigned", "--out", out]:
                return package_host(Path(out), unsigned=True)
            case ["package", *_]:
                return package_host(Path("target/release-stage"), unsigned=False)
            case ["verify", path]:
                problems = verify(Path(path) if Path(path).is_absolute() else ROOT / path)
                print("\n".join(problems) if problems else f"{Path(path).name}: intact")
                return 1 if problems else 0
            case ["selftest"]:
                return selftest()
            case _:
                print(__doc__)
                return 2
    except ReleaseError as e:
        print(f"release: {e}")
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
