#import "../template/env.typ": *

#heading(level: 1, numbering: none)[Abstract]
<abstract>

A soul (御魂) in #emph[Onmyoji] is produced by a random process that the
game's numerical designers specified and, in large part, published:
which sub-attributes a soul starts with, which attribute each
enhancement strengthens, and within which range each increment falls. A
quality score for souls is only as trustworthy as its relation to that
process. This paper presents the Yata Quality Model, a scoring standard
organized around one principle, #strong[mechanics before preference]:
the standard first reconstructs the designers' generative process as a
probability measure, taking every probability from a published rule and
fitting nothing to data, and only then applies a small number of
declared adjustments that make the numbers read the way players read
souls. The reconstruction is exact: the law of how many increments reach
an attribute set is computed by rational propagation, and the one
quantity the designers did not publish --- the distribution of an
increment within its range --- enters every score only through its mean,
which three independent sources constrain. The adjustments measure value
in #emph[roll units]\; judge a soul against a published catalogue of
role #emph[archetypes] --- output, hit, healing, and a one-attribute
speed archetype that is considered only once a soul's Speed reaches four
roll units; anchor each archetype's score at zero, at its own expected
soul and at its own attainable maximum; and cut tiers N to UR at stated
utility milestones. Archetypes need not have the same size:
comparability comes from each archetype's own normalization. We prove in
Lean 4 that none of the adjustments feeds rarity back into the score,
that the score is bounded, monotone and dominance-preserving, that the
healing archetype is the output archetype under a relabelling the
published rules respect, that tier assignment is deterministic with the
stated precedence, that one perfect line alone is exactly SSR, that
every UR soul outscores every other soul of its archetype, and that the
speed archetype's gate and tiers mean what they say. A calibration
program recomputes every number in this paper and fails the build when a
published number drifts.

#strong[Keywords:] game mechanics, probabilistic modelling, scoring
standards, normalization, exact computation, mechanized proof, Lean 4

= Introduction
<introduction>

A player looking at a soul asks two questions: #emph[is this soul good?]
and #emph[is it good for this Shikigami?] Yata answers them separately.
This paper is about the first, the soul's quality with no holder in
mind, and about a property that a quality score has to earn before any
of its numbers deserve trust: that it measures the soul against the game
the designers made, not against a model the scorer invented.

That property is not automatic. A score is a function of a soul, and a
normalization is a function of a reference distribution; if the
reference distribution is fitted to a sample, or assumed for
convenience, or tuned until the tiers feel right, then the score
reflects those choices as much as the game. The soul system invites
this, because it has parts the designers published and a part they did
not. The published parts fix which attributes a soul can hold and with
what probabilities they are drawn and strengthened. The unpublished part
is how large an increment is within its published range.

We therefore separate the standard into two layers and keep them apart
throughout.

- #strong[\(L1) The mechanics layer] is the designers' process as a
  probability measure. Every probability in it is a published fraction
  or a published rule; nothing in it is estimated from observed souls;
  it is propagated exactly. Where the designers published nothing, it
  depends on as little as possible and says how much.
- #strong[\(L2) The reading layer] is the standard's own: how a soul's
  values become one number and one tier that a player can read. Every
  choice in it is a declared adjustment with a stated reason, and none
  of them alters the measure of L1.

The requirements on the standard follow from this split, and we label
them so that the rest of the paper can name the requirement each
mechanism answers.

- #strong[\(R1) Faithful.] Probabilities come from the published rules,
  and the reference quantities are computed exactly from them.
- #strong[\(R2) Robust where the designers were silent.] A score depends
  on unpublished quantities only through what three independent readings
  constrain.
- #strong[\(R3) No reward for rarity.] A soul is not scored higher
  because the outcome it shows is improbable.
- #strong[\(R4) A common scale.] Souls judged against different roles
  are placed on one 0--100 scale without favouring the role whose
  attributes are drawn less often, whatever the number of attributes the
  role values.
- #strong[\(R5) Tiers mean what their words say.] The player-facing name
  of a tier and its formal predicate coincide, and the tier order cannot
  be contradicted by the score order where the standard promises it
  cannot.

== Contributions
<contributions>

+ #strong[The object of analysis] (§2). We fix exactly which part of the
  game the standard models --- the six-star enhancement system --- and
  which sources each rule rests on, under a stated precedence: an
  official publication first, then the newest analysis, then larger
  samples.
+ #strong[A faithful reconstruction] (§3, R1--R2). The generative
  process of the published rules is a finite probability measure over
  hit vectors, computed exactly in rational arithmetic. Proposition 1
  shows that expected utility depends on the unpublished increment law
  only through its mean; the reconstruction reproduces an exact symmetry
  check, the analysis's theoretical best soul, and a Monte Carlo
  simulation within two and a half standard errors.
+ #strong[Declared adjustments for players] (§4--§7, R3--R4). Roll
  units; a catalogue of archetypes of different sizes; main-attribute
  gating and a soul-level eligibility predicate; three anchors per
  archetype; and utility-milestone tiers. Theorem 6 shows that
  probability enters a score only through the archetype's expected
  utility; Theorem 7 that archetypes the published rules treat alike
  share one scale exactly, and that `healing` and `output` are such a
  pair; the residual between the others is measured.
+ #strong[Tier semantics] (§6--§7, R5). Theorems 8--13 prove that the
  four-attribute ladder is deterministic with precedence UR, SP, band;
  that SP cannot bypass its floor; that one perfect line alone is
  exactly SSR; that UR is above every other soul of its archetype in
  score and in the exposed order; that dominance never lowers a tier;
  and that the v1.1 revision of the SSR edge only raised tiers.
  Proposition 14 and Theorems 15--16 do the same for the one-attribute
  `speed` archetype: which souls it may judge, its scale, and its tier
  rule.
+ #strong[Calibration and reproducibility] (§8--§9). A calibration
  program computes every number here --- anchors, thresholds, tier rates
  under three increment laws and two readings of the weights, the
  eligibility rate of `speed`, a Monte Carlo check, and thirty golden
  test vectors --- and a repository check fails when a committed number
  drifts.

Every theorem is a Lean 4 declaration in `formal/lean/` of
`KCN-judu/yata` #cite(<moura2021lean>)#cite(<mathlib2020>), cited by its Lean name where
it is stated and indexed in Appendix A; Proposition 1 and the statements
marked #emph[computation] are established by the paper's argument and by
the calibration program, not in Lean. Two conventions bound every claim.
A theorem proves consequences of the stated definitions; that the game
follows the stated rules rests on the sources of §2, not on Lean. And
the standard makes no claim about what players prefer: its adjustments
are designed to match how players read souls, and whether they do is a
question no theorem answers.

This paper explains `yata-quality-v2`. Version 1.1 moved the SSR edge
(§6.4); version 2 replaced the catalogue (§4.2, §7).

= The object of analysis
<the-object-of-analysis>

== What is modelled
<what-is-modelled>

The object is the #strong[six-star soul at +15] and the random process
that produces its sub-attributes. A soul has a slot (1--6), a main
attribute from that slot's pool, a level, and up to four sub-attributes
drawn from eleven: `AtkFlat`, `AtkPercent`, `DefFlat`, `DefPercent`,
`HpFlat`, `HpPercent`, `Spd`, `EffectHit`, `EffectRes`, `Crit`,
`CritDmg`. A sub-attribute's value is the sum of its #emph[increments]:
the initial one, and one for each enhancement that strengthens it.

The following are outside the object, and the standard says nothing
about them: souls below six stars, whose increment ranges are not
published; set effects, which belong to the second, need-dependent pass
of Yata's scoring; the innate attribute of boss souls; and how often
each slot or main attribute is dropped by a given activity, which the
designers disclose only for four shop sources.

== The published rules
<the-published-rules>

The rules are those of Yata's mechanics specification, each with its
source. Table 1 lists the ones the standard uses.

#figure(
  align(center)[#table(
    columns: 3,
    align: (auto,auto,auto,),
    table.header([Rule], [Source], [Status],),
    table.hline(),
    [a soul starts with 2, 3 or 4 sub-attributes, about one third
    each], [community data-mining cited by #cite(<hu2026soul>)], [secondary; the
    cited forum requires a login],
    [at +3, +6, +9, +12 and +15, one enhancement], [#cite(<hu2026soul>)\;
    official guides], [primary],
    [with four sub-attributes, each is strengthened with probability
    1/4], [#cite(<netease2017notice>)], [official],
    [below four, one attribute is drawn from all eleven; a new one is
    added, an existing one strengthened], [#cite(<netease2017notice>)\;
    #cite(<hu2026soul>)], [official weights; the strengthening reading from the
    analysis],
    [class weights 攻击类 36%, 防御类 36%, 功能类 28%, shared equally
    within a class], [#cite(<netease2017notice>)\; the equal share
    #cite(<hu2026soul>)], [official],
    [one increment of attribute $a$ lies in $\[0.8 thin h_a\,h_a\]$,
    $h_a$ its largest increment], [#cite(<hu2026soul>), Table 4], [primary for
    six stars],
  )]
  , kind: table
  )

#emph[Table 1. The rules of the six-star enhancement process and their
sources.]

#strong[Precedence of sources.] Where sources disagree on data, an
official publication is authoritative wherever it gives the number;
otherwise the newest analysis prevails; among sources of one kind, the
larger sample. The rule decides one visible case. The analysis
#cite(<hu2026soul>) reads the notice's class figures 36/36/28 as the rounding of
one eleventh per attribute, $4\/11$, $4\/11$ and $3\/11$\; but
$3\/11 = 27.27 %$ does not round to 28%, and the notice is official. The
standard therefore uses the notice's figures, shared equally within a
class, and reports the analysis's reading as a sensitivity (§8).

#strong[What the designers did not publish.] Three things. The
distribution of an increment within its range: the official site gives
only averages #cite(<liulang2017guide>), and a 2018 test of two to three hundred
souls #cite(<wangzhe2018growth>) states figures that neither a continuous
uniform law nor a uniform law on steps of 0.1 reproduces. Whether
initial sub-attributes may repeat: the count "2, 3 or 4" presupposes
that they do not. And the per-slot drop distribution, outside the
object.

= A faithful reconstruction of the designers' process
<a-faithful-reconstruction-of-the-designers-process>

== The generative model
<the-generative-model>

Let $A$ be the eleven attributes and $w : A arrow.r bb(Q)$ the draw
weights: $w\(a\)= 9\/100$ for the four 攻击类 and the four 防御类
attributes, and $w\(a\)= 28\/300$ for the three 功能类 attributes. A +15
soul's sub-attributes are generated as follows.

$ N_0 tilde.op upright("Uniform") { 2\,3\,4 }\
I = N_0 upright(" distinct attributes, drawn one by one by ") w upright(", without replacement")\
upright("for ") t = 1\,dots.h\,5 :\
quad upright("fewer than four present: ") X_t tilde.op w upright(" on ") A upright(", added or strengthened")\
quad upright("four present: ") X_t tilde.op upright("Uniform")\(upright("present")\)upright(", strengthened")\
upright("an increment of ") a upright(" is ") h_a R\,upright(" with ") R upright(" i.i.d. on ")\[4\/5\,1\]upright(", mean ") mu $

Three sources of randomness are kept apart: #emph[which attributes
exist] ($N_0$, $I$ and the adding enhancements), #emph[which attribute
each enhancement strengthens] ($X_t$), and #emph[how large each
increment is] ($R$). The first two are discrete and published; the third
is continuous and not.

A soul's value on $a$ in #strong[roll units] is $e_a = S\(a\)\/h_a$, the
sum of the $R$s of $a$'s increments, and $e_a = 0$ when $a$ is absent.
For a set $P$ of attributes, let $K_P$ be the number of increments that
land on $P$.

#thm("Definition", "")[The hit law][The #emph[hit law] is the
distribution of the vector of increment counts per attribute at +15. It
is a finite distribution with rational probabilities.]

#emph[Computation (Exact propagation).] The calibration program computes
the hit law by propagating the process above state by state in exact
rational arithmetic: a state is the vector of counts, the initial draws
are weighted without replacement, and each enhancement is one
transition. No simulation and no rounding enters any reference quantity.
Table 2 gives the law of $K_P$ for the archetypes of §4.

#figure(
  align(center)[#table(
    columns: 4,
    align: (auto,auto,auto,auto,),
    table.header([K], [P(K), output and healing], [P(K), hit], [P(K),
      speed],),
    table.hline(),
    [0], [10.6772%], [10.4176%], [62.8839%],
    [1], [13.8563%], [13.7302%], [12.0758%],
    [2], [20.2318%], [20.1178%], [15.2167%],
    [3], [18.6657%], [18.6903%], [7.6708%],
    [4], [16.1611%], [16.2911%], [1.9056%],
    [5], [11.0711%], [11.2136%], [0.2349%],
    [6], [5.9451%], [6.0585%], [0.0122%],
    [7], [2.5199%], [2.5828%], [0],
    [8], [0.7711%], [0.7932%], [0],
    [9], [0.1006%], [0.1050%], [0],
    [E\[K\]], [2.907009880], [2.929901204], [0.743920963],
  )]
  , kind: table
  )

#emph[Table 2. The law of the number of useful increments, exact under
the official weights.]

== The designers were silent on one thing, and it barely matters
<the-designers-were-silent-on-one-thing-and-it-barely-matters>

Given the hit law, the utility of a set $P$ is a sum of $K_P$
increments, $U_P = R_1 + dots.h + R_(K_P)$. The standard needs the
expected utility of each archetype, and nothing else from the
distribution of $R$.

#thm("Proposition", "1")[Mean-only dependence; paper proof][For every
attribute set $P$, $bb(E)\[U_P\]= mu thin bb(E)\[K_P\]$, whatever the
law of $R$ on $\[4\/5\,1\]$ with mean $mu$.]

#proof[Condition on the hit law. Given $K_P = k$, $U_P$ is a sum
of $k$ increments, each with mean $mu$ and independent of which
attribute received it, so $bb(E)\[U_P divides K_P = k\]= k mu$.
Averaging over the hit law gives $bb(E)\[U_P\]= mu thin bb(E)\[K_P\]$.]

Every score in the standard is determined by a soul's own utility and by
$bb(E)\[U_P\]$ (§5), so by Proposition 1 the unpublished law enters a
score only through $mu$. Three independent readings constrain $mu$: the
midpoint of the published range, $0.9$\; the official site's averages,
$2.72\/3.0 approx 0.907$ and $3.65\/4.0 approx 0.913$
#cite(<liulang2017guide>)\; and the 2018 test, whose figures lie between the
uniform and the stepped laws #cite(<wangzhe2018growth>). The standard takes
$mu = 9\/10$. Its tier #emph[rates], which do depend on the full law,
are reported under three laws with this mean (§8).

== Why the reconstruction is faithful
<why-the-reconstruction-is-faithful>

Faithfulness here is a list of checkable properties, not a claim about
the designers' intent.

+ #strong[Every probability is published.] The weights, the $1\/4$ rule
  and the enhancement schedule come from the notice; the ranges from the
  analysis. The initial count is the one secondary figure, and it is
  reported as such.
+ #strong[Nothing is fitted.] No parameter is estimated from observed
  souls, and the tiers are not tuned to a target rate (§6.4).
+ #strong[The propagation is exact.] Every probability of Table 2 is a
  rational number, printed to four decimals.
+ #strong[The unpublished part is isolated.] By Proposition 1 it enters
  through one number, and the effect of that number is reported: over
  the whole published range, $mu in\[0.8\,1\]$, no test vector's score
  moves by more than one point from its value at $mu = 0.9$ (Table 9).
+ #strong[It reproduces what it should.] Under the analysis's
  one-eleventh reading the process is invariant under every permutation
  of the attributes, so four attributes expect exactly
  $8 dot.op 4\/11 = 32\/11$ useful increments and one attribute $8\/11$,
  $8$ being $bb(E)\[N_0\]+ 5$\; the program asserts both of its
  propagation. The analysis's theoretical best soul #cite(<hu2026soul>) --- 18
  Crit, 4 CritDmg, 3 AtkPercent and 3 Spd --- reaches exactly the
  attainable maximum (Proposition 2). An independent Monte Carlo
  simulation of four million souls agrees with every exact tier rate
  (§8).

= Utility: reading a soul as players do
<utility-reading-a-soul-as-players-do>

The reading layer begins here. Each subsection states an adjustment, the
player intuition it follows, and what it cannot do.

== Adjustment 1: roll units
<adjustment-1-roll-units>

Players count lines and rolls: a line that took all five enhancements is
拉满, and 3 Crit and 114 flat HP are both one good increment. The
standard measures a sub-attribute in roll units, $e_a = S\(a\)\/h_a$, so
that one maximal increment of any attribute is worth $1$. Without this,
a weighted sum of raw values measures how much flat HP a soul has.

== Adjustment 2: a catalogue of role archetypes
<adjustment-2-a-catalogue-of-role-archetypes>

Players judge a soul by what it is for. The standard judges it against a
small published catalogue of #strong[archetypes], each a set $A_p$ of
useful sub-attributes with weight 1, a set of main attributes it accepts
per slot, and possibly an eligibility predicate on the soul (§7).

#figure(
  align(center)[#table(
    columns: 6,
    align: (auto,auto,auto,auto,auto,auto,),
    table.header([Archetype], [Useful sub-attributes], [Eligible
      when], [Slot 2 mains], [Slot 4 mains], [Slot 6 mains],),
    table.hline(),
    [`output` 输出], [`AtkPercent` `Crit` `CritDmg`
    `Spd`], [always], [`Spd` `AtkPercent`], [`AtkPercent`], [`Crit`
    `CritDmg` `AtkPercent`],
    [`hit` 命中], [`EffectHit` `Spd` `HpPercent`
    `DefPercent`], [always], [`Spd` `HpPercent`
    `DefPercent`], [`EffectHit` `HpPercent` `DefPercent`], [`HpPercent`
    `DefPercent`],
    [`healing` 治疗], [`HpPercent` `Crit` `CritDmg`
    `Spd`], [always], [`Spd` `HpPercent`], [`HpPercent`], [`Crit`
    `CritDmg` `HpPercent`],
    [`speed` 速度], [`Spd`], [$e_(italic("Spd")) gt.eq 4$], [`Spd`
    `HpPercent` `DefPercent`], [`EffectHit` `EffectRes` `HpPercent`
    `DefPercent`], [`HpPercent` `DefPercent`],
  )]
  , kind: table
  )

#emph[Table 3. The archetypes of v2. Slots 1, 3 and 5 have one fixed
main attribute, which every archetype accepts.]

`output` evaluates general offensive quality; its set is the analysis's
four core output attributes #cite(<hu2026soul>). `hit` evaluates Effect-Hit and
control-oriented quality. `healing` evaluates the pattern of HP-based
healers that can crit; it is `output` with `AtkPercent` and `HpPercent`
exchanged, and a general pattern rather than a prescription for any
Shikigami. `speed` evaluates exceptional pure-Speed quality. The last
three are authored, and marked so. The archetypes are not exclusive: a
soul may be judged by several, and its quality is the best result
(§6.5). A soul's #strong[utility] under archetype $p$ is

$ U_p\(s\)= sum_(a in A_p) e_a\(s\). $

#strong[Cross-archetype comparability is provided by archetype-specific
normalization, not by forcing every archetype to contain the same number
of equally useful attributes.] Version 1 gave every archetype four
attributes, and its third archetype, `resist`,
${ italic("Spd")\,italic("EffectRes")\,italic("HpPercent")\,italic("DefPercent") }$,
was chosen partly because it mirrored `hit` one class member apart,
which made its scale provably equal to `hit`'s. Version 2 removes it. A
resist-oriented build is a need of particular Shikigami and sets, which
the need-dependent pass judges; EffectRes is useful to no archetype of
the quality pass. `healing` is not given `DefPercent` to keep the old
shape, and `speed` is not given filler attributes to reach four. Each
archetype carries its own maximum and its own expected soul (§5), and
those make the scales comparable.

== Adjustment 3: the main attribute gates, it does not score
<adjustment-3-the-main-attribute-gates-it-does-not-score>

Players know that a speed main belongs on slot 2, and that a rare main
does not make a good soul by itself. The main attribute decides which
archetypes may judge a soul and adds nothing to its utility. A main is
worth up to 22 roll units against the sub-attributes' 9 #cite(<hu2026soul>)\;
counting it would let the main decide every score, by its magnitude
rather than its use. In particular a Speed main adds nothing to
`speed`'s utility: `speed` reads the Speed sub-attribute alone.

== The attainable maximum, and two structural milestones
<the-attainable-maximum-and-two-structural-milestones>

#thm("Proposition", "2")[The attainable maximum;
`Archetype.utility_le_nine`, `paperSoul_utility`, `paperSoul_score`][For every legal soul and four-attribute archetype,
$0 lt.eq U_p\(s\)lt.eq 9$\; the analysis's theoretical output soul has
$U_(italic("output")) = 9$ and scores $100$.]

#proof[A line's value is at most its increment count, each
increment being at most one roll unit (`Soul.features_le_hits`); a legal
soul has at most nine increments, so
$U_p lt.eq sum_(a in A_p) italic("hits")\(a\)lt.eq 9$. The paper's soul
has six maximal increments on Crit and one on each of the other three
output attributes, $6 + 1 + 1 + 1 = 9$, and $g\(M\)= 100$ (Theorem 4).]

#thm("Proposition", "3")[Six and eight; `Soul.six_hits_of_gt_five`,
`Archetype.single_line_le_six`, `beyond_six_needs_another_line`,
`nine_useful_of_gt_eight`][(i) A line above five roll units has taken
all five enhancements. (ii) A soul whose useful value lies on one line
has $U_p lt.eq 6$\; a soul with $U_p > 6$ has, for every line, another
useful line with value. (iii) A soul with $U_p > 8$ has all nine
increments on useful lines.]

#proof[(i) $e_a lt.eq italic("hits")\(a\)lt.eq 6$ and $e_a > 5$
force $italic("hits")\(a\)= 6$. (ii) One line holds at most six
increments of at most one roll unit; if every other useful line were
zero, $U_p$ would be that line alone, at most $6$. (iii)
$8 < U_p lt.eq sum_(a in A_p) italic("hits")\(a\)lt.eq 9$ forces the
useful increment count to be $9$.]

Six is therefore the boundary between the best one line can be and
quality that needs several lines, and eight the boundary beyond which
only the sizes of the increments separate a four-attribute soul from the
maximum. §6 builds on both; §7 shows what remains of them for a
one-attribute archetype.

= Normalization: a common scale without rewarding rarity
<normalization-a-common-scale-without-rewarding-rarity>

== Adjustment 4: three anchors per archetype
<adjustment-4-three-anchors-per-archetype>

A player reads a number against what is usual: #emph[is this better than
the souls I normally get?] The standard maps utility to 0--100 by three
anchors per archetype, $0$ for no useful value, $50$ for the archetype's
expected +15 soul, and $100$ for the archetype's attainable maximum:

$ g_p\(u\)= cases(delim: "{", max (0 \, med 50 thin u \/ E_p) & u lt.eq E_p\,, min (100 \, med 50 + 50 thin \( u - E_p \) \/ \( M_p - E_p \)) & u > E_p\,) $

with $E_p = mu thin bb(E)\[K_(A_p)\]$, $M_p = 9$ for the four-attribute
archetypes, and $M_(italic("speed")) = 6$. The score of a soul under
$p$ is $g_p\(U_p\(s\)\)$. The anchors are exact rationals; Table 4 gives
them.

Parameter set `yata-quality-v2`. Reference measure: the official class
weights, equal within a class. Exact values:
`formal/calibration/out/constants.json`.

#figure(
  align(center)[#table(
    columns: 3,
    align: (auto,auto,auto,),
    table.header([Milestone], [Roll units], [Meaning],),
    table.hline(),
    [μ], [9/10], [mean increment, roll units],
    [M], [9], [attainable maximum of a four-attribute archetype],
    [u\_SR], [9/2], [SR: five useful increments at mean value, 5μ],
    [u\_SSR], [6], [SSR: the most one useful line can hold, six
    increments at maximum],
    [u\_SP\_floor], [63/10], [SP's quality floor: seven useful
    increments at mean value, 7μ],
    [u\_UR], [8], [UR: above M − 1],
    [M\_speed], [6], [attainable maximum of speed: one line, six
    increments at maximum],
    [u\_gate], [4], [speed is a candidate only when e\_Spd ≥ 4],
    [u\_UR,speed], [5], [speed's UR: above M\_speed − 1],
  )]
  , kind: table
  )

#figure(
  align(center)[#table(
    columns: 9,
    align: (auto,auto,auto,auto,auto,auto,auto,auto,auto,),
    table.header([Archetype], [E\[K\]], [E = μ ·
      E\[K\]], [M], [c\_R], [c\_SR], [c\_SSR = g(6)], [c\_SP], [t\_UR],),
    table.hline(),
    [output], [2.907009880], [2.616308892], [9], [50], [64.753934], [76.502622], [78.852360], [92.167541],
    [hit], [2.929901204], [2.636911083], [9], [50], [64.639815], [76.426543], [78.783889], [92.142181],
    [healing], [2.907009880], [2.616308892], [9], [50], [64.753934], [76.502622], [78.852360], [92.167541],
  )]
  , kind: table
  )

#figure(
  align(center)[#table(
    columns: 6,
    align: (auto,auto,auto,auto,auto,auto,),
    table.header([Archetype], [E\[K\]], [E = μ · E\[K\]], [M], [score at
      the gate, g(4)], [t\_UR = g(5)],),
    table.hline(),
    [speed], [0.743920963], [0.669528867], [6], [81.239932], [90.619966],
  )]
  , kind: table
  )

#emph[Table 4. The milestones in roll units, and each archetype's
anchors and score thresholds.]

#thm("Theorem", "4")[Range and anchors; `Anchors.g_nonneg`, `g_le_100`,
`g_zero`, `g_E`, `g_M`][For any anchors $0 < E < M$,
$0 lt.eq g\(u\)lt.eq 100$ for all $u$, and $g\(0\)= 0$, $g\(E\)= 50$,
$g\(M\)= 100$.]

#proof[Below $E$ the score is $max\(0\,50 u\/E\)$, which is at
least $0$ and, for $u lt.eq E$, at most $50$\; above $E$ it is the
minimum of $100$ and a value at least $50$. The three values are direct
computations.]

#thm("Theorem", "5")[Monotonicity; `Anchors.g_mono`, `g_strict`,
`g_scale`, `Profile.score_mono`, `score_strict`][$g$ is nondecreasing,
strictly increasing on $\[0\,M\]$, and invariant when the utility and
both anchors are scaled together. A soul no worse on every useful
attribute scores no lower, and one strictly better on some useful
attribute scores strictly higher while within the maximum.]

#proof[Each segment is affine with positive slope, and the lower
segment ends at $50$ where the upper begins, so $g$ is nondecreasing
across the join; the clamps preserve this, and on $\[0\,M\]$ neither is
active, which gives strictness. Scaling multiplies the numerator and the
denominator of each segment alike. Utility is a sum with nonnegative
weights, positive on useful attributes, so improving useful attributes
weakly raises it, and strictly when one improves; compose with $g$.]

== No reward for rarity
<no-reward-for-rarity>

#thm("Theorem", "6")[No rarity term; `Profile.score_factors`][Two souls
with equal utility under an archetype have equal scores under it,
however probable or improbable either soul is.]

#proof[The score is $g_p$ applied to the utility, and $g_p$
depends on the archetype, not on the soul.]

The theorem is short because the design is: probability enters a score
only through $E_p$, a property of the whole archetype, never through how
often a particular soul occurs. That is also how a rarer role is kept
from gaining. An archetype whose attributes are drawn less often has a
lower $E_p$, and a soul is placed by its distance above that
expectation, not by its raw utility. The rejected alternative, a bonus
for improbable outcomes, is exactly what Theorem 6 rules out (§10).

== A common scale across archetypes
<a-common-scale-across-archetypes>

#thm("Theorem", "7")[Relabelling fairness; `expected_relabel`,
`score_relabel`, `healing_is_output_relabelled`,
`healing_expected_eq_output`][Let the reference measure be invariant
under a permutation $pi$ of the attributes. Then an archetype and its
image under $pi$ have the same expected utility, and with the same
maximum, each scores the $pi$-image of a soul exactly as the other
scores the soul. The archetype `healing` is the image of `output` under
the exchange of `AtkPercent` and `HpPercent`, so under a measure
invariant under that exchange the two have the same expected utility.]

#proof[Utility is a sum over attributes, so relabelling both the
profile and the features by $pi$ leaves it unchanged; reindex the sum by
$pi^(- 1)$. Invariance of the measure gives a bijection of outcomes that
preserves probabilities and relabels features; reindexing the
expectation along it shows the two expected utilities equal. The
anchors, hence the score functions, are then the same. The image of an
equal-weight archetype is the equal-weight archetype of the image set,
and the exchange maps
${ italic("AtkPercent")\,italic("Crit")\,italic("CritDmg")\,italic("Spd") }$
onto
${ italic("HpPercent")\,italic("Crit")\,italic("CritDmg")\,italic("Spd") }$.]

The official weights are invariant under every permutation that keeps
attributes in their class. `AtkPercent` is 攻击类 and `HpPercent`
防御类, but both weigh 9%, so the exchange preserves the weights, and
`healing` and `output` have identical scales; the calibration asserts
that their anchors and every tier rate agree. `hit` holds two 功能类
attributes where `output` holds one, so no weight-preserving permutation
relates them. Its $E_p$ is under one percent higher, and the residual in
tier rates is reported in §8. It is small, and it has the right sign:
`hit`'s attributes are drawn slightly more often, so the same utility
lies slightly less far above its expectation. Version 1's `resist` was
`hit`'s exact partner; with it removed, `hit` has none, and the standard
does not claim one.

= Tiers on the four-attribute ladder
<tiers-on-the-four-attribute-ladder>

== Adjustment 5: tiers at utility milestones
<adjustment-5-tiers-at-utility-milestones>

Players read souls on the game's own rarity ladder, N \< R \< SR \< SSR
\< SP \< UR. For the four-attribute archetypes the standard borrows the
names and defines each tier by a milestone in roll units, the same for
every such archetype, with R beginning at the archetype's expected soul
$E_p$:

$ u_(upright("SR")) = 5 mu = 4.5 & upright("five useful increments at mean value")\
u_(upright("SSR")) = 6 & upright("the most one useful line can hold")\
u_(upright("SP")) = 7 mu = 6.3 & upright("SP’s quality floor: seven useful increments at mean value")\
u_(upright("UR")) = M - 1 = 8 & upright("within one roll unit of the maximum") $

Each milestone $u$ becomes the score threshold $g_p\(u\)$ (Table 4).
With $italic("spec")_p\(s\)$ true when some useful line holds more than
five roll units, the tier is UR when the score exceeds
$g_p\(u_(upright("UR"))\)$\; otherwise SP when $italic("spec")_p\(s\)$
holds and the score is at least $g_p\(u_(upright("SP"))\)$\; otherwise
SSR, SR, R or N as the score reaches $g_p\(u_(upright("SSR"))\)$,
$g_p\(u_(upright("SR"))\)$ or $50$.

In players' words, which match the predicates exactly: #strong[N] is
below the average soul of its role, #strong[R] at least average, and
#strong[SR] about five useful rolls' worth. #strong[SSR] begins where
one useful sub-attribute could, in theory, have received all six
increments at their maximum; going beyond it needs useful value outside
that line. #strong[SP] requires both an extreme specialized line and a
higher total quality, so one spectacular line alone remains SSR.
#strong[UR] is a soul on or next to the best the game allows. These are
Yata's scoring decisions, not rarity rules of the game.

#thm("Theorem", "8")[The tier function; `milestones_ordered`,
`Thresholds.tier_cases`, `tier_UR_iff`, `tier_SP_iff`, `band_ne_SP`,
`band_ne_UR`][The milestones satisfy
$u_(upright("SR")) < u_(upright("SSR")) < u_(upright("SP")) < u_(upright("UR"))$.
For thresholds
$c_(upright(R)) < c_(upright("SR")) < c_(upright("SSR")) < c_(upright("SP")) < t_(upright("UR")) < 100$
the tier is a function of the score and of specialization, and exactly
one of three cases holds: UR iff the score exceeds $t_(upright("UR"))$\;
SP iff not UR, specialized and at least $c_(upright("SP"))$\; otherwise
the band. A band is never SP or UR.]

#proof[The ordering is arithmetic, $4.5 < 6 < 6.3 < 8$. The tier
is a nested conditional on the score and a boolean; a case analysis of
the conditional gives exactly one branch, and the band's conditional
returns only N, R, SR or SSR.]

== Adjustment 6: SSR at one perfect line, SP with a floor of its own
<adjustment-6-ssr-at-one-perfect-line-sp-with-a-floor-of-its-own>

#thm("Theorem", "9")[SP needs its floor; `Thresholds.sp_needs_floor`,
`sp_is_ssr_quality`, `single_line_not_SP`][Every SP soul is
specialized and at or above $c_(upright("SP"))$, hence above the SSR
threshold. If the SP floor lies above $g_p\(6\)$, a soul whose useful
value lies on one line is never SP.]

#proof[The SP branch is taken only when both conditions hold, and
$c_(upright("SSR")) < c_(upright("SP"))$. A soul with one useful line has
$U_p lt.eq 6$ (Proposition 3), so its score is at most $g_p\(6\)$, below
the floor.]

#thm("Theorem", "10")[One perfect line is SSR;
`perfect_line_at_least_SSR`, `perfect_single_line_is_SSR`][If the SSR
threshold is $g_p\(6\)$, a soul with six roll units on one useful line
ranks at least SSR. If moreover that line is its only useful value, the
SP floor is above $g_p\(6\)$ and the UR boundary is $g_p\(8\)$, the soul
is exactly SSR.]

#proof[The line is at most the whole utility
(`Archetype.line_le_utility`), so $U_p gt.eq 6$ and the score is at
least the SSR threshold, where only SSR, SP and UR occur
(`Thresholds.at_ssr_floor_high_tier`). Theorem 9 excludes SP. Since
$U_p lt.eq 6 < 8$, UR is excluded as well: a soul is UR exactly when
$U_p > 8$ (Theorem 11).]

Test vector V04 is this soul, 18 Crit and three useless lines. It is SSR
and not SP.

== UR above everything
<ur-above-everything>

#thm("Theorem", "11")[UR; `UR_iff_gt_eight`, `UR_structure`,
`Thresholds.UR_score_gt`, `UR_top`][With $M = 9$ and
$t_(upright("UR")) = g_p\(8\)$, a soul is UR under $p$ exactly when
$U_p > 8$, and then all nine of its increments are useful. Every non-UR
soul scores strictly below every UR soul of the same archetype, and
every UR soul is strictly first in the exposed order, tier first and
then score.]

#proof[$g_p$ is strictly increasing on $\[0\,9\]$, where legal
utilities lie (Proposition 2), so $g_p\(U_p\)> g_p\(8\)$ iff $U_p > 8$\;
Proposition 3 gives the structure. A non-UR score is at most
$t_(upright("UR"))$ and a UR score exceeds it. A UR soul has rank 5 and
every other tier's rank is at most 4.]

== Monotonicity of tiers, and the v1.1 revision
<monotonicity-of-tiers-and-the-v1.1-revision>

#thm("Theorem", "12")[Dominance never lowers a tier;
`Thresholds.tier_mono`, `tier_mono_dominance`, `tierOn_mono_dominance`,
`exposed_mono_dominance`][A soul no worse on every useful attribute of
an archetype has, under it, a tier no lower, and at an equal tier a
score no lower. This holds for every archetype of the catalogue, `speed`
included.]

#proof[Its score is no lower (Theorem 5), and specialization is
preserved upward, since a line above five stays above five. The
four-attribute tier function is monotone in the score and in
specialization, by case analysis on the order of the thresholds;
`speed`'s rule is monotone in its one attribute (Theorem 16).]

The quality of a soul is the best (tier, score) pair over its candidate
archetypes. Each archetype's pair is monotone under dominance, and the
best of pointwise larger pairs is larger.

#thm("Theorem", "13")[Lowering SSR only raises tiers;
`lower_ssr_floor_never_lowers_tier`][Two threshold sets that agree
except that the second has a lower SSR threshold rank every soul at
least as high under the second.]

#proof[Case analysis: the UR and SP branches are unchanged; in
the band, a lower SSR threshold can only move a score from SR to SSR.]

Version 1 put the SSR edge at $7 mu = 6.3$, the same threshold as the SP
floor. That is consistent, but a soul with six maximum increments on one
useful line, the best one line can be, was SR. Version 1.1 moved the SSR
edge to six and kept the SP floor at $7 mu$. Scores did not change; by
Theorem 13 tiers changed only upward, and only for souls with
$6 lt.eq U_p < 6.3$. Four edges were compared, with the SP floor and the
UR boundary fixed:

SP's floor stays 7μ = 6.3 and UR's boundary 8 in every row; only SSR's
edge moves. The four-attribute archetypes of v2; speed has no SSR edge
of this kind.

#figure(
  align(center)[#table(
    columns: 11,
    align: (auto,auto,auto,auto,auto,auto,auto,auto,auto,auto,auto,),
    table.header([Candidate], [u\_SSR], [Increment
      law], [Archetype], [N], [R], [SR], [SSR], [SP], [UR], [SSR or
      better],),
    table.hline(),
    [A. v1: 7μ], [6.300], [continuous uniform on \[0.8, 1\]], [output,
    healing], [48.6960%], [36.4318%], [12.7406%], [2.0388%], [0.0209%], [0.0719%], [2.1316%],
    [A. v1: 7μ], [6.300], [continuous uniform on \[0.8,
    1\]], [hit], [49.3844%], [35.4693%], [12.9568%], [2.0933%], [0.0212%], [0.0751%], [2.1896%],
    [A. v1: 7μ], [6.300], [7 equal steps on \[0.8, 1\]], [output,
    healing], [49.3366%], [35.3133%], [13.1256%], [2.1364%], [0.0218%], [0.0663%], [2.2245%],
    [A. v1: 7μ], [6.300], [7 equal steps on \[0.8,
    1\]], [hit], [50.6409%], [33.7287%], [13.3456%], [2.1934%], [0.0221%], [0.0692%], [2.2847%],
    [A. v1: 7μ], [6.300], [2 equal steps on \[0.8, 1\]], [output,
    healing], [54.0982%], [31.0296%], [12.7406%], [2.0613%], [0.0200%], [0.0503%], [2.1316%],
    [A. v1: 7μ], [6.300], [2 equal steps on \[0.8,
    1\]], [hit], [53.6107%], [31.2430%], [12.9568%], [2.1167%], [0.0203%], [0.0525%], [2.1896%],
    [B. v1.1: one perfect line], [6.000], [continuous uniform on \[0.8,
    1\]], [output,
    healing], [48.6960%], [36.4318%], [11.5411%], [3.2383%], [0.0209%], [0.0719%], [3.3311%],
    [B. v1.1: one perfect line], [6.000], [continuous uniform on \[0.8,
    1\]], [hit], [49.3844%], [35.4693%], [11.7274%], [3.3227%], [0.0212%], [0.0751%], [3.4190%],
    [B. v1.1: one perfect line], [6.000], [7 equal steps on \[0.8,
    1\]], [output,
    healing], [49.3366%], [35.3133%], [12.0488%], [3.2133%], [0.0218%], [0.0663%], [3.3013%],
    [B. v1.1: one perfect line], [6.000], [7 equal steps on \[0.8,
    1\]], [hit], [50.6409%], [33.7287%], [12.2419%], [3.2972%], [0.0221%], [0.0692%], [3.3885%],
    [B. v1.1: one perfect line], [6.000], [2 equal steps on \[0.8,
    1\]], [output,
    healing], [54.0982%], [31.0296%], [11.5452%], [3.2567%], [0.0200%], [0.0503%], [3.3270%],
    [B. v1.1: one perfect line], [6.000], [2 equal steps on \[0.8,
    1\]], [hit], [53.6107%], [31.2430%], [11.7321%], [3.3414%], [0.0203%], [0.0525%], [3.4142%],
    [C. 6μ], [5.400], [continuous uniform on \[0.8, 1\]], [output,
    healing], [48.6960%], [36.4318%], [8.5081%], [6.2713%], [0.0209%], [0.0719%], [6.3641%],
    [C. 6μ], [5.400], [continuous uniform on \[0.8,
    1\]], [hit], [49.3844%], [35.4693%], [8.6361%], [6.4140%], [0.0212%], [0.0751%], [6.5103%],
    [C. 6μ], [5.400], [7 equal steps on \[0.8, 1\]], [output,
    healing], [49.3366%], [35.3133%], [8.7502%], [6.5118%], [0.0218%], [0.0663%], [6.5999%],
    [C. 6μ], [5.400], [7 equal steps on \[0.8,
    1\]], [hit], [50.6409%], [33.7287%], [8.8799%], [6.6592%], [0.0221%], [0.0692%], [6.7505%],
    [C. 6μ], [5.400], [2 equal steps on \[0.8, 1\]], [output,
    healing], [54.0982%], [31.0296%], [7.5792%], [7.2227%], [0.0200%], [0.0503%], [7.2930%],
    [C. 6μ], [5.400], [2 equal steps on \[0.8,
    1\]], [hit], [53.6107%], [31.2430%], [7.6894%], [7.3841%], [0.0203%], [0.0525%], [7.4569%],
    [D. SSR or better at 5% (uniform, output)], [5.488], [continuous
    uniform on \[0.8, 1\]], [output,
    healing], [48.6960%], [36.4318%], [9.8667%], [4.9127%], [0.0209%], [0.0719%], [5.0055%],
    [D. SSR or better at 5% (uniform, output)], [5.488], [continuous
    uniform on \[0.8,
    1\]], [hit], [49.3844%], [35.4693%], [10.0206%], [5.0294%], [0.0212%], [0.0751%], [5.1257%],
    [D. SSR or better at 5% (uniform, output)], [5.488], [7 equal steps
    on \[0.8, 1\]], [output,
    healing], [49.3366%], [35.3133%], [10.1216%], [5.1404%], [0.0218%], [0.0663%], [5.2285%],
    [D. SSR or better at 5% (uniform, output)], [5.488], [7 equal steps
    on \[0.8,
    1\]], [hit], [50.6409%], [33.7287%], [10.2774%], [5.2616%], [0.0221%], [0.0692%], [5.3529%],
    [D. SSR or better at 5% (uniform, output)], [5.488], [2 equal steps
    on \[0.8, 1\]], [output,
    healing], [54.0982%], [31.0296%], [9.4370%], [5.3649%], [0.0200%], [0.0503%], [5.4352%],
    [D. SSR or better at 5% (uniform, output)], [5.488], [2 equal steps
    on \[0.8,
    1\]], [hit], [53.6107%], [31.2430%], [9.5827%], [5.4908%], [0.0203%], [0.0525%], [5.5636%],
  )]
  , kind: table
  )

#emph[Table 5. SSR edge candidates under every increment law, for the
four-attribute archetypes of v2. Only the SSR edge moves.]

The edge of v1, $7 mu$, leaves the best single line at SR. The edge
$6 mu = 5.4$ falls in the middle of what six increments at mean value
produce, so its rate moves most with the unpublished increment law, and
it admits souls whose best line is well below perfect. An edge set so
that 5% of souls are SSR or better has no structural meaning and moves
with the assumed law. Six has a structural meaning, keeps SSR rare ---
about one soul in thirty is SSR or better under one archetype --- and
depends least on what the designers did not publish: a soul reaches it
only with a seventh useful increment, or with exactly six useful
increments all at maximum, so its rate follows the exact hit law.

== The exposed order, and the quality of a soul
<the-exposed-order-and-the-quality-of-a-soul>

The exposed order is tier first, then score. UR is first in both orders
within an archetype (Theorem 11). SP is above SSR in tier order, but an
SSR soul may outscore an SP soul by up to
$t_(upright("UR")) - c_(upright("SP"))$: SP marks a kind of excellence, a
fully specialized line on an already strong soul, not a higher number. A
soul's quality is its best (tier, score) under its candidate archetypes.
A soul with several candidates has several chances, which raises its
tier distribution (Table 8). This is fit, not rarity: the commonest
mains, the flat mains of slots 1, 3 and 5, share the advantage with the
rare speed main.

= A one-attribute archetype: speed
<a-one-attribute-archetype-speed>

A soul whose value is one long Speed line is kept for its Speed alone.
Judged by the four-attribute archetypes, it reaches at most six of nine
roll units (Proposition 3) and reads as an ordinary soul: 12.0 Speed
beside two useless lines is SR under `hit` (V19). The `speed` archetype
reads the Speed line by itself, and only when it is long enough to mean
something.

== Adjustment 7: a gate at four roll units
<adjustment-7-a-gate-at-four-roll-units>

`speed` is a #strong[candidate] for a soul only when
$e_(italic("Spd")) gt.eq 4$, 12 displayed Speed at six stars; every
other archetype is always eligible. The gate reads the result, the
line's value, not the number of increments inferred from it: a line of
four maximal increments and a line of five minimal ones, both exactly
four roll units, are judged alike (V19, V21). Under the published range
the two readings nearly coincide --- five increments always reach four
roll units, and four increments reach it only at their maximum --- but a
gate stated on inferred history would let four low increments outrank
fewer high ones for the wrong reason. Four roll units is Yata's choice,
not a rule of the game.

#thm("Proposition", "14")[Candidates; `Arch.coverage`,
`candidate_exists_iff`, `speed_not_candidate`, `speed_gate_mono`][Every legal main attribute of every slot is accepted by some archetype.
A legal soul has a candidate archetype unless its main is `EffectRes` on
slot 4 and $e_(italic("Spd")) < 4$. Below the gate `speed` is not a
candidate, and raising Speed never removes it from the candidates.]

#proof[Coverage and the characterization are case analyses over
the six slots and their legal mains: every main but slot 4's `EffectRes`
is accepted by a four-attribute archetype, which is always eligible, and
`EffectRes` on slot 4 is accepted by `speed` alone. The gate is an
inequality on $e_(italic("Spd"))$, preserved when $e_(italic("Spd"))$
grows.]

With `resist` removed, a slot 4 `EffectRes` soul whose Speed is below
the gate has no candidate. It is #strong[unrated] in the quality pass:
no score and no tier, and its value is a question for the need-dependent
pass. Version 1 required every soul to receive a quality score; that
requirement was a consequence of `resist`, and it is not kept by
re-adding it. Below the gate, `speed` is absent, not scored low: a low
speed score competing in the best-of selection would say nothing the
other archetypes do not.

== The gate is not a condition on the reference
<the-gate-is-not-a-condition-on-the-reference>

`speed`'s anchors are
$E_(italic("speed")) = mu thin bb(E)\[K_(italic("Spd"))\]$ over all
\+15 souls, and $M_(italic("speed")) = 6$. The alternative conditions
the reference on the gate, taking
$E' = bb(E)\[e_(italic("Spd")) divides e_(italic("Spd")) gt.eq 4\]$. The
two answer different questions. The gate decides when the Speed reading
is shown; the normalization says where a Speed line stands in the Speed
dimension of the game's souls. Conditioning would place 60% of eligible
souls below 50 and score the gate itself at 44 --- calling a soul in the
top quarter percent of Speed below the expected soul --- while the
unconditional reference scores the gate at 81.24, above every
four-attribute SSR threshold, and no eligible soul below it (Table 6).
The standard uses the unconditional reference, and checks that it
produces no absurd score at the boundary.

== Tiers for one attribute
<tiers-for-one-attribute>

The four-attribute ladder does not transfer. Its SSR milestone, six roll
units, is `speed`'s maximum; its UR boundary, eight, is unreachable; and
every eligible soul already lies above its R and SR milestones. Its
specialization predicate, some useful line above five roll units, is
exactly the statement that the soul is within one roll unit of `speed`'s
maximum. The standard therefore gives `speed` a rule of its own: an
eligible soul is SSR, and UR when
$e_(italic("Spd")) > M_(italic("speed")) - 1 = 5$ --- the same
frontier principle as the four-attribute UR. `speed` has no SP: an SP
rule built on the generic predicate would never fire below UR, and the
standard does not invent a second condition to fill the tier.

#thm("Theorem", "15")[The speed scale; `speed_utility`,
`speed_utility_bounds`, `speed_score_mono`, `speed_score_strict`,
`maxSpeedSoul_score`][$U_(italic("speed"))\(s\)= e_(italic("Spd"))$
and $0 lt.eq U_(italic("speed")) lt.eq 6$. Raising Speed never lowers
the speed score; the score is strictly increasing over $\[0\,6\]$\; and
the theoretical maximum Speed line, six increments at maximum, scores
$100$.]

#proof[`speed` values one attribute with weight 1, so its utility
is that attribute's value, at most its six increments of at most one
roll unit each. Monotonicity and strictness are Theorem 5 for this
profile with $M = 6$, and $g\(M\)= 100$ (Theorem 4).]

#thm("Theorem", "16")[Speed tiers; `speedTier_cases`, `speedTier_high`,
`speedTier_ne_SP`, `speedTier_mono`, `speed_specialized_iff_UR`,
`speed_UR_six_hits`, `speed_candidate_rank`, `speed_milestones`][`speed`'s tier is a function of $e_(italic("Spd"))$, and exactly one of
two cases holds: UR with $e_(italic("Spd")) > 5$, or SSR with
$e_(italic("Spd")) lt.eq 5$. It is never N, R, SR or SP, it is monotone
in $e_(italic("Spd"))$, and every candidate is at least SSR. The generic
specialization predicate holds under `speed` exactly when the soul is
UR, and a UR soul put all six increments on Speed. The gate lies below
the UR boundary, which is one roll unit below the maximum.]

#proof[The rule is one conditional on $e_(italic("Spd"))$.
Specialization under a one-attribute archetype is
$e_(italic("Spd")) > 5$, the UR condition; Proposition 3 (i) gives six
increments. The milestones are arithmetic, $4 < 5 = 6 - 1$.]

Of all +15 souls, under the reference measure; speed has no N, R, SR or
SP.

#figure(
  align(center)[#table(
    columns: 5,
    align: (auto,auto,auto,auto,auto,),
    table.header([Increment law], [P(eligible)], [SSR], [UR], [eligible:
      1 in],),
    table.hline(),
    [continuous uniform on \[0.8,
    1\]], [0.2471%], [0.2349%], [0.0122%], [405],
    [7 equal steps on \[0.8,
    1\]], [0.2479%], [0.2358%], [0.0121%], [403],
    [2 equal steps on \[0.8,
    1\]], [0.3662%], [0.3554%], [0.0109%], [273],
  )]
  , kind: table
  )

Of eligible souls: where e\_Spd lies, the speed score and tier there,
and how many increments the Speed line has.

#figure(
  align(center)[#table(
    columns: 8,
    align: (auto,auto,auto,auto,auto,auto,auto,auto,),
    table.header([Increment law], [\[4, 4.5)], [\[4.5, 5\]], [\(5,
      5.5\]], [\(5.5, 6\]], [4 increments], [5 increments], [6
      increments],),
    table.hline(),
    [continuous uniform on \[0.8,
    1\]], [47.5306%], [47.5374%], [3.7245%], [1.2075%], [0.0000%], [95.0612%], [4.9388%],
    [7 equal steps on \[0.8,
    1\]], [43.6082%], [51.5074%], [3.6927%], [1.1917%], [0.3201%], [94.7568%], [4.9230%],
    [2 equal steps on \[0.8,
    1\]], [64.5940%], [32.4378%], [1.8226%], [1.1456%], [32.5207%], [64.1466%], [3.3327%],
    [speed score], [81.24 to 85.93], [85.93 to 90.62], [90.62 to
    95.31], [95.31 to 100.00], [], [], [],
    [tier], [SSR], [SSR], [UR], [UR], [], [], [],
  )]
  , kind: table
  )

The reference of speed's normalization, uniform increments: A, the
standard, is unconditional; B would condition it on the gate.

#figure(
  align(center)[#table(
    columns: 7,
    align: (auto,auto,auto,auto,auto,auto,auto,),
    table.header([Reference], [E], [score at e\_Spd =
      4], [4.5], [5], [6], [eligible souls scored below 50],),
    table.hline(),
    [A. unconditional (the
    standard)], [0.6695], [81.24], [85.93], [90.62], [100.00], [0.0000%],
    [B. conditioned on e\_Spd ≥
    4], [4.5444], [44.01], [49.51], [65.65], [100.00], [59.9700%],
  )]
  , kind: table
  )

#emph[Table 6. `speed` under the reference measure: how often a +15 soul
is eligible, where the eligible souls lie, and the two candidate
references.]

About one +15 soul in four hundred is a `speed` candidate; nineteen in
twenty of those have five increments on Speed, and the rest six. One
soul in about eight thousand is `speed` UR, rarer than the
four-attribute UR, as a single line near its maximum should be. Across
archetypes the score thresholds differ: a `speed` UR soul scores at
least $g_(italic("speed"))\(5\)approx 90.6$, below the four-attribute
$t_(upright("UR")) approx 92.2$, and it still ranks above every SSR and
SP soul, because the exposed order is tier first. The gate makes `speed`
a candidate, not the winner: a speed-eligible soul whose four-attribute
reading is higher is best fit under that archetype.

= Calibration
<calibration>

Every table in this section is generated by `formal/calibration/` and
checked against the program by the repository's preflight (§9).

== Tier rates
<tier-rates>

#figure(
  align(center)[#table(
    columns: 8,
    align: (auto,auto,auto,auto,auto,auto,auto,auto,),
    table.header([Increment
      law], [Archetype], [N], [R], [SR], [SSR], [SP], [UR],),
    table.hline(),
    [continuous uniform on \[0.8, 1\]], [output,
    healing], [48.6960%], [36.4318%], [11.5411%], [3.2383%], [0.0209%], [0.0719%],
    [continuous uniform on \[0.8,
    1\]], [hit], [49.3844%], [35.4693%], [11.7274%], [3.3227%], [0.0212%], [0.0751%],
    [7 equal steps on \[0.8, 1\]], [output,
    healing], [49.3366%], [35.3133%], [12.0488%], [3.2133%], [0.0218%], [0.0663%],
    [7 equal steps on \[0.8,
    1\]], [hit], [50.6409%], [33.7287%], [12.2419%], [3.2972%], [0.0221%], [0.0692%],
    [2 equal steps on \[0.8, 1\]], [output,
    healing], [54.0982%], [31.0296%], [11.5452%], [3.2567%], [0.0200%], [0.0503%],
    [2 equal steps on \[0.8,
    1\]], [hit], [53.6107%], [31.2430%], [11.7321%], [3.3414%], [0.0203%], [0.0525%],
  )]
  , kind: table
  )

#emph[Table 7. Tier rates of one four-attribute archetype under the
official weights, for three increment laws with mean 0.9; `output` and
`healing` are identical.]

About half of all +15 souls are below the average soul of a given
four-attribute archetype, one in nine is SR, one in thirty is SSR or
better, and about one in fourteen hundred is UR. SP is rarer than UR: a
line must take all five enhancements, which happens to a given line with
probability $\(1\/4\)^5$ even when all four lines exist. The ladder
orders quality, not rarity.

4000000 souls per row, seed 0x594154417631, uniform increments, the
reference measure. Each cell: rate (standard error), and its z-score
against the exact rate. For speed, "not eligible" is e\_Spd \< 4.

#figure(
  align(center)[#table(
    columns: 8,
    align: (auto,auto,auto,auto,auto,auto,auto,auto,),
    table.header([Row], [N], [R], [SR], [SSR], [SP], [UR], [not
      eligible],),
    table.hline(),
    [exact,
    output], [48.6960%], [36.4318%], [11.5411%], [3.2383%], [0.0209%], [0.0719%], [0.0000%],
    [Monte Carlo, output], [48.7182% (0.0250%) z=+0.89], [36.4113%
    (0.0241%) z=-0.85], [11.5315% (0.0160%) z=-0.60], [3.2486% (0.0089%)
    z=+1.17], [0.0199% (0.0007%) z=-1.28], [0.0705% (0.0013%)
    z=-1.08], [0.0000% (0.0000%)],
    [exact,
    hit], [49.3844%], [35.4693%], [11.7274%], [3.3227%], [0.0212%], [0.0751%], [0.0000%],
    [Monte Carlo, hit], [49.3473% (0.0250%) z=-1.48], [35.5240%
    (0.0239%) z=+2.29], [11.7048% (0.0161%) z=-1.40], [3.3295% (0.0090%)
    z=+0.76], [0.0200% (0.0007%) z=-1.64], [0.0744% (0.0014%)
    z=-0.47], [0.0000% (0.0000%)],
    [exact,
    healing], [48.6960%], [36.4318%], [11.5411%], [3.2383%], [0.0209%], [0.0719%], [0.0000%],
    [Monte Carlo, healing], [48.7135% (0.0250%) z=+0.70], [36.4206%
    (0.0241%) z=-0.46], [11.5309% (0.0160%) z=-0.64], [3.2430% (0.0089%)
    z=+0.53], [0.0192% (0.0007%) z=-2.25], [0.0727% (0.0013%)
    z=+0.56], [0.0000% (0.0000%)],
    [exact,
    speed], [0.0000%], [0.0000%], [0.0000%], [0.2349%], [0.0000%], [0.0122%], [99.7529%],
    [Monte Carlo, speed], [0.0000% (0.0000%)], [0.0000%
    (0.0000%)], [0.0000% (0.0000%)], [0.2338% (0.0024%)
    z=-0.48], [0.0000% (0.0000%)], [0.0118% (0.0005%)
    z=-0.75], [99.7544% (0.0025%) z=+0.64],
  )]
  , kind: table
  )

Largest |z| of a Monte Carlo rate against the exact rate: 2.29.

#figure(
  align(center)[#table(
    columns: 9,
    align: (auto,auto,auto,auto,auto,auto,auto,auto,auto,),
    table.header([Quality over the candidate
      archetypes], [Accepting], [N], [R], [SR], [SSR], [SP], [UR], [unrated],),
    table.hline(),
    [slots 1, 3, 5; Spd on 2], [output, hit, healing, speed], [19.3472%
    (0.0198%)], [48.0466% (0.0250%)], [23.8363% (0.0213%)], [8.5068%
    (0.0139%)], [0.0340% (0.0009%)], [0.2291% (0.0024%)], [0.0000%
    (0.0000%)],
    [AtkPercent on 2, 4, 6], [output], [48.7182% (0.0250%)], [36.4113%
    (0.0241%)], [11.5315% (0.0160%)], [3.2486% (0.0089%)], [0.0199%
    (0.0007%)], [0.0705% (0.0013%)], [0.0000% (0.0000%)],
    [Crit, CritDmg on 6], [output, healing], [36.4321%
    (0.0241%)], [41.7664% (0.0247%)], [16.2661% (0.0185%)], [5.3640%
    (0.0113%)], [0.0283% (0.0008%)], [0.1431% (0.0019%)], [0.0000%
    (0.0000%)],
    [HpPercent on 2, 4, 6], [hit, healing, speed], [28.1760%
    (0.0225%)], [45.8030% (0.0249%)], [19.4226% (0.0198%)], [6.4132%
    (0.0122%)], [0.0264% (0.0008%)], [0.1588% (0.0020%)], [0.0000%
    (0.0000%)],
    [EffectHit on 4; DefPercent on 2, 4, 6], [hit, speed], [49.3473%
    (0.0250%)], [35.4905% (0.0239%)], [11.5736% (0.0160%)], [3.4878%
    (0.0092%)], [0.0147% (0.0006%)], [0.0861% (0.0015%)], [0.0000%
    (0.0000%)],
    [EffectRes on 4], [speed], [0.0000% (0.0000%)], [0.0000%
    (0.0000%)], [0.0000% (0.0000%)], [0.2338% (0.0024%)], [0.0000%
    (0.0000%)], [0.0118% (0.0005%)], [99.7544% (0.0025%)],
  )]
  , kind: table
  )

#emph[Table 8. A Monte Carlo check of the exact rates, and the quality
of a soul over its candidate archetypes, for every distinct set of
accepting archetypes.]

The four archetypes are scored on the same souls, so their Monte Carlo
counts are correlated and form one sample, not four. A conditional
simulation of the values given the exact hit law, a simulation of the
hit process alone, and a separate simulation of twenty million souls
agreed with the exact SP rate of version 1.1, which v2 leaves unchanged;
these were development checks and are not in the repository.

== Sensitivity to what was not published
<sensitivity-to-what-was-not-published>

#figure(
  align(center)[#table(
    columns: 8,
    align: (auto,auto,auto,auto,auto,auto,auto,auto,),
    table.header([μ], [E, output and healing], [E, hit], [E,
      speed], [score V03], [score V05], [score V11], [score V22],),
    table.hline(),
    [0.8 (all rolls
    minimal)], [2.3256], [2.3439], [0.5951], [80.52], [87.26], [98.15], [88.90],
    [0.9 (the standard:
    midpoint)], [2.6163], [2.6369], [0.6695], [79.64], [86.68], [98.12], [88.74],
    [0.9067 (官网
    2.72/3.0)], [2.6357], [2.6564], [0.6745], [79.57], [86.64], [98.12], [88.73],
    [0.9125 (官网
    3.65/4.0)], [2.6526], [2.6735], [0.6788], [79.52], [86.61], [98.12], [88.72],
    [1.0 (all rolls
    maximal)], [2.9070], [2.9299], [0.7439], [78.66], [86.05], [98.10], [88.58],
  )]
  , kind: table
  )

#emph[Table 9. Anchors, and the scores of four test vectors under their
best-fit archetypes, under other means of the increment.]

#figure(
  align(center)[#table(
    columns: 9,
    align: (auto,auto,auto,auto,auto,auto,auto,auto,auto,),
    table.header([Measure], [Archetype], [E], [N], [R], [SR], [SSR], [SP], [UR],),
    table.hline(),
    [36/36/28 per class (official notice; the
    reference)], [output], [2.6163], [48.6960%], [36.4318%], [11.5411%], [3.2383%], [0.0209%], [0.0719%],
    [36/36/28 per class (official notice; the
    reference)], [hit], [2.6369], [49.3844%], [35.4693%], [11.7274%], [3.3227%], [0.0212%], [0.0751%],
    [36/36/28 per class (official notice; the
    reference)], [healing], [2.6163], [48.6960%], [36.4318%], [11.5411%], [3.2383%], [0.0209%], [0.0719%],
    [1/11 per attribute (Hu 2026
    reading)], [output], [2.6182], [48.6513%], [36.4509%], [11.5584%], [3.2464%], [0.0209%], [0.0722%],
    [1/11 per attribute (Hu 2026
    reading)], [hit], [2.6182], [49.8328%], [35.2693%], [11.5584%], [3.2464%], [0.0209%], [0.0722%],
    [1/11 per attribute (Hu 2026
    reading)], [healing], [2.6182], [48.6513%], [36.4509%], [11.5584%], [3.2464%], [0.0209%], [0.0722%],
    [official weights; initial count 1/2, 1/3,
    1/6], [output], [2.5073], [50.8994%], [36.4873%], [10.3522%], [2.2146%], [0.0105%], [0.0360%],
    [official weights; initial count 1/2, 1/3,
    1/6], [hit], [2.5273], [51.6120%], [35.5242%], [10.5395%], [2.2761%], [0.0107%], [0.0375%],
    [official weights; initial count 1/2, 1/3,
    1/6], [healing], [2.5073], [50.8994%], [36.4873%], [10.3522%], [2.2146%], [0.0105%], [0.0360%],
  )]
  , kind: table
  )

#figure(
  align(center)[#table(
    columns: 5,
    align: (auto,auto,auto,auto,auto,),
    table.header([Measure], [E, speed], [P(eligible)], [SSR], [UR],),
    table.hline(),
    [36/36/28 per class (official notice; the
    reference)], [0.6695], [0.2471%], [0.2349%], [0.0122%],
    [1/11 per attribute (Hu 2026
    reading)], [0.6545], [0.2410%], [0.2291%], [0.0119%],
    [official weights; initial count 1/2, 1/3,
    1/6], [0.6419], [0.1516%], [0.1454%], [0.0062%],
  )]
  , kind: table
  )

#emph[Table 10. Tier rates and `speed`'s eligibility under the official
weights, the analysis's one-eleventh reading, and a skewed initial
count, all with v2's thresholds.]

The two readings of the weights differ in $E_p$ by under one percent and
in any tier rate by under half a percentage point. Under the
one-eleventh reading all three four-attribute archetypes would share one
expected utility.

== Comparing normalizations
<comparing-normalizations>

1000000 souls, seed 0x4E4F524D, uniform increments; the same souls for
every profile. Each cell: share of souls scoring at or above the output
archetype's c\_R / c\_SR / c\_SSR / above its t\_UR, and the mean score.

#figure(
  align(center)[#table(
    columns: 7,
    align: (auto,auto,auto,auto,auto,auto,auto,),
    table.header([Normalization], [Profile], [≥ c\_R], [≥ c\_SR], [≥
      c\_SSR], [\> t\_UR], [mean],),
    table.hline(),
    [A. bound: 100·U/M], [speed only (1
    attribute)], [2.153%], [0.247%], [0.070%], [0.0019%], [11.16],
    [A. bound: 100·U/M], [crit pair (2
    attributes)], [8.774%], [2.035%], [0.616%], [0.0197%], [18.54],
    [A. bound: 100·U/M], [output archetype (4
    attributes)], [14.883%], [3.387%], [0.872%], [0.0134%], [29.08],
    [C. mean ratio: min(100, 50·U/E)], [speed only (1
    attribute)], [37.183%], [33.089%], [25.022%], [25.0223%], [33.19],
    [C. mean ratio: min(100, 50·U/E)], [crit pair (2
    attributes)], [44.801%], [43.026%], [23.312%], [23.2598%], [43.90],
    [C. mean ratio: min(100, 50·U/E)], [output archetype (4
    attributes)], [51.295%], [36.025%], [20.394%], [9.4334%], [48.95],
    [H. anchored (the standard)], [speed only (1
    attribute)], [37.183%], [9.821%], [1.784%], [0.0119%], [22.54],
    [H. anchored (the standard)], [crit pair (2
    attributes)], [44.801%], [10.053%], [3.470%], [0.1019%], [33.12],
    [H. anchored (the standard)], [output archetype (4
    attributes)], [51.295%], [14.883%], [3.324%], [0.0748%], [42.29],
    [B. percentile (mid-rank)], [speed only (1
    attribute)], [37.183%], [35.246%], [23.497%], [7.8325%], [50.00],
    [B. percentile (mid-rank)], [crit pair (2
    attributes)], [50.000%], [35.246%], [23.497%], [7.8325%], [50.00],
    [B. percentile (mid-rank)], [output archetype (4
    attributes)], [50.000%], [35.246%], [23.497%], [7.8325%], [50.00],
  )]
  , kind: table
  )

#emph[Table 11. Four normalizations on profiles of one, two and four
useful attributes, on the same simulated souls, against the output
archetype's score thresholds.]

#figure(
  align(center)[#table(
    columns: 7,
    align: (auto,auto,auto,auto,auto,auto,auto,),
    table.header([Soul], [U], [anchored score (any law with mean
      0.9)], [percentile, uniform], [percentile, 7 steps], [percentile,
      2 steps], [P(equal or better), uniform],),
    table.hline(),
    [V02], [2.7000], [50.66], [54.10], [54.10], [54.10], [45.9018% (1 in
    2)],
    [V03], [6.4000], [79.64], [98.47], [98.40], [98.21], [1.5276% (1 in
    65)],
    [V05], [7.3000], [86.68], [99.69], [99.67], [99.62], [0.3115% (1 in
    321)],
    [V10], [8.0000], [92.17], [99.93], [99.93], [99.94], [0.0719% (1 in
    1390)],
  )]
  , kind: table
  )

#emph[Table 12. The anchored score, the percentile under three increment
laws, and the probability of an equal or better soul.]

Table 11 is also why `speed` has a tier rule of its own. On the
four-attribute ladder, a one-attribute profile's tails differ from a
four-attribute profile's under every normalization but the percentile,
and the percentile compresses the top. `speed` keeps the anchored scale,
and replaces the ladder by a gate and a frontier.

= Mechanization and reproducibility
<mechanization-and-reproducibility>

The Lean development in `formal/lean/` has seven modules, pinned to Lean
4.34.0 and Mathlib v4.34.0 #cite(<moura2021lean>)#cite(<mathlib2020>). It models exactly
what the theorems need: attributes, slots, a soul as its increments per
attribute, profiles and archetypes of any size, eligibility and
candidates, anchors, thresholds and tiers, `speed`'s tier rule, and a
finite reference measure. It builds with no `sorry`\; the theorems
depend only on propositional extensionality, choice and quotient
soundness, the axioms of Mathlib's classical logic. The anchors are
parameters in Lean satisfying $0 < E < M$\; their values, and every
rate, come from the calibration program, which is tested and not proved.

The calibration program in `formal/calibration/` is a standalone Rust
program. It computes the hit law exactly with arbitrary-precision
rationals; the tier rates of the continuous uniform law exactly through
the Irwin--Hall distribution #cite(<irwin1927>)#cite(<hall1927>), with one numerical
integral for the SP rate; the stepped laws by exact convolution; and the
Monte Carlo check with a fixed seed. It writes the generated blocks of
this paper and of the specification, and asserts the facts the text
states in words: the milestones ordered and equal to the Lean
definitions, $32\/11$ and $8\/11$ under the one-eleventh reading,
`output` and `healing` equal, the Monte Carlo groups equal to the
catalogue's acceptance, one perfect line below the SP floor, and what
each golden vector exists to show. Preflight runs `lake build` and the
program's `check`\; a changed definition not reflected in both fails the
build.

```bash
cd formal/lean && lake exe cache get && lake build
cargo run --manifest-path formal/calibration/Cargo.toml --release -- check \
  docs/spec/quality-model.md papers/quality-model-v2/paper.md
```

= Rejected alternatives
<rejected-alternatives>

Each alternative below was computed or proved against before it was
rejected.

+ #strong[A bonus for rarity],
  $U + lambda thin\(- log P\(U' gt.eq U\)\)$. Two souls of equal utility
  would score differently by how often the game produces such a soul;
  Theorem 6 rules this out.
+ #strong[Percentile normalization.] Exactly fair by construction and,
  here, robust to the increment law; but it compresses the top, placing
  SSR, SP and near-UR souls within about one and a half points (Table
  12). It is kept as an explanation, never as the score.
+ #strong[Utility over the maximum.] The expected soul would score under
  30, and a one-attribute profile's scores a third of a four-attribute
  profile's (Table 11).
+ #strong[Utility over the expectation.] Unbounded; a quarter of
  speed-only souls saturate at 100 (Table 11).
+ #strong[UR as distance to the Pareto frontier.] With weights
  $\(2\,1\,1\,1\)$, the frontier point with six maximal increments on a
  weight-1 line has utility $10$, while a non-frontier soul with 5.5
  roll units on the weight-2 line has $13.9$, so an SP would outscore a
  UR. For equal weights the utility gap is the distance to the frontier.
+ #strong[SP as "one attribute took every enhancement".] A perfect line
  beside three useless ones would be SP; SP needs a floor of its own.
+ #strong[SSR at $7 mu$], version 1. Superseded: the best single line
  stayed SR (§6.4).
+ #strong[`resist` kept for symmetry], version 1's third archetype.
  Removed: its reason was the shape of the catalogue, and EffectRes is a
  need-dependent question. #strong[`healing` with `DefPercent`] and
  #strong[`speed` with filler attributes] were rejected for the same
  reason.
+ #strong[`speed` normalized against the souls that pass its gate.] Most
  eligible souls would score below 50 (§7.2).
+ #strong[`speed` on the four-attribute ladder], or
  #strong[one-attribute profiles on the ladder in general], whose tails
  differ from four-attribute profiles (Table 11). Rejected; `speed` has
  a gate and a frontier instead (§7.3). #strong[An SP for `speed` from
  an invented second condition] was rejected as a tier with no meaning
  of its own.
+ #strong[The main attribute in utility] (§4.3), and #strong[depth as
  the concentration of all value], which would reward concentration on a
  useless attribute.

= Discussion and limits
<discussion-and-limits>

#emph[What faithfulness covers.] The measure is the published process,
and the reading is the standard's. The standard is faithful in that the
reading never alters the measure (Theorem 6), and the measure takes
nothing that was not published, except where §2 says so. The eligibility
gate is part of the reading: it decides which readings are shown and
leaves the measure, and every anchor computed from it, unchanged.

#emph[Most souls are R.] Within one four-attribute archetype about half
of all souls are N; over the candidate archetypes, the plurality of
souls with a flexible main is R (Table 8). This follows from taking the
best of several archetypes, not from where the R edge lies, and the
standard keeps it: R reads as "at least average for some role the soul
can serve".

#emph[Weights are equal.] Every useful attribute has weight 1. The
maintainer expects `EffectHit` and `Spd` to weigh more in `hit` in a
future weighted model. That is a separate calibration problem; this
version isolates the catalogue from it, and Rejected alternative 5
records the constraint weights put on UR.

The formal and numerical claims are further bounded by the following.

- #emph[Six stars only.] Lower-star ranges are not published.
- #emph[The increment law is unpublished.] Scores depend on it only
  through $mu$\; tier rates are reported under three laws.
- #emph[The catalogue is authored.] `hit`, `healing` and `speed`, their
  mains, and the gate await review; a changed catalogue is a new model
  version.
- #emph[Interactions are out of the quality pass.] Crit's cap, speed
  thresholds and set effects belong to the need-dependent pass.
- #emph[Candidates use current values.] A soul below +15 whose Speed may
  later pass the gate is not yet judged by `speed`\; its growth is
  reported under its current best fit.
- #emph[The calibration is tested, not proved.] It is checked against
  Monte Carlo and against the Lean definitions of the milestones.
- #emph[Nothing about players.] The adjustments follow how players read
  souls; whether they succeed is an empirical question.

= Conclusion
<conclusion>

The standard is interesting not because it produces a number, but
because it can say which parts of that number belong to the game and
which to the standard. The game's part is the published enhancement
process, reconstructed exactly, with the one unpublished quantity
entering only through its mean. The standard's part is a short list of
declared adjustments --- roll units, a catalogue of archetypes of
different sizes, main-attribute gating and a Speed gate, three anchors
per archetype, and tiers at utility milestones --- each chosen to read
as players read souls, and each proved not to feed rarity back into the
score. That the adjustments preserve bounds, monotonicity and dominance,
that `healing` and `output` share one scale because the rules treat
their attributes alike, that one perfect line is exactly SSR, that UR is
above everything of its archetype, and that `speed` is judged only above
its gate and only by a rule that means something for one line, are
theorems; that the rates are what the published rules imply is a
computation anyone can repeat.

#heading(level: 1, numbering: none)[Appendix A --- Theorem index]
<appendix-a-theorem-index>

Every name is a Lean declaration in `formal/lean/` of `KCN-judu/yata`,
in the namespace `Yata`.

#figure(
  align(center)[#table(
    columns: 3,
    align: (auto,auto,auto,),
    table.header([Paper], [Lean names], [File],),
    table.hline(),
    [Prop 1], [paper proof; computed by the calibration's
    `mean_useful`], [---],
    [Prop 2], [`Archetype.utility_le_nine`, `utility_le_useful_hits`,
    `paperSoul_utility`, `paperSoul_score`], [`Archetype`],
    [Prop 3], [`Soul.six_hits_of_gt_five`,
    `Archetype.single_line_le_six`, `beyond_six_needs_another_line`,
    `nine_useful_of_gt_eight`, `line_le_utility`], [`Soul`,
    `Archetype`],
    [Thm 4], [`Anchors.g_nonneg`, `g_le_100`, `g_zero`, `g_E`,
    `g_M`], [`Score`],
    [Thm 5], [`Anchors.g_mono`, `g_strict`, `g_scale`\;
    `Profile.utility_mono`, `utility_strict`, `score_mono`,
    `score_strict`], [`Score`, `Soul`],
    [Thm 6], [`Profile.score_factors`], [`Soul`],
    [Thm 7], [`expected_relabel`, `score_relabel`, `utility_relabel`\;
    `archetype_relabel`, `healing_is_output_relabelled`,
    `healing_expected_eq_output`], [`Fairness`, `Quality`],
    [Thm 8], [`milestones_ordered`\; `Thresholds.tier_cases`,
    `tier_UR_iff`, `tier_SP_iff`, `band_ne_SP`, `band_ne_UR`\;
    `Tier.ladder`], [`Quality`, `Tier`],
    [Thm 9], [`Thresholds.sp_needs_floor`, `sp_is_ssr_quality`\;
    `single_line_not_SP`], [`Tier`, `Quality`],
    [Thm 10], [`perfect_line_at_least_SSR`,
    `perfect_single_line_is_SSR`\; `Thresholds.at_ssr_floor_high_tier`,
    `ssr_floor_rank`], [`Quality`, `Tier`],
    [Thm 11], [`UR_iff_gt_eight`, `UR_structure`, `paperSoul_UR`\;
    `Thresholds.UR_score_gt`, `UR_top`], [`Quality`, `Tier`],
    [Thm 12], [`Thresholds.tier_mono`\; `tier_mono_dominance`,
    `tierOn_mono_dominance`, `tierOn_broad`,
    `exposed_mono_dominance`], [`Tier`, `Quality`],
    [Thm 13], [`lower_ssr_floor_never_lowers_tier`], [`Tier`],
    [Prop 14], [`Arch.coverage`, `Arch.accepts_legal`,
    `Arch.card_broad`, `Arch.card_speed`, `candidate_exists_iff`,
    `speed_not_candidate`, `speed_candidate_iff`, `speed_gate_mono`,
    `gate_broad`], [`Quality`],
    [Thm 15], [`speed_utility`, `speed_utility_bounds`,
    `speed_score_mono`, `speed_score_strict`, `maxSpeedSoul_speed`,
    `maxSpeedSoul_score`], [`Quality`],
    [Thm 16], [`speedTier_cases`, `speedTier_high`, `speedTier_ne_SP`,
    `speedTier_mono`, `speed_specialized_iff_UR`, `speed_UR_six_hits`,
    `speed_candidate_rank`, `speed_milestones`], [`Quality`],
  )]
  , kind: table
  )

#heading(level: 1, numbering: none)[Appendix B --- Golden test vectors]
<appendix-b-golden-test-vectors>

Stored values are in display units, with each line's increment count in
parentheses; the full results per archetype are in
`formal/calibration/out/test-vectors.json`.

#figure(
  align(center)[#table(
    columns: 12,
    align: (auto,auto,auto,auto,auto,auto,auto,auto,auto,auto,auto,auto,),
    table.header([Id], [Soul], [Slot, main, level], [Best
      fit], [U], [Score], [Specialized], [Tier], [Depth], [Breadth], [Growth], [Every
      candidate],),
    table.hline(),
    [V01], [AtkFlat 72.0 (3); HpFlat 205.0 (2); DefFlat 13.5 (3);
    EffectRes 3.6 (1)], [1, AtkFlat,
    \+15], [output], [0.0000], [0.00], [no], [N], [0.0], [0.0], [---], [output
    N 0.00; hit N 0.00; healing N 0.00],
    [V02], [Crit 5.4 (2); AtkPercent 2.7 (1); HpFlat 300.0 (3); DefFlat
    13.8 (3)], [3, DefFlat,
    \+15], [output], [2.7000], [50.66], [no], [R], [30.0], [26.7], [---], [output
    R 50.66; healing N 34.40; hit N 0.00],
    [V03], [Crit 8.4 (3); Spd 5.4 (2); AtkPercent 5.4 (2); HpFlat 205.0
    (2)], [6, CritDmg,
    \+15], [output], [6.4000], [79.64], [no], [SSR], [46.7], [62.0], [---], [output
    SSR 79.64; healing SR 65.54],
    [V04], [Crit 18.0 (6); HpFlat 114.0 (1); DefFlat 5.0 (1); AtkFlat
    27.0 (1)], [1, AtkFlat,
    \+15], [output], [6.0000], [76.50], [yes], [SSR], [100.0], [0.0], [---], [output
    SSR 76.50; healing SSR 76.50; hit N 0.00],
    [V05], [Crit 16.5 (6); CritDmg 3.6 (1); AtkPercent 2.7 (1); HpFlat
    100.0 (1)], [5, HpFlat,
    \+15], [output], [7.3000], [86.68], [yes], [SP], [91.7], [22.4], [---], [output
    SP 86.68; healing SP 79.64; hit N 0.00],
    [V06], [Crit 15.0 (5); CritDmg 3.6 (1); AtkPercent 2.7 (1); HpFlat
    200.0 (2)], [5, HpFlat,
    \+15], [output], [6.8000], [82.77], [no], [SSR], [83.3], [24.6], [---], [output
    SSR 82.77; healing SR 75.72; hit N 0.00],
    [V07], [Crit 15.3 (6); AtkPercent 3.0 (1); DefFlat 4.5 (1); HpFlat
    100.0 (1)], [1, AtkFlat,
    \+15], [output], [6.1000], [77.29], [yes], [SSR], [85.0], [12.6], [---], [output
    SSR 77.29; healing SR 69.45; hit N 0.00],
    [V08], [Crit 18.0 (6); CritDmg 4.0 (1); AtkPercent 3.0 (1); Spd 3.0
    (1)], [6, CritDmg,
    \+15], [output], [9.0000], [100.00], [yes], [UR], [100.0], [35.9], [---], [output
    UR 100.00; healing SP 92.17],
    [V09], [Crit 8.7 (3); CritDmg 7.8 (2); AtkPercent 5.8 (2); Spd 5.8
    (2)], [2, Spd,
    \+15], [output], [8.7167], [97.78], [no], [UR], [48.3], [95.3], [---], [output
    UR 97.78; healing SSR 82.64; hit N 36.66],
    [V10], [Crit 18.0 (6); CritDmg 4.0 (1); AtkPercent 3.0 (1); HpFlat
    114.0 (1)], [6, CritDmg,
    \+15], [output], [8.0000], [92.17], [yes], [SP], [100.0], [22.8], [---], [output
    SP 92.17; healing SP 84.34],
    [V11], [Spd 17.4 (6); EffectHit 3.6 (1); HpPercent 2.7 (1); DefFlat
    4.5 (1)], [2, Spd,
    \+15], [speed], [5.8000], [98.12], [yes], [UR], [96.7], [n/a], [---], [speed
    UR 98.12; hit SP 89.00; healing SP 81.99; output SR 74.94],
    [V12], [Crit 17.4 (6); AtkPercent 2.7 (1); Spd 2.7 (1); DefFlat 4.5
    (1)], [6, CritDmg,
    \+15], [output], [7.6000], [89.03], [yes], [SP], [96.7], [21.3], [---], [output
    SP 89.03; healing SP 81.99],
    [V13], [CritDmg 23.2 (6); AtkPercent 2.7 (1); Spd 2.7 (1); HpFlat
    110.0 (1)], [6, Crit,
    \+15], [output], [7.6000], [89.03], [yes], [SP], [96.7], [21.3], [---], [output
    SP 89.03; healing SP 81.99],
    [V14], [Crit 8.1 (3); Spd 5.4 (2); AtkPercent 2.7 (1); HpFlat 300.0
    (3)], [4, AtkPercent,
    \+15], [output], [5.4000], [71.80], [no], [SR], [45.0], [52.4], [---], [output
    SR 71.80],
    [V15], [Crit 8.4 (3); Spd 5.4 (2); AtkPercent 2.7 (1); HpFlat 300.0
    (3)], [4, AtkPercent,
    \+15], [output], [5.5000], [72.59], [no], [SR], [46.7], [51.5], [---], [output
    SR 72.59],
    [V16], [AtkFlat 72.0 (3); HpFlat 205.0 (2); DefFlat 13.5 (3);
    CritDmg 3.6 (1)], [4, EffectHit,
    \+15], [hit], [0.0000], [0.00], [no], [N], [0.0], [0.0], [---], [hit
    N 0.00],
    [V17], [Spd 5.7 (2); Crit 5.6 (2); AtkPercent 2.8 (1)], [2, Spd,
    \+9], [output], [4.7000], [66.32], [no], [SR], [31.7], [59.1], [73.58], [output
    SR 66.32; healing R 59.01; hit N 36.03],
    [V18], [Spd 11.9 (4); HpPercent 2.7 (1); DefPercent 2.7 (1); AtkFlat
    27.0 (1)], [2, Spd,
    \+15], [hit], [5.7667], [74.59], [no], [SR], [66.1], [30.5], [---], [hit
    SR 74.59; healing SR 67.63; output R 60.58],
    [V19], [Spd 12.0 (5); HpPercent 2.7 (1); DefPercent 2.7 (1); AtkFlat
    27.0 (1)], [2, Spd,
    \+15], [speed], [4.0000], [81.24], [no], [SSR], [66.7], [n/a], [---], [speed
    SSR 81.24; hit SR 74.85; healing SR 67.89; output R 60.84],
    [V20], [Spd 12.1 (5); HpPercent 2.7 (1); DefPercent 2.7 (1); AtkFlat
    27.0 (1)], [2, Spd,
    \+15], [speed], [4.0333], [81.55], [no], [SSR], [67.2], [n/a], [---], [speed
    SSR 81.55; hit SR 75.12; healing SR 68.15; output R 61.10],
    [V21], [Spd 12.0 (4); HpPercent 2.7 (1); DefPercent 2.7 (1); AtkFlat
    27.0 (1)], [2, Spd,
    \+15], [speed], [4.0000], [81.24], [no], [SSR], [66.7], [n/a], [---], [speed
    SSR 81.24; hit SR 74.85; healing SR 67.89; output R 60.84],
    [V22], [Spd 14.4 (5); HpPercent 2.7 (1); DefPercent 2.7 (1); AtkFlat
    27.0 (1)], [2, Spd,
    \+15], [speed], [4.8000], [88.74], [no], [SSR], [80.0], [n/a], [---], [speed
    SSR 88.74; hit SSR 81.14; healing SR 74.15; output SR 67.10],
    [V23], [Spd 15.0 (6); HpPercent 2.7 (1); DefPercent 2.7 (1); AtkFlat
    27.0 (1)], [2, Spd,
    \+15], [speed], [5.0000], [90.62], [no], [SSR], [83.3], [n/a], [---], [speed
    SSR 90.62; hit SSR 82.71; healing SR 75.72; output SR 68.67],
    [V24], [Spd 16.2 (6); HpFlat 100.0 (1); DefFlat 4.5 (1); AtkFlat
    25.0 (1)], [4, EffectRes,
    \+15], [speed], [5.4000], [94.37], [yes], [UR], [90.0], [n/a], [---], [speed
    UR 94.37],
    [V25], [Spd 18.0 (6); HpPercent 2.7 (1); DefPercent 2.7 (1); AtkFlat
    27.0 (1)], [2, Spd,
    \+15], [speed], [6.0000], [100.00], [yes], [UR], [100.0], [n/a], [---], [speed
    UR 100.00; hit SP 90.57; healing SP 83.55; output SSR 76.50],
    [V26], [Spd 8.1 (3); Crit 5.4 (2); HpPercent 2.7 (1); DefFlat 5.0
    (1)], [4, EffectRes,
    \+15], [---], [---], [---], [---], [unrated], [---], [---], [---], [none],
    [V27], [HpPercent 8.4 (3); CritDmg 7.6 (2); Spd 5.7 (2); DefFlat 5.0
    (1)], [6, Crit,
    \+15], [healing], [6.6000], [81.20], [no], [SSR], [46.7], [63.1], [---], [healing
    SSR 81.20; output R 59.27],
    [V28], [HpPercent 11.1 (4); Crit 5.7 (2); Spd 2.7 (1); AtkFlat 27.0
    (1)], [4, HpPercent,
    \+15], [healing], [6.5000], [80.42], [no], [SSR], [61.7], [44.4], [---], [healing
    SSR 80.42; hit SR 65.43],
    [V29], [EffectHit 13.6 (4); Spd 5.4 (2); HpPercent 2.7 (1); DefFlat
    5.0 (1)], [4, EffectHit,
    \+15], [hit], [6.1000], [77.21], [no], [SSR], [56.7], [46.1], [---], [hit
    SSR 77.21],
    [V30], [Spd 13.5 (5); HpPercent 2.5 (1)], [2, Spd,
    \+12], [speed], [4.5000], [85.93], [no], [SSR], [75.0], [n/a], [86.72], [speed
    SSR 85.93; healing SR 71.28; hit SR 71.19; output SR 64.75],
  )]
  , kind: table
  )

#figure(
  align(center)[#table(
    columns: 2,
    align: (auto,auto,),
    table.header([Id], [What it shows],),
    table.hline(),
    [V01], [obviously poor: flat lines and one EffectRes increment,
    useful to no v2 archetype (resist, 17.07, in v1.1)],
    [V02], [ordinary useful: three useful increments],
    [V03], [strong balanced SSR, no specialized line],
    [V04], [one perfect line, nothing useful beside it: SSR, not SP (SR
    in v1)],
    [V05], [valid SP],
    [V06], [near SP: five roll units exactly is not specialized],
    [V07], [specialized, below the SP floor: SSR, not SP (SR in v1)],
    [V08], [valid UR: the paper's theoretical output soul],
    [V09], [valid UR without a specialized line],
    [V10], [near UR: eight useful increments at maximum, one wasted],
    [V11], [Speed-focused: UR under speed, SP under hit (hit SP in
    v1.1)],
    [V12], [Crit-focused],
    [V13], [CritDmg-focused: the counterpart of V12],
    [V14], [dominated: V15 improves its Crit],
    [V15], [dominates V14],
    [V16], [rare main, useless lines: a rare main earns nothing],
    [V17], [growth: +9, two rolls left],
    [V18], [speed gate, just below: 3.97 roll units, speed is not a
    candidate],
    [V19], [speed gate, exactly four roll units: speed is a candidate],
    [V20], [speed gate, just above],
    [V21], [four roll units from four maximal increments: eligible by
    value, like V19],
    [V22], [strong Speed: 4.8 roll units],
    [V23], [speed's UR boundary: exactly five roll units is SSR],
    [V24], [extreme Speed on an EffectRes main: UR under speed, its only
    candidate],
    [V25], [the theoretical maximum Speed line: 100 under speed],
    [V26], [unrated: an EffectRes main with Speed below the gate has no
    candidate archetype],
    [V27], [healing: SSR under healing, R under output on the same
    soul],
    [V28], [healing on an HpPercent main: SSR under healing, SR under
    hit],
    [V29], [hit: EffectHit and Speed, equal weights],
    [V30], [growth under speed: +12, eligible now, one roll left],
  )]
  , kind: table
  )
