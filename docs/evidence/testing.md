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

| Suite                           | Command                                               | Runs where            |
| ------------------------------- | ----------------------------------------------------- | --------------------- |
| Rust unit and integration tests | `cargo test --workspace` (in `just check`, CI `rust`) | Linux, Windows, macOS |

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

The plans are tested as values here, and run against SQLite below.

## The store on SQLite — `yata-daemon`

Integration tests in `crates/yata-daemon/tests/store.rs`, each against a real
SQLite file under a directory whose name has Chinese characters and a space
(ADR-0010), on Linux, Windows, and macOS.

| Claim                                                                | Test                                            |
| -------------------------------------------------------------------- | ----------------------------------------------- |
| a store is created, initialized with its format and id, and reopened | `a_store_is_created_initialized_and_reopened`   |
| opening never creates a missing store                                | `opening_never_creates_a_missing_store`         |
| an existing store is opened without a new id                         | `an_existing_store_is_opened_without_a_new_id`  |
| commits append densely; a gap or a repeat is refused                 | `commits_append_densely_and_read_back_in_order` |
| a failed commit keeps none of its writes                             | `a_failed_commit_keeps_none_of_its_writes`      |
| blobs are stored once and pruned by digest                           | `blobs_are_stored_once_and_pruned_by_digest`    |
| the projection cache keeps one entry                                 | `the_projection_cache_keeps_one_entry`          |
| a file that is not a database is refused and left untouched          | `a_foreign_file_is_refused_and_left_untouched`  |
| an empty file is initialized only on request                         | `an_empty_file_is_initialized_only_on_request`  |

## Frame codec — `yata-protocol`

| Claim in [core-protocol.md](../spec/core-protocol.md), § Frame | Test in `frame::tests`                                    |
| -------------------------------------------------------------- | --------------------------------------------------------- |
| the prefix is the payload length, big-endian, excluding itself | `the_prefix_is_the_payload_length_big_endian`             |
| an empty payload or a zero prefix is a protocol error          | `an_empty_payload_is_not_a_message`                       |
| the maximum is 16 MiB, inclusive                               | `the_maximum_is_sixteen_mebibytes_inclusive`              |
| an oversized prefix is rejected before its bytes are awaited   | `an_oversized_prefix_is_rejected_before_its_bytes_arrive` |
| frames arriving in pieces decode whole and in order            | `frames_arriving_in_pieces_decode_whole_and_in_order`     |
| a recording cut inside a frame is truncated, not ended         | `a_recording_cut_inside_a_frame_is_truncated_not_ended`   |

## Probe schema — `yata-protocol`

| Claim                                                              | Test in `tests` (crate root)                   |
| ------------------------------------------------------------------ | ---------------------------------------------- |
| a probe message survives encoding, framing, and decoding unchanged | `a_probe_message_survives_a_frame`             |
| an unrecorded roll count is distinct from zero rolls on the wire   | `an_absent_roll_count_differs_from_zero_rolls` |

## Scheme-code transport — `yata-core::scheme`

| Claim in [scheme-code.md](../spec/scheme-code.md), § Transport                                | Tests                                                                                                                                                          |
| --------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| a text from another zlib and Base64 implementation decodes                                    | `transport::tests::a_text_from_another_implementation_decodes`, `padding_and_a_stored_block_decode`                                                            |
| malformed Base64, whitespace, and the URL-safe alphabet fail in the Base64 layer              | `malformed_base64_is_a_base64_error`, `surrounding_whitespace_is_not_part_of_a_code`                                                                           |
| truncated, corrupted, and non-zlib streams fail in the zlib layer; trailing bytes are refused | `a_truncated_stream_is_a_zlib_error`, `a_corrupted_checksum_is_a_zlib_error`, `bytes_that_are_not_zlib_are_a_zlib_error`, `bytes_after_the_stream_are_refused` |
| decompression stops at `MAX_PAYLOAD_LEN`, inclusive; a bomb is refused                        | `a_decompression_bomb_stops_at_the_limit`, `the_payload_limit_is_inclusive`                                                                                    |
| an empty payload is refused; the text and compressed limits hold                              | `an_empty_stream_is_not_a_scheme`, `an_overlong_text_is_refused_before_decoding`, `an_incompressible_payload_too_big_for_a_code_is_refused`, `payload::tests`  |
| payload identity: `decode(encode(p)) = p`                                                     | property `every_encodable_payload_round_trips` (1 byte to 2 KiB); `encoding_is_deterministic_and_decodes_to_the_same_payload`                                  |
| dump rows, diff by offset with XOR and bits, no alignment across lengths                      | `inspect::tests`; properties `changes_are_exactly_the_offsets_whose_xor_is_nonzero`, `diff_is_symmetric`, `a_payload_never_differs_from_itself`                |
| bits read LSB-first; spans are bounded                                                        | `bits_are_read_lsb_first_across_bytes`, `a_span_past_the_end_or_of_no_length_is_refused`                                                                       |

## QR codes and research commands — `yata-daemon`

| Claim in [scheme-code.md](../spec/scheme-code.md), § QR codes                                               | Tests                                                                                                                                                                    |
| ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| the same text always gives the same matrix                                                                  | `qr::tests::the_same_text_always_gives_the_same_matrix`                                                                                                                  |
| a rendered code, as pixels or as a PNG, reads back as its text                                              | `a_rendered_code_reads_back_as_its_text`, `a_png_rendering_reads_back_as_its_text`                                                                                       |
| a text beyond version 40 is refused; oversized images, non-PNG bytes, and images without a code are refused | `a_text_beyond_version_forty_is_refused`, `oversized_images_are_refused_before_reading`, `bytes_that_are_not_a_png_are_refused`, `an_image_without_a_code_reads_as_none` |
| the render scale is bounded                                                                                 | `the_render_scale_is_bounded`                                                                                                                                            |
| dump and diff output is fixed                                                                               | `scheme::tests`                                                                                                                                                          |
| code files read as text or PNG, bounded                                                                     | `tests/scheme_files.rs`                                                                                                                                                  |
| every real code decodes, re-encodes to the same payload, and survives the QR loop                           | `tests/scheme_corpus.rs`, over the local corpus only; in CI it checks nothing                                                                                            |

On the one real code in the local corpus (a 30-plan strengthening set, 1 102
bytes), the payload round-trips and survives the QR loop. The text this encoder
produces differs from the game's, as § Transport allows.

## Not covered

Acquisition probabilities (§ Acquisition) and 奉纳 rates are specified and not
implemented.
