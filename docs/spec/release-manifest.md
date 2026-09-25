---
kind: spec
status: current
area: tooling
---

# Release manifest (draft)

The package names, the manifest, and the checksum file that
[ADR-0011](../decisions/0011-packaging-and-self-update.md) asks every release to
carry, as `scripts/release.py` writes them today. The format is a **draft**:
`manifest_format` is `0` until a daemon first verifies a manifest, and until
then any field may change without a version bump. Nothing is signed or published
yet.

## Packages

One package per shipping platform of
[ADR-0008](../decisions/0008-platforms-and-export-file-import.md):

```text
yata-<version>-<platform>/
  <files>          the programs of the package, flat
  manifest.json    the manifest, in canonical form
  manifest.sig     the detached signature; absent while signing is not wired
  SHA256SUMS       one "<sha256>  <name>" line per file above, sorted by name
```

- `<version>` is the workspace version in `Cargo.toml`, `major.minor.patch` with
  an optional `-pre` suffix.
- `<platform>` is `windows-x64` or `macos-arm64`. No other platform is packaged.
- The same version and platform always give the same name. A staging directory
  is never overwritten.

Today a package holds `yata-daemon` alone, because the daemon is the only
program this repository builds. The application, the installer, and the `.dmg`
of ADR-0011 are not built yet.

## Manifest

| Field                     | Value                                                                                                  |
| ------------------------- | ------------------------------------------------------------------------------------------------------ |
| `manifest_format`         | `0`, the draft                                                                                         |
| `product`                 | `"yata"`                                                                                               |
| `version`                 | as in the package name                                                                                 |
| `platform`                | as in the package name                                                                                 |
| `commit`                  | the source commit, with `-dirty` when tracked files differed from it                                   |
| `store_format_version`    | `STORE_FORMAT_VERSION` of `yata-daemon`, the store format this build writes                            |
| `snapshot_schema_version` | `snapshot::VERSION` of `yata-protocol`, as `"major.minor"`: the yata-snapshot schema this build writes |
| `files`                   | every program file: `path`, `size` in bytes, `sha256` in lowercase hexadecimal                         |

The canonical form, which is what is hashed and what will be signed, is the JSON
object with keys sorted, no whitespace, and ASCII only. `files` is sorted by
`path`.

ADR-0011 asks for the _minimum_ store format and protocol versions a release
reads. The draft records the versions the build itself writes and speaks, and
the two coincide while only version 1 of each exists. The distinction is settled
before `manifest_format` becomes `1`.

## Signing

The signer takes the canonical manifest bytes and returns a detached signature.
ADR-0011 fixes the algorithm (Ed25519) and where the private key lives (CI
secrets only). Until the signer is wired to that secret, `release.py package`
refuses to run without `--unsigned`, and `release.py verify` refuses a package
that carries a signature it cannot check. Private keys and certificates are
never tracked: the `publication` check fails on them.

## Open

- where releases are published, and how the release key is rotated (ADR-0011)
- the minimum-version semantics above
- OS code signing and notarization (ISS-0003)

## Related

- What CI proves about this: [../project/ci.md](../project/ci.md)
- The decision: [ADR-0011](../decisions/0011-packaging-and-self-update.md)
