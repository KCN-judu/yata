# Yata

Yata (八咫镜) is a desktop tool for players of 阴阳师 (Onmyoji). It imports the
player's own souls (御魂), filters them with the same conditions as the game's
own soul schemes, and scores them, both by their own quality and by how well
they fit a Shikigami's needs.

Nothing is released yet. The design is recorded in [docs/](docs/README.md), and
[docs/project/status.md](docs/project/status.md) says what exists.

Yata is an unofficial fan project, not affiliated with or endorsed by NetEase.
The game's names, data, and artwork belong to NetEase and are not covered by
this repository's license.

## How it is built

| Part                                | Technology                         | Where                                                           |
| ----------------------------------- | ---------------------------------- | --------------------------------------------------------------- |
| the core: domain, decoding, scoring | Rust, pure functions               | `crates/`                                                       |
| the application                     | Flutter desktop, presentation only | `app/`                                                          |
| reading the game                    | a separate, open-source reader     | [KCN-judu/yata-reader](https://github.com/KCN-judu/yata-reader) |

The core runs as a child process, `yata-daemon`, and the application talks to it
over a typed protocol.
[docs/architecture/overview.md](docs/architecture/overview.md) explains the
layers.

## Working in this repository

The requirements every change meets are
[docs/guides/engineering-requirements.md](docs/guides/engineering-requirements.md).

| Tool                         | Version                           |
| ---------------------------- | --------------------------------- |
| Rust                         | pinned by `rust-toolchain.toml`   |
| Flutter                      | 3.44.7 (Dart 3.12.2)              |
| protoc                       | 36.2                              |
| Python (tooling only)        | 3.14, with `requirements-dev.txt` |
| Node (Markdown tooling only) | 24                                |
| just                         | 1.58                              |

```bash
just fast   # before a commit
just check  # before a push: everything CI runs
just fmt    # rewrites files with every formatter
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option. Unless you state otherwise, any
contribution you submit for inclusion is dual licensed as above, without any
additional terms or conditions.
