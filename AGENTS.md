# qop-core

Core data model for **Qop** ("qwert-o-phone", working name): a configurable instrument emulator played from a computer keyboard. This crate holds the instrument-definition structures and the tuning data. It makes no sound, opens no window, and has no I/O. Upstream: https://github.com/JazzerThighs/qop-core

## Status — read this before assuming anything works

- **Edit side is written; Play side is not.** `Engine<Edit>` has a full set of mutation methods. Every file under `src/engine/_play/` is empty (0 bytes), so there is no code yet that turns key presses into notes.
- **Nothing is exported.** `mod engine` and `mod temperament` are private and the structs are `pub(crate)`. The only public item is the `QopPart<T>` trait, which has no implementors. A downstream crate cannot use `Engine` until visibility is opened up.
- **No binary, no examples, few tests.** `src/engine/_edit/_tests.rs` holds regression tests for the edit methods. `#![allow(dead_code)]` at the crate root hides the fact that almost nothing else is called.
- `Temperament::new` panics for `EqualTemperament` and `PrimeLimit` (`TemperamentType::intervals` is `todo!()`).

## Commands

```bash
cargo check
cargo test
```

Edition 2024. Every dependency in `Cargo.toml` is pinned to `"*"`; `Cargo.lock` is committed and is what actually fixes versions (winit 0.30.12, serde 1.0.219, better_default 1.0.5, nestify 0.3.3, duplicate 2.0.0). Don't run `cargo update` casually.

## Layout

| Path | Contents |
|---|---|
| `src/lib.rs` | `Edit` / `Play` typestate markers, `QopPart<T>` trait |
| `src/engine.rs` | The whole `Engine<Mode>` struct tree (one `nest!` block) and `Engine<Edit>::to_play()` |
| `src/engine/_edit.rs` | `NewTrait` + `NewEnginePartParams`: constructors that size per-gut vectors correctly |
| `src/engine/_edit/_dig_inputs.rs` | Key registry edits (insert / purge / swap / remap) and the index-bounds validator |
| `src/engine/_edit/_gut.rs` | Gut insert/remove, gut triggers, gut transpositions, delta-length validator |
| `src/engine/_edit/_holds.rs` | Sustain / sostenuto key assignment, for guts and for each set kind |
| `src/engine/_edit/_trnsp.rs` | `trnsp_all` / `trnsp_one` transposition edits on sets |
| `src/engine/_edit/_vfc_sets.rs` | Set, button, and combo insert/remove; delta edits; min/max pressed; radio mode |
| `src/engine/_edit/_tests.rs` | Regression tests for the edit methods (`#[cfg(test)]`) |
| `src/engine/_play/*` | Empty placeholders mirroring `_edit/` |
| `src/temperament.rs` | `Temperament<Mode>`: reference note, tuning Hz, octave factor, note names, interval list |
| `src/temperament/intervals.rs` | Harmonic helpers and `WIKI_MICROTONAL_INTERVALS` (~315 named ratios) |
| `src/temperament/scale_list/twelve_tet.rs` | `TWELVE_TET_SCALES`: 1012 named 12-TET scales, 5.8k lines of data. Grep it, don't read it. |
| `collected_texts.txt` | Raw ratio list the interval table was transcribed from |

## Data model

`Engine` describes one instrument. Terms as used in the code:

- **`dig_inputs: Vec<KeyCode>`** — registry of physical keys (winit `KeyCode`). Everything else refers to a key by its **index into this vec**, never by `KeyCode`.
- **Gut** — one sounding voice, like a string. Has `togs` (key indices that sound it), a base output `index_out: usize` and `extra_out: f64`, and `trnsp_gut` (key-triggered offsets for that gut alone).
- **`v_multi` / `f_multi`: `Vec<VFSet>`** — sets of buttons. Each `VFBtn` carries `i_delta` / `x_delta` vectors with **one entry per gut**: how far that button shifts each gut. The two fields share a type; the names suggest valve-like and fret-like sets, but nothing in the code yet says how they differ. That difference will live in the Play logic — ask the author rather than guess.
- **`c_multi: Vec<ComboSet>`** — buttons plus a table of `Combo`s. Each combo is a `Vec<bool>` pattern over the set's buttons with its own per-gut deltas, like a fingering chart.
- **Transpositions** — `Trnsp` (scalar, per gut), `MulTrnsp` (per-gut vectors). On sets, `trnsp_all` applies to the whole set and `trnsp_one` to a single button/combo.
- **Holds** — `HoldBtns { sustain, inv_sustain, sostenuto, inv_sostenuto }`, present on the gut layer and on every set.
- **`radio_mode`, `min_pressed`, `max_pressed`** — constraints on simultaneous presses, on the gut layer and per set.
- **`i_*` vs `x_*`** — `i` is an integer step (an index into the temperament's notes); `x` is a continuous `f64` offset. The engine's per-voice output is the pair `(usize, f64)`, which is what `QopPart` passes around.
- **`*_mem`, `pressed`** — runtime state stored inside the definition structs, intended for Play mode.

### Invariants

`to_play()` enforces two of these; the rest are only maintained by going through the edit methods.

1. Every key index (`togs`, `triggers`, hold `togs`) is `< dig_inputs.len()`. Checked by `check_digitalref_invariants`.
2. Every per-gut vector (`i_delta`, `x_delta`, `i_mem`, `x_mem`, including inside `trnsp_one` / `trnsp_all`) has length `guts.len()`. Checked by `check_multi_delta_lengths`.
3. `set.pressed.len() == set.buttons.len()`. **Not checked.**
4. `combo.combo.len() == combo_set.buttons.len()`. **Not checked.**
5. There is always at least one gut.

Adding or removing a gut must touch every set (`insert_gut` / `remove_gut`). Removing a key must renumber every index vec in the engine (`dig_inputs_global_vec_manip`). Any new field that stores key indices or per-gut data has to be added to those walkers and to the matching validator.

## Code conventions

- **`nest!`** (nestify) declares the entire struct tree inline in `engine.rs`. `#[...]*` applies an attribute to every nested struct. New fields go there.
- **`#[duplicate_item]`** stamps out one method per row of a substitution table. That is why `v_multi_insert_set`, `f_multi_sustain_remove_dig` and friends exist but won't be found by grepping for `fn v_multi_insert_set`. Grep for the name without `fn`, or read the table header. To add a variant, add a row.
- **Typestate**: `Engine<Edit>` is mutable configuration; `Engine<Play>` is meant to be the validated, performable form. Edit methods are on `impl Engine<Edit>` only.
- **Edit methods fail silently.** Out-of-range arguments are a no-op, not an error or panic. Keep that contract unless the author decides otherwise.
- **`better_default::Default`** is used instead of std's derive, for `#[default(...)]` field attributes.
- File and module names start with an underscore (`_edit`, `_gut`).
- Git history uses auto-generated `modified: path` commit messages; there is no convention to follow.

## Open issues

- `TemperamentType::intervals` is `todo!()` for `EqualTemperament` and `PrimeLimit`, so `Temperament::new` panics for those two.
- `to_play()` does not check invariants 3 and 4 (`pressed` and `combo` lengths).
- `scale_omnibus` is a dependency but is never imported.
- `#[repr(C)]` is applied to structs containing `Vec` and `String`; it does not make them FFI-safe.

## Things to keep in mind when building on this

- `winit` is pulled in only for `KeyCode`. A plugin or a WASM build would be better served by the crate owning its key type and converting at the edge.
- Runtime state lives inside `Vec`-heavy, `String`-carrying structs. Code that runs on an audio thread must not allocate, lock, or format strings, so Play mode will need either a pre-sized runtime form or strict discipline about what it touches.
- Engine definitions already derive `Serialize` / `Deserialize`, so instrument presets are nearly free once the types are public.
