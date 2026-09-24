// 先机制，后偏好 —— 中文版版式。
// 可编辑的源文件是 paper.zh.md，经 Pandoc 与 scripts/build_papers.py 生成 body.zh.typ。
// 版式参照国内计算机期刊（软件学报、计算机学报）：A4 单栏，宋体正文，黑体标题，
// 中文摘要、关键词、中图法分类号在前，英文题名与摘要随后，参考文献按 GB/T 7714。

#let title = "先机制，后偏好"
#let subtitle = "《阴阳师》御魂品质评分标准的忠实建模与形式化验证"
#let title-en = "Mechanics Before Preference: A Faithful, Formally Verified Quality Standard for the Souls of Onmyoji"
#let author = "KCN-judu"

#let song = ("Source Han Serif SC", "SimSun")
#let hei = ("Source Han Sans SC", "SimHei")
#let kai = ("LXGW WenKai", "KaiTi")
#let latin = "Libertinus Serif"

#set page(
  paper: "a4",
  margin: (top: 2.6cm, bottom: 2.4cm, left: 2.2cm, right: 2.2cm),
  header: context {
    let p = counter(page).get().first()
    if p > 1 {
      set text(size: 8pt, font: (latin, ..song))
      if calc.even(p) [#p #h(1fr) #author：#title] else [#subtitle #h(1fr) #p]
      v(-0.6em)
      line(length: 100%, stroke: 0.4pt)
    }
  },
)
#set text(font: (latin, ..song), size: 10.5pt, lang: "zh", region: "cn")
#set par(justify: true, leading: 0.75em, first-line-indent: (amount: 2em, all: true), spacing: 0.75em)
#set heading(numbering: "1.1")
#set enum(indent: 1.0em, body-indent: 0.5em)
#set list(indent: 1.0em, body-indent: 0.5em)
#set math.equation(numbering: none)
#show math.equation.where(block: true): set block(above: 0.8em, below: 0.8em)
#show raw: set text(font: ("DejaVu Sans Mono", ..hei), size: 0.84em)
#show raw.where(block: true): set block(inset: (x: 0.6em, y: 0.5em), fill: luma(248), width: 100%, radius: 2pt)
#show table: set text(size: 7.4pt)
#show table.cell: set align(left + top)
#show table.cell: set par(justify: false, first-line-indent: 0em)
#show table.cell.where(y: 0): set text(font: (latin, ..hei), weight: "bold")
#set table(inset: (x: 3pt, y: 2.5pt), stroke: (x: none, y: 0.3pt))
#show figure.where(kind: table): set block(breakable: true)
#show figure.where(kind: table): set figure(placement: none)
#show raw.where(block: false): it => {
  show regex("[_./:]"): m => m.text + sym.zws
  it
}
#show link: set text(fill: rgb("#1a3d7c"))
#show emph: it => text(font: (latin, ..kai), style: "normal", it.body)
#show strong: it => text(font: (latin, ..hei), weight: "bold", it.body)

// ---------------------------------------------------------------- 标题（期刊式）
#show heading.where(level: 1): it => {
  v(10pt)
  set text(font: (latin, ..hei), size: 12pt, weight: "bold")
  block(it)
  v(3pt)
}
#show heading.where(level: 2): it => {
  v(6pt)
  set text(font: (latin, ..hei), size: 10.5pt, weight: "bold")
  block(it)
  v(2pt)
}
#show heading.where(level: 3): it => {
  v(4pt)
  set text(font: (latin, ..kai), size: 10.5pt)
  block(it)
  v(1pt)
}
#show heading.where(numbering: none): it => {
  v(8pt)
  set text(font: (latin, ..hei), size: 12pt, weight: "bold")
  block(it)
  v(3pt)
}

#import "../template/env.zh.typ": *

#let field(label, body) = par(first-line-indent: 0em, hanging-indent: 0em)[
  #text(font: hei, weight: "bold")[#label] #body
]

// ---------------------------------------------------------------- 题名、作者、摘要
#align(center)[
  #v(0.1in)
  #text(font: hei, size: 20pt, weight: "bold")[#title]
  #v(4pt)
  #text(font: hei, size: 14pt)[#subtitle]
  #v(10pt)
  #text(font: kai, size: 12pt)[#author]
  #v(2pt)
  #text(font: (latin, ..song), size: 8.5pt)[
    Yata 品质模型 v2（`yata-quality-v2`）；形式化：`KCN-judu/yata` 的 `formal/`（Lean 4.34.0，Mathlib v4.34.0）；论文修订于 2026-09-25
  ]
  #v(8pt)
]

#block(inset: (x: 1.5em))[
  #set text(size: 9pt, font: (latin, ..kai))
  #set par(first-line-indent: 0em, leading: 0.7em)
  #field("摘　要：")[
    《阴阳师》的御魂由一个随机过程产生：它从哪些副属性开始、每次强化落在哪个属性上、每次增量落在什么区间内，这些都由数值策划规定，并在很大程度上公开发布。一个御魂品质分数是否可信，取决于它与这一过程的关系。本文提出 Yata 品质模型，其组织原则只有一条——#strong[先机制，后偏好]：标准先把策划的生成过程重建为概率测度，每个概率都取自公布的规则、不对数据作任何拟合；然后才施加少量声明式调整，使数字符合玩家解读御魂的方式。重建是精确的：落在某组属性上的增量次数的分布律以有理数传播计算；策划唯一未公布的量——一次增量在其区间内的分布——只通过其均值进入分数，而三个独立来源约束了这个均值。调整包括：以#emph[强化单位]衡量数值；以公开的#emph[评分原型]目录评判御魂——输出、命中、治疗，以及仅当速度达到四个强化单位时才参与的单属性速度原型；以各原型自身的零点、期望御魂与可达上限作锚点；并以明确的效用里程碑划分 N 至 UR 档位。原型不必大小相同：可比性来自各原型自身的归一化。我们在 Lean 4 中证明：任何调整都不会把稀有度带回分数；分数有界、单调并保持占优；治疗原型是输出原型在公布规则所尊重的一个重标记下的像；档位判定确定且具有所述优先级；仅一条完美副属性恰为 SSR；每个 UR 御魂高于其原型中的其他所有御魂；速度原型的门槛与档位名副其实。校准程序重算本文的每个数字，一旦公布的数字漂移，构建即告失败。
  ]
  #field("关键词：")[游戏机制；概率建模；评分标准；归一化；精确计算；机械化证明；Lean 4]
  #field("中图法分类号：")[TP301]
]

#v(10pt)
#align(center)[
  #text(font: latin, size: 13pt, weight: "bold")[#title-en]
  #v(6pt)
  #text(font: latin, size: 10.5pt)[#upper(author)]
  #v(6pt)
]
#block(inset: (x: 1.5em))[
  #set text(size: 9pt, font: latin)
  #set par(first-line-indent: 0em, leading: 0.65em)
  #par[#text(weight: "bold")[Abstract:] A soul in _Onmyoji_ is produced by a random process that the game's numerical designers specified and, in large part, published. This paper presents the Yata Quality Model, a scoring standard organized around one principle, mechanics before preference: it first reconstructs the designers' generative process exactly as a probability measure, taking every probability from a published rule and fitting nothing to data, and only then applies a few declared adjustments that make the numbers read as players read souls — roll units; a catalogue of role archetypes of different sizes, output, hit, healing, and a one-attribute speed archetype considered only above four Speed roll units; three anchors per archetype; and tiers at utility milestones. The one unpublished quantity enters every score only through its mean. We prove in Lean 4 that no adjustment feeds rarity back into the score, that the score is bounded, monotone and dominance-preserving, that healing is output under a relabelling the rules respect, that tiers are deterministic, that one perfect line is exactly SSR, that UR is above every other soul of its archetype, and that the speed archetype's gate and tiers mean what they say. A calibration program recomputes every number and fails the build when one drifts.]
  #par[#text(weight: "bold")[Key words:] game mechanics; probabilistic modelling; scoring standards; normalization; exact computation; mechanized proof; Lean 4]
]
#v(6pt)
#line(length: 100%, stroke: 0.4pt)

#include "body.zh.typ"

#set par(first-line-indent: 0em)
#bibliography("references.bib", style: "gb-7714-2015-numeric", title: "参考文献")
