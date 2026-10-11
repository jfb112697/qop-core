# qop-core

Core library for **Qop** ("qwert-o-phone", working name): a configurable instrument emulator played from a computer keyboard, a gamepad, MIDI, or any mix of them. This crate holds the instrument definition, the engine that plays it, and the tuning data. It makes no sound, opens no window, and talks to no device: it is a library that any front end (egui app, plugin, CLI, WASM) builds on, and it must stay free of UI, windowing, audio, and device dependencies. Upstream: https://github.com/JazzerThighs/qop-core

## Status

- **Edit side**: `Engine<Edit>` holds an instrument definition, with a full set of edit methods and read accessors.
- **Play side**: `Player` takes an `Engine<Play>`, receives key and analog input, and returns note events. It is covered by `tests/play.rs`.
- **Temperament**: `frequency(index, extra)` returns Hz and `note_name(index)` a label.
- **Not here**: sound, device I/O, a binary, examples. The `QopPart<T>` trait has no implementors.
- The play rules below were chosen by a contributor and are **proposals until the author confirms them**. See "Decisions that may be reversed".

## Commands

```bash
cargo check
cargo test
cargo test --features winit   # also tests the winit key conversion
```

Edition 2024. Dependencies are pinned to a major version in `Cargo.toml` and `Cargo.lock` is committed. `winit` is optional and off by default; the only thing it enables is `Key::from_winit`.

## Public API

Re-exported at the crate root: `Engine`, `Edit`, `Play`, `Player`, `NoteEvent`, `Voice`, `Key`, `Analog`, `Temperament`. Nested structs (`Gut`, `VFSet`, `ComboSet`, `AnalogTrigger`, ...) are under `qop_core::engine`.

```rust
let mut player = engine.to_play()?.into_player();      // Engine<Edit> -> Engine<Play> -> Player
for event in player.key_down(Key::KeyJ, None) { ... }  // None = no velocity; plays at 1.0
player.key_down(Key::MidiNote(60), Some(0.5));         // MIDI passes its own velocity, 0.0..=1.0
player.analog(Analog::PadRightTrigger, 0.8, now_secs); // normalized value plus a clock
player.key_up(Key::KeyJ);
// NoteEvent::On { gut, index, extra, velocity } | Change { gut, index, extra } | Off { gut }
let hz = temperament.frequency(index, extra);
```

- **Reading**: every definition field has a getter of the same name (`engine.guts()`, `gut.togs()`), generated in `src/engine/_access.rs`. `name` and `description` are plain `pub` fields. `player.voices()` gives what each gut is doing now.
- **Writing**: only through `Engine<Edit>` methods. Fields are `pub(crate)` on purpose so outside code cannot break the invariants. A new field needs a getter in `_access.rs` and, if editable, an edit method.
- **Edit methods return `bool`**: `true` if applied, `false` if rejected (index out of range, unknown key, constraint violated, key already registered). A rejected edit changes nothing, and none of them panic.
- `Engine<Edit>::validate()` runs every invariant check and `to_play()` calls it. Call `validate()` on anything deserialized before editing it. There is no Play-to-Edit conversion: keep the `Engine<Edit>` and call `to_play()` again.
- Each input method returns the events it caused, in gut order, as a slice that is valid until the next call. `all_off()` releases everything.
- Engines derive `Serialize` / `Deserialize`; the crate picks no file format. Presets contain the definition only.

## Layout

| Path | Contents |
|---|---|
| `src/lib.rs` | `Edit` / `Play` typestate markers, `QopPart<T>` trait, root re-exports |
| `src/key.rs` | `Key` (digital inputs) and `Analog` (continuous inputs), plus the optional winit conversion |
| `src/engine.rs` | The whole `Engine<Mode>` struct tree (one `nest!` block), `validate()`, `to_play()` |
| `src/engine/_access.rs` | Read-only getters for every struct in the tree |
| `src/engine/_edit.rs` | `NewTrait` + `NewEnginePartParams`: constructors that size per-gut vectors correctly |
| `src/engine/_edit/_dig_inputs.rs` | Key registry edits (insert / purge / swap / remap) and the index-bounds validator |
| `src/engine/_edit/_analog.rs` | Analog registry, analog triggers, analog mods, and their validator |
| `src/engine/_edit/_gut.rs` | Gut insert/remove, triggers, transpositions, names, flags, length validators |
| `src/engine/_edit/_holds.rs` | Sustain / sostenuto key assignment and names, for guts and for each set kind |
| `src/engine/_edit/_trnsp.rs` | `trnsp_all` / `trnsp_one` transposition edits on sets |
| `src/engine/_edit/_vfc_sets.rs` | Set, button, and combo insert/remove; delta edits; combo patterns; names; min/max pressed; radio mode |
| `src/engine/_play.rs` | `Player`, `NoteEvent`, `Voice`, the input methods, and `refresh()` which drives everything else |
| `src/engine/_play/_holds.rs` | `Layer`: pressed/active state for one group, holds, press limits, `LIMIT_POLICY` |
| `src/engine/_play/_trnsp.rs` | Momentary and latching transposition state |
| `src/engine/_play/_vfc_sets.rs` | Per-set state and the three ways a set contributes pitch (summed, highest, matching) |
| `src/engine/_play/_gut.rs` | Pitch of one gut, and turning state changes into `NoteEvent`s |
| `src/engine/_play/_analog.rs` | Analog trigger: value crossing thresholds becomes a key press with velocity |
| `src/engine/_play/_dig_inputs.rs` | Key and analog lookup helpers |
| `src/temperament.rs` | `Temperament<Mode>`: tuning data, `frequency`, `note_name` |
| `src/temperament/intervals.rs` | Harmonic helpers and `WIKI_MICROTONAL_INTERVALS` (~315 named ratios) |
| `src/temperament/scale_list/twelve_tet.rs` | `TWELVE_TET_SCALES`: 1012 named 12-TET scales, 5.8k lines of data. Grep it, don't read it. |
| `tests/public_api.rs` | Building, reading, and saving an instrument through the public API |
| `tests/play.rs` | One test per play rule, through the public API |
| `tests/no_alloc.rs` | Counts heap allocations while playing; must stay at zero |
| `src/engine/_edit/_tests.rs`, `src/engine/_play/_tests.rs` | Crate-internal tests |
| `collected_texts.txt` | Raw ratio list the interval table was transcribed from |

## Data model

`Engine` describes one instrument. Terms as used in the code:

- **`dig_inputs: Vec<Key>`** — registry of on/off inputs: keyboard keys (W3C `code` names), gamepad buttons (`Key::PadSouth`, ...), `Key::MidiNote(n)`, and `Key::Other(n)` for anything else. Everything else refers to an input by its **index into this vec**, never by `Key`.
- **`analog_inputs: Vec<Analog>`** — registry of continuous inputs: gamepad sticks and triggers, MIDI CC, pitch bend, channel pressure, `Analog::Other(n)`. Referred to by index too. The front end normalizes values: `0.0..=1.0` for triggers, CCs and pressure, `-1.0..=1.0` for sticks and pitch bend.
- **Gut** — one sounding voice, like a string. Has `togs` (key indices that sound it), a base output `index_out: i32` and `extra_out: f64`, `trnsp_gut` (transpositions for that gut alone), and `retrigger`.
- **`v_multi` / `f_multi`: `Vec<VFSet>`** — sets of buttons. Each `VFBtn` carries `i_delta` / `x_delta` vectors with **one entry per gut**: how far that button shifts each gut. The two fields share a type and differ in how pressed buttons combine (see Play rules).
- **`c_multi: Vec<ComboSet>`** — buttons plus a table of `Combo`s. Each combo is a `Vec<bool>` pattern over the set's buttons with its own per-gut deltas, like a fingering chart.
- **Transpositions** — `Trnsp` (scalar, per gut), `MulTrnsp` (per-gut vectors), each with `triggers` and a `latch` flag. On sets, `trnsp_all` applies to the whole set and `trnsp_one` to a single button/combo.
- **Holds** — `HoldBtns { sustain, inv_sustain, sostenuto, inv_sostenuto }`, present on the gut layer and on every set.
- **`radio_mode`, `min_pressed`, `max_pressed`** — limits on simultaneous presses, on the gut layer and per set. `max_pressed == 0` means no limit.
- **`AnalogTrigger`** — reads one analog input and presses its `togs` keys when the value rises past `high`, with a velocity from how fast it got there from `low`. Releases them when it falls below `low`.
- **`AnalogMod`** — adds `value * x_delta[gut]` to each gut's `extra` (pitch bend, vibrato).
- **`i_*` vs `x_*`** — `i` is an integer step (an index into the temperament's notes); `x` is a continuous `f64` offset in the same unit. The engine's per-voice output is the pair `(i32, f64)`.

All state that changes while playing lives in `Player`, never in the definition.

### Invariants

`validate()` (and therefore `to_play()`) checks all of these except 5, which the edit methods maintain.

1. Every key index (`togs`, `triggers`, hold `togs`, analog trigger `togs`) is `< dig_inputs.len()`.
2. Every per-gut vector (`i_delta`, `x_delta`, including inside `trnsp_one` / `trnsp_all` and `AnalogMod`) has length `guts.len()`.
3. `combo.combo.len() == combo_set.buttons.len()`.
4. Every analog `input` index is `< analog_inputs.len()`; a trigger has `low < high` and `fast_secs < slow_secs`.
5. There is always at least one gut.

Adding or removing a gut must touch every per-gut vector (`insert_gut` / `remove_gut`, and `analog_mods`). Removing a key must renumber every index vec (`dig_inputs_global_vec_manip`). Any new field that stores key indices or per-gut data has to be added to those walkers and to the matching validator.

## Play rules

Each has a test of the same intent in `tests/play.rs`.

1. **Combining buttons in a set.** `v_multi`: the deltas of all active buttons add up (valves). `f_multi`: per gut, only the active button with the highest `i_delta` for that gut applies (frets); ties go to the larger `x_delta`, then the later button. A set's `trnsp_all` always applies while active; a button's `trnsp_one` only while that button's delta is applying.
2. **Combos.** The first combo whose pattern equals the set's active buttons exactly applies. No match means no delta.
3. **Sounding.** A gut sounds while it is active on the gut layer. When its pitch changes while sounding, `retrigger == false` emits `Change` (slide) and `retrigger == true` emits `Off` then `On`.
4. **Holds.** On the gut layer they hold notes; on a set they hold buttons. Sustain keeps every active member active after release. Sostenuto keeps only the members that were active when it engaged. An inverse hold with keys assigned is engaged while its keys are *up*.
5. **Press limits.** Past `max_pressed`, the newest press wins and the oldest active member is released; it stays out until pressed again. Below `min_pressed`, the most recently released members stay latched. Radio mode is max = min = 1.
6. **Transpositions.** `latch == false`: active while a trigger is held. `latch == true`: each press flips it. Active transpositions add.
7. **Several keys on one member.** Any one of them presses it.
8. **Global switches.** `index_delta_bool == false`: every gut plays its base `index_out`. `extra_delta_bool == false`: `extra` is always `0.0` (no microtones).
9. **Signed index.** Deltas can take an index below zero. `Temperament` handles negative indices.
10. **Velocity.** A press that carries a velocity (MIDI) uses it. An `AnalogTrigger` computes one. Anything else plays at `1.0`. A gut keeps the velocity of the press that last triggered it.

`Player` must not allocate after construction; `tests/no_alloc.rs` enforces this. All state vectors are sized in `Player::new`, and the event buffer is sized for the worst case there.

## Decisions that may be reversed

- **Newest press wins at a limit (rule 5).** To switch to "ignore new presses at the limit", change the constant `LIMIT_POLICY` in `src/engine/_play/_holds.rs` to `LimitPolicy::IgnoreNew`. Both policies are implemented in `Layer::resolve` and tested in `src/engine/_play/_tests.rs`. Then update `the_newest_press_wins_at_the_limit` in `tests/play.rs`, which expects the current policy. To make it a per-instrument choice instead, add a field to `Engine` and pass it where `LIMIT_POLICY` is passed (`_play.rs` and `_play/_vfc_sets.rs`).
- **`PrimeLimit(p)` is only a label.** It does not generate a scale, because a prime limit alone does not pick one; the caller passes `intervals` to `Temperament::new`. To make it generate one, add a `PrimeLimit` arm to `TemperamentType::intervals()` in `src/temperament.rs` that returns the scale in interval units (for a ratio `r`: `len * log(r) / log(octave_scalar_factor)`). `new` already replaces `intervals` with whatever that function returns, as it does for `EqualTemperament`. `WIKI_MICROTONAL_INTERVALS` has the ratios.
- **v sets sum, f sets take the highest (rule 1).** The choice is made in `Player::pitch` (`_play/_gut.rs`), which calls `add_summed` for `v_multi` and `add_highest` for `f_multi`.
- **Inverse holds, min-pressed latching, and the velocity curve** are the least certain readings of the model; each is one function (`Layer::update_holds`, `Layer::resolve`, `AnalogTrigger::velocity`).

## Temperament units

Assumptions read off the defaults (12 notes, `intervals` = 0.0 to 11.0, reference note 69 at 440 Hz), not yet confirmed by the author.

- `intervals[k]` is the position of note class `k` inside one octave, in steps of `1 / intervals.len()` of that octave on a log scale. Twelve equal notes are `0.0..=11.0`, and a just fifth in a 12-note scale is about `7.02`.
- An index counts note classes upward: `index.div_euclid(len)` is the octave, `index.rem_euclid(len)` the note class. `reference_note` sounds at `tuning_hz`, and one octave multiplies frequency by `octave_scalar_factor`.
- `extra` is in the same step unit, so `frequency(69, 1.0) == frequency(70, 0.0)` in 12-TET.
- `octave_label` is the printed octave number of the octave containing `reference_note` (69 is "A4").
- `EqualTemperament(divisions, octave_factor)` generates `0.0..divisions` and sets the octave factor.

## Code conventions

- **`nest!`** (nestify) declares the entire struct tree inline in `engine.rs`. `#[...]*` applies an attribute to every nested struct. New fields go there.
- **`#[duplicate_item]`** stamps out one method per row of a substitution table. That is why `v_multi_insert_set`, `f_multi_sustain_remove_dig` and friends exist but won't be found by grepping for `fn v_multi_insert_set`. Grep for the name without `fn`, or read the table header. To add a variant, add a row.
- **Typestate**: `Engine<Edit>` is mutable configuration; `Engine<Play>` is the validated form a `Player` runs. Edit methods are on `impl Engine<Edit>` only.
- **Edit methods** have the shape `if valid { ...; true } else { false }`.
- **`better_default::Default`** is used instead of std's derive, for `#[default(...)]` field attributes. It is glob-imported from `engine`, so a file that derives the std `Default` needs `use std::default::Default;`.
- File and module names start with an underscore (`_edit`, `_gut`). `_play/` mirrors `_edit/`.
- Git history uses auto-generated `modified: path` commit messages; there is no convention to follow.

## Open issues

- Several gamepads or MIDI channels at once are not distinguished: `Key` and `Analog` carry no device or channel id. The front end has to route, or use `Other(n)`.
- An analog input can only pluck (through a trigger) or bend `extra` (through a mod). It cannot scale a button's delta or drive velocity continuously (aftertouch).
- A stolen member (rule 5) does not come back when the newer one is released.
- `Temperament` has no way to change `intervals`, the note names, or the type after `new`.
- The `QopPart<T>` trait is unused; `Player` and `Temperament::frequency` do what it describes.
