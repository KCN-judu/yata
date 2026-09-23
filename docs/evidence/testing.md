---
kind: evidence
status: current
area: process
---

# Testing

Which test backs which claim. A row in
[../project/status.md](../project/status.md) may say _tested_ only for what a
test named here exercises. Test names are relative to the crate's source.

## Suites

| Suite           | Command                                               | Runs where            |
| --------------- | ----------------------------------------------------- | --------------------- |
| Rust unit tests | `cargo test --workspace` (in `just check`, CI `rust`) | Linux, Windows, macOS |

## Soul mechanics — `yata-core`

| Rule in [soul-mechanics.md](../spec/soul-mechanics.md)                 | Tests                                                                                                                                                                                               |
| ---------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| slot and main-attribute table (`Main(k)`)                              | `soul::slot::tests`                                                                                                                                                                                 |
| attribute categories and roll classes                                  | `soul::attribute::tests`                                                                                                                                                                            |
| § Values, `Max = 6 · hi`, `lo = 0.8 · hi` at 6★, TBD below 6★          | `mechanics::values::tests`                                                                                                                                                                          |
| M-Main                                                                 | `values::tests::main_values_at_plus_fifteen_match_the_table`, `main_values_at_the_nodes_match_the_growth_table`; the warning: `legality::tests::a_main_value_off_the_growth_line_is_a_warning_only` |
| W-Soul                                                                 | `mechanics::legality::tests`                                                                                                                                                                        |
| I-Hits, including the 14.8 speed example and "above 5 · hi forces six" | `mechanics::inference::tests`                                                                                                                                                                       |
| comparing values with a tolerance                                      | `inference::tests::exact_decimal_bounds_are_admitted_despite_float_error`                                                                                                                           |
| 双速, 拉满, 真 n, 顶段; `top_band` decidable from values alone         | `mechanics::predicates::tests`                                                                                                                                                                      |
| class weights, P-Draw, the three-leg table, the first roll's 72–73%    | `mechanics::distribution::tests`                                                                                                                                                                    |

## Not covered

Acquisition probabilities (§ Acquisition) and 奉纳 rates are specified and not
implemented.
