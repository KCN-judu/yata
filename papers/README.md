# Papers

Public manuscripts that explain a versioned standard (ADR-0023). They are
explanatory; the normative definitions are in `docs/spec/`.

| Paper                       | Explains            | State                                   |
| --------------------------- | ------------------- | --------------------------------------- |
| `quality-model-v2/`         | `yata-quality-v2`   | current; Markdown, Typst layout and PDF |
| `archive/quality-model-v1/` | `yata-quality-v1.1` | frozen record; Markdown only            |

## Layout of a paper

- `paper.md` is the English source, and `paper.zh.md` the Chinese one where it
  exists. They are the only files edited by hand, besides the layout.
- `main.typ` and `main.zh.typ` are the layouts: the English one after ACM's
  single-column `acmsmall` format, the Chinese one after the format of Chinese
  computing journals. `../template/env.typ` and `env.zh.typ` hold the theorem,
  proof and figure environments.
- `body.typ`, `body.zh.typ` and the PDFs are generated and committed. Never edit
  them by hand.
- Numbers inside `<!-- generated:NAME -->` blocks are written by
  `formal/calibration` (`render`), never by hand.

## Building

```bash
just papers
```

`scripts/build_papers.py` runs Pandoc (verified with 3.11) from Markdown to
Typst, turns theorem, proof and figure paragraphs into the environments, and
compiles each layout with Typst (verified with 0.14). The `papers-current`
preflight check fails when a committed body is stale, and is skipped on a host
without Pandoc.

Pandoc is found on `PATH`, through the `PANDOC` variable, or under
`C:/dev/pandoc-*`. Fonts beyond Typst's own are found through
`TYPST_FONT_PATHS`:

| Font                                | Used for                            |
| ----------------------------------- | ----------------------------------- |
| Libertinus Serif (bundled by Typst) | English text                        |
| DejaVu Sans Mono (bundled by Typst) | code                                |
| Source Han Serif SC (思源宋体)      | Chinese body text                   |
| Source Han Sans SC (思源黑体)       | Chinese headings and theorem heads  |
| LXGW WenKai (霞鹜文楷)              | Chinese theorem bodies and abstract |
