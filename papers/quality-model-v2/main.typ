// Mechanics Before Preference — Typst layout.
// Primary editable source: paper.md -> body.typ (Pandoc + scripts/build_papers.py).
// Document chrome in the style of PACMPL (acmsmall), after KCN-judu/BDL_FV paper/core_calculus:
// single column, Libertinus, numbered sections, running heads, theorem/proof environments,
// ACM references.

#let title = "Mechanics Before Preference"
#let subtitle = "A Faithful, Formally Verified Quality Standard for the Souls of Onmyoji"
#let author = "KCN-judu"

#set page(
  paper: "us-letter",
  margin: (top: 1.0in, bottom: 1.0in, left: 1.1in, right: 1.1in),
  header: context {
    let p = counter(page).get().first()
    if p > 1 {
      set text(size: 8pt, fill: luma(60))
      if calc.even(p) [#p #h(1fr) #author] else [#title #h(1fr) #p]
    }
  },
)
#set text(font: "Libertinus Serif", size: 10pt, lang: "en")
#set par(justify: true, leading: 0.6em, first-line-indent: 1.1em, spacing: 0.65em)
#set heading(numbering: "1.1")
#set enum(indent: 1.0em, body-indent: 0.5em)
#set list(indent: 0.9em, body-indent: 0.5em)
#set math.equation(numbering: none)
#show math.equation.where(block: true): set block(above: 0.8em, below: 0.8em)
#show raw: set text(font: "DejaVu Sans Mono", size: 0.84em)
#show raw.where(block: true): set block(inset: (x: 0.6em, y: 0.5em), fill: luma(248), width: 100%, radius: 2pt)
#show table: set text(size: 7.6pt)
#show table.cell: set align(left + top)
#show table.cell: set par(justify: false, first-line-indent: 0em)
#show table.cell.where(y: 0): set text(weight: "bold")
#set table(inset: (x: 3pt, y: 2.5pt), stroke: (x: none, y: 0.3pt))
#show figure.where(kind: table): set block(breakable: true)
#show figure.where(kind: table): set figure(placement: none)
#show raw.where(block: false): it => {
  show regex("[_./:]"): m => m.text + sym.zws
  it
}
#show link: set text(fill: rgb("#1a3d7c"))

// ---------------------------------------------------------------- headings (acmsmall)
#show heading.where(level: 1): it => {
  v(14pt)
  set text(size: 10.5pt, weight: "bold")
  block(upper(it))
  v(4pt)
}
#show heading.where(level: 2): it => {
  v(8pt)
  set text(size: 10pt, weight: "bold")
  block(it)
  v(3pt)
}
#show heading.where(level: 3): it => {
  v(5pt)
  set text(size: 10pt, weight: "regular", style: "italic")
  block(it)
  v(2pt)
}
#show heading.where(numbering: none): it => {
  v(6pt)
  set text(size: 10pt, weight: "bold")
  block(it)
  v(2pt)
}

#import "../template/env.typ": *

// ---------------------------------------------------------------- title block
#align(left)[
  #v(0.15in)
  #text(size: 20pt, weight: "bold")[#title]
  #v(5pt)
  #text(size: 12.5pt)[#subtitle]
  #v(12pt)
  #text(size: 11pt)[#upper(author)]
  #v(2pt)
  #text(size: 9.5pt, style: "italic")[
    Yata Quality Model v2 (`yata-quality-v2`); formal development `KCN-judu/yata`, `formal/` (Lean 4.34.0, Mathlib v4.34.0); paper revision of 2026-09-25
  ]
  #v(10pt)
]

#include "body.typ"

#set par(first-line-indent: 0em)
#bibliography("references.bib", style: "association-for-computing-machinery", title: "References")
