# qop-core

Core data model for **Qop** ("qwert-o-phone", working name): a configurable instrument emulator played from a computer keyboard. This crate holds the instrument-definition structures and the tuning data. It makes no sound, opens no window, and has no I/O: it is a library that any front end (egui app, plugin, CLI, WASM) builds on, and it must stay free of UI and windowing dependencies. Upstream: https://github.com/JazzerThighs/qop-core

## Status — read this before assuming anything works

- **Edit side is written and public; Play side is not written.** `Engine<Edit>` has a full set of mutation methods and read accessors. Every file under `src/engine/_play/` is empty (0 bytes), so there is no code yet that turns key presses into notes. `Engine<Play>` can be created and read, nothing more.
- **`Temperament` only stores data.** It has read accessors but no edit methods and no index-to-frequency lookup. `Temperament::new` panics for `EqualTemperament` and `PrimeLimit` (`TemperamentType::intervals` is `todo!()`).
- **No binary, no examples.** Tests live in `src/engine/_edit/_tests.rs` (crate-internal) and `tests/public_api.rs` (public API only).
- The `QopPart<T>` trait has no implementors.

## Commands

```bash
cargo check
cargo test
cargo test --features winit   # also tests the winit key conversion
```

Edition 2024. Dependencies are pinned to a major version in `Cargo.toml` and `Cargo.lock` is committed. `winit` is optional and off by default; the only thing it enables is `Key::from_winit`.

## Public API

- `qop_core::{Engine, Edit, Play, Key, Temperament}` are re-exported at the crate root; the nested structs (`Gut`, `VFSet`, `ComboSet`, ...) are under `qop_core::engine`.
- **Reading**: every field has a getter of the same name (`engine.guts()`, `gut.togs()`, `set.buttons()`), generated in `src/engine/_access.rs`. `name` and `description` are plain `pub` fields.
- **Writing**: only through `Engine<Edit>` methods. Fields are `pub(crate)` on purpose so outside code cannot break the invariants. A new field needs a getter in `_access.rs` and, if editable, an edit method.
- **Edit methods return `bool`**: `true` if the edit was applied, `false` if the arguments were rejected (index out of range, unknown key, constraint violated, key already registered). A rejected edit changes nothing.
- `Engine<Edit>::validate()` runs every invariant check; `to_play()` calls it and returns the `Engine<Play>` copy. There is no Play-to-Edit conversion: keep the `Engine<Edit>` and call `to_play()` again.
- Engines derive `Serialize` / `Deserialize`; the crate picks no file format.

## Layout

| Path | Contents |
|---|---|
| `src/lib.rs` | `Edit` / `Play` typestate markers, `QopPart<T>` trait, root re-exports |
| `src/key.rs` | `Key`: the crate's own physical-key type, plus the optional winit conversion |
| `src/engine.rs` | The whole `Engine<Mode>` struct tree (one `nest!` block) and `Engine<Edit>::to_play()` |
| `src/engine/_access.rs` | Read-only getters for every struct in the tree |
| `src/engine/_edit.rs` | `NewTrait` + `NewEnginePartParams`: constructors that size per-gut vectors correctly |
| `src/engine/_edit/_dig_inputs.rs` | Key registry edits (insert / purge / swap / remap) and the index-bounds validator |
| `src/engine/_edit/_gut.rs` | Gut insert/remove, gut triggers, gut transpositions, names, delta-length and pressed-length validators |
| `src/engine/_edit/_holds.rs` | Sustain / sostenuto key assignment, for guts and for each set kind |
| `src/engine/_edit/_trnsp.rs` | `trnsp_all` / `trnsp_one` transposition edits on sets |
| `src/engine/_edit/_vfc_sets.rs` | Set, button, and combo insert/remove; delta edits; combo patterns; names; min/max pressed; radio mode |
| `src/engine/_edit/_tests.rs` | Regression tests for the edit methods (`#[cfg(test)]`) |
| `tests/public_api.rs` | Integration tests that use only the public API, as an application would |
| `src/engine/_play/*` | Empty placeholders mirroring `_edit/` |
| `src/temperament.rs` | `Temperament<Mode>`: reference note, tuning Hz, octave factor, note names, interval list |
| `src/temperament/intervals.rs` | Harmonic helpers and `WIKI_MICROTONAL_INTERVALS` (~315 named ratios) |
| `src/temperament/scale_list/twelve_tet.rs` | `TWELVE_TET_SCALES`: 1012 named 12-TET scales, 5.8k lines of data. Grep it, don't read it. |
| `collected_texts.txt` | Raw ratio list the interval table was transcribed from |

## Data model

`Engine` describes one instrument. Terms as used in the code:

- **`dig_inputs: Vec<Key>`** — registry of digital inputs. `Key` names physical keys after the W3C `code` values, with `Key::Other(u32)` for anything else (unlisted keys, gamepad buttons, MIDI notes). Everything else refers to an input by its **index into this vec**, never by `Key`.
- **Gut** — one sounding voice, like a string. Has `togs` (key indices that sound it), a base output `index_out: usize` and `extra_out: f64`, and `trnsp_gut` (key-triggered offsets for that gut alone).
- **`v_multi` / `f_multi`: `Vec<VFSet>`** — sets of buttons. Each `VFBtn` carries `i_delta` / `x_delta` vectors with **one entry per gut**: how far that button shifts each gut. The two fields share a type; the names suggest valve-like and fret-like sets, but nothing in the code yet says how they differ. That difference will live in the Play logic — ask the author rather than guess.
- **`c_multi: Vec<ComboSet>`** — buttons plus a table of `Combo`s. Each combo is a `Vec<bool>` pattern over the set's buttons with its own per-gut deltas, like a fingering chart.
- **Transpositions** — `Trnsp` (scalar, per gut), `MulTrnsp` (per-gut vectors). On sets, `trnsp_all` applies to the whole set and `trnsp_one` to a single button/combo.
- **Holds** — `HoldBtns { sustain, inv_sustain, sostenuto, inv_sostenuto }`, present on the gut layer and on every set.
- **`radio_mode`, `min_pressed`, `max_pressed`** — constraints on simultaneous presses, on the gut layer and per set.
- **`i_*` vs `x_*`** — `i` is an integer step (an index into the temperament's notes); `x` is a continuous `f64` offset. The engine's per-voice output is the pair `(usize, f64)`, which is what `QopPart` passes around.
- **`*_mem`, `pressed`** — runtime state stored inside the definition structs, intended for Play mode.

### Invariants

`validate()` (and therefore `to_play()`) checks 1 to 4.

1. Every key index (`togs`, `triggers`, hold `togs`) is `< dig_inputs.len()`. Checked by `check_digitalref_invariants`.
2. Every per-gut vector (`i_delta`, `x_delta`, `i_mem`, `x_mem`, including inside `trnsp_one` / `trnsp_all`) has length `guts.len()`. Checked by `check_multi_delta_lengths`.
3. `set.pressed.len() == set.buttons.len()`. Checked by `check_pressed_lengths`.
4. `combo.combo.len() == combo_set.buttons.len()`. Checked by `check_pressed_lengths`.
5. There is always at least one gut.

Adding or removing a gut must touch every set (`insert_gut` / `remove_gut`). Removing a key must renumber every index vec in the engine (`dig_inputs_global_vec_manip`). Any new field that stores key indices or per-gut data has to be added to those walkers and to the matching validator.

## Code conventions

- **`nest!`** (nestify) declares the entire struct tree inline in `engine.rs`. `#[...]*` applies an attribute to every nested struct. New fields go there.
- **`#[duplicate_item]`** stamps out one method per row of a substitution table. That is why `v_multi_insert_set`, `f_multi_sustain_remove_dig` and friends exist but won't be found by grepping for `fn v_multi_insert_set`. Grep for the name without `fn`, or read the table header. To add a variant, add a row.
- **Typestate**: `Engine<Edit>` is mutable configuration; `Engine<Play>` is meant to be the validated, performable form. Edit methods are on `impl Engine<Edit>` only.
- **Edit methods never panic on bad arguments.** They return `false` and leave the engine untouched. New edit methods follow the same shape: `if valid { ...; true } else { false }`.
- **`better_default::Default`** is used instead of std's derive, for `#[default(...)]` field attributes.
- File and module names start with an underscore (`_edit`, `_gut`).
- Git history uses auto-generated `modified: path` commit messages; there is no convention to follow.

## Open issues

- The Play engine is unwritten (see Status). How `v_multi` and `f_multi` differ, and how holds, radio mode, and min/max pressed behave at play time, is the author's design to settle.
- `TemperamentType::intervals` is `todo!()` for `EqualTemperament` and `PrimeLimit`, and nothing converts an output index to a frequency.
- Runtime state (`pressed`, `i_mem`, `x_mem`) is stored in the definition structs and is serialized with them. Skipping it on save would break the per-gut length invariant on load, so it needs to move into a separate runtime struct when Play is designed.
- Hold buttons (`HoldTog`) have `name` / `description` fields but no setter.
- `#[repr(C)]` is applied to structs containing `Vec` and `String`; it does not make them FFI-safe.

## Things to keep in mind when building on this

- Keep the crate front-end agnostic: no egui, no windowing, no audio. Front ends convert their key events to `Key` and call in.
- Code that runs on an audio thread must not allocate, lock, or format strings. The definition structs are `Vec`-heavy and carry `String`s, so Play mode will need either a pre-sized runtime form or strict discipline about what it touches.
