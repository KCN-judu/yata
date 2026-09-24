// 中文论文的定理、证明与图题环境，供各 main.zh.typ 与 body.zh.typ 使用。
// 版式参照国内计算机与数学期刊：定理名用黑体，正文用楷体，证明以"证明."起、以"□"止。
#let thm(kind, num, name, body) = block(
  above: 1.0em, below: 1.0em, width: 100%, breakable: true,
)[
  #set par(first-line-indent: 0em)
  #text(font: ("Source Han Sans SC", "SimHei"), weight: "bold")[#kind#if num != "" [ #num]]
  #if name != [] [（#name）] else []
  #h(0.5em)
  #text(font: ("LXGW WenKai", "KaiTi"))[#body]
]
#let proof(body) = block(above: 0.5em, below: 1.0em, width: 100%, breakable: true)[
  #set par(first-line-indent: 0em)
  #text(font: ("Source Han Sans SC", "SimHei"))[证明.] #body #h(1fr) $square$
]
#let figcaption(body) = block(above: 0.4em, below: 1.2em, width: 100%)[
  #set par(first-line-indent: 0em)
  #set text(size: 9pt)
  #body
]
