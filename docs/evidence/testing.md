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

## Store instructions — `yata-store`

| Claim                                                         | Tests                                                                                                                                                               |
| ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| no instruction updates or deletes a commit (ADR-0019, rule 5) | `plan::tests::no_instruction_updates_or_deletes_a_commit`                                                                                                           |
| instructions planned together share one atomic batch          | `plan::tests::instructions_that_land_together_share_one_atomic_batch`                                                                                               |
| appending guards density; reading back checks it              | `plan::tests::appending_binds_seq_and_bytes_and_guards_density`, `interpret::tests::an_append_that_changed_nothing_was_not_next`, `commits_read_back_must_be_dense` |
| empty, Yata, and foreign files are told apart                 | `interpret::tests::inspect_tells_empty_yata_and_foreign_files_apart`                                                                                                |
| every table is `STRICT`; exclusive locking precedes WAL       | `schema::tests`                                                                                                                                                     |
| results of the wrong shape are errors, not guesses            | `interpret::tests::a_result_of_the_wrong_shape_is_an_error`                                                                                                         |

The plans are tested as values. That the SQL runs on SQLite is tested with the
daemon's executor.

## Not covered

Acquisition probabilities (§ Acquisition) and 奉纳 rates are specified and not
implemented.
