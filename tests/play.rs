// Plays instruments through the public API and checks the note events that come back.
use qop_core::NoteEvent::{Change, Off, On};
use qop_core::{Analog, Edit, Engine, Key, Player, Temperament};

// Key indices in the order `base` registers them.
const J: usize = 0;
const K: usize = 1;
const D1: usize = 2;
const D2: usize = 3;
const A: usize = 4;
const S: usize = 5;
const SPACE: usize = 6;
const BRACKET: usize = 7;
const Z: usize = 8;
const X: usize = 9;

// Two guts: gut 0 at index 40 on J, gut 1 at index 45 on K.
fn base() -> Engine<Edit> {
    let mut e: Engine<Edit> = Engine::default();
    for key in [
        Key::KeyJ, Key::KeyK, Key::Digit1, Key::Digit2, Key::KeyA, Key::KeyS,
        Key::Space, Key::BracketRight, Key::KeyZ, Key::KeyX,
    ] {
        assert!(e.dig_inputs_insert_k(key));
    }
    assert!(e.gut_insert_g(1));
    assert!(e.gut_change_index_delta_out(0, 40));
    assert!(e.gut_change_index_delta_out(1, 45));
    assert!(e.gut_insert_dig(0, J));
    assert!(e.gut_insert_dig(1, K));
    e
}

// Adds an f set with Digit1 = +1 and Digit2 = +2 on both guts.
fn with_frets(mut e: Engine<Edit>) -> Engine<Edit> {
    assert!(e.f_multi_insert_set(0));
    assert!(e.f_multi_insert_btn(0, 1));
    assert!(e.f_multi_insert_btn_dig(0, 0, D1));
    assert!(e.f_multi_insert_btn_dig(0, 1, D2));
    assert!(e.f_multi_change_i_deltas(0, 0, vec![Some(1), Some(1)]));
    assert!(e.f_multi_change_i_deltas(0, 1, vec![Some(2), Some(2)]));
    e
}

fn player(e: &Engine<Edit>) -> Player {
    e.to_play().unwrap().into_player()
}

fn on(gut: usize, index: i32) -> qop_core::NoteEvent {
    On { gut, index, extra: 0.0, velocity: 1.0 }
}

fn change(gut: usize, index: i32) -> qop_core::NoteEvent {
    Change { gut, index, extra: 0.0 }
}

#[test]
fn a_gut_sounds_while_its_key_is_held() {
    let mut p = player(&base());
    assert_eq!(p.key_down(Key::KeyJ, None), &[on(0, 40)]);
    assert_eq!(p.key_down(Key::KeyJ, None), &[]);
    assert!(p.voices()[0].sounding);
    assert_eq!(p.key_up(Key::KeyJ), &[Off { gut: 0 }]);
    assert_eq!(p.key_down(Key::KeyQ, None), &[]);
}

#[test]
fn v_set_buttons_add_up() {
    let mut e = base();
    assert!(e.v_multi_insert_set(0));
    assert!(e.v_multi_insert_btn(0, 1));
    assert!(e.v_multi_insert_btn_dig(0, 0, A));
    assert!(e.v_multi_insert_btn_dig(0, 1, S));
    assert!(e.v_multi_change_i_deltas(0, 0, vec![Some(-2), Some(-2)]));
    assert!(e.v_multi_change_i_deltas(0, 1, vec![Some(-1), Some(-1)]));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    assert_eq!(p.key_down(Key::KeyA, None), &[change(0, 38)]);
    assert_eq!(p.key_down(Key::KeyS, None), &[change(0, 37)]);
    assert_eq!(p.key_up(Key::KeyA), &[change(0, 39)]);
}

#[test]
fn f_set_uses_only_the_highest_button() {
    let mut p = player(&with_frets(base()));
    p.key_down(Key::KeyJ, None);
    assert_eq!(p.key_down(Key::Digit2, None), &[change(0, 42)]);
    assert_eq!(p.key_down(Key::Digit1, None), &[]);
    assert_eq!(p.key_up(Key::Digit2), &[change(0, 41)]);
    assert_eq!(p.key_up(Key::Digit1), &[change(0, 40)]);
}

#[test]
fn combos_apply_only_on_an_exact_match() {
    let mut e = base();
    assert!(e.c_multi_insert_set(0));
    assert!(e.c_multi_insert_btn(0, 1));
    assert!(e.c_multi_insert_btn_dig(0, 0, Z));
    assert!(e.c_multi_insert_btn_dig(0, 1, X));
    assert!(e.c_multi_insert_combo(0, 1));
    assert!(e.c_multi_change_combo(0, 0, &[true, false]));
    assert!(e.c_multi_change_combo(0, 1, &[true, true]));
    assert!(e.c_multi_change_i_deltas(0, 0, vec![Some(5), Some(5)]));
    assert!(e.c_multi_change_i_deltas(0, 1, vec![Some(7), Some(7)]));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    assert_eq!(p.key_down(Key::KeyZ, None), &[change(0, 45)]);
    assert_eq!(p.key_down(Key::KeyX, None), &[change(0, 47)]);
    // Only X is down now, and no combo has that pattern.
    assert_eq!(p.key_up(Key::KeyZ), &[change(0, 40)]);
}

#[test]
fn a_gut_slides_or_retriggers_according_to_its_flag() {
    let mut e = with_frets(base());
    assert!(e.gut_set_retrigger(1, true));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    p.key_down(Key::KeyK, None);
    assert_eq!(p.key_down(Key::Digit1, None), &[change(0, 41), Off { gut: 1 }, on(1, 46)]);
}

#[test]
fn gut_sustain_keeps_notes_sounding() {
    let mut e = base();
    assert!(e.gut_sustain_insert_dig(SPACE));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    p.key_down(Key::Space, None);
    assert_eq!(p.key_up(Key::KeyJ), &[]);
    assert_eq!(p.key_down(Key::KeyK, None), &[on(1, 45)]);
    assert_eq!(p.key_up(Key::KeyK), &[]);
    assert_eq!(p.key_up(Key::Space), &[Off { gut: 0 }, Off { gut: 1 }]);
}

#[test]
fn gut_sostenuto_keeps_only_notes_sounding_when_it_was_pressed() {
    let mut e = base();
    assert!(e.gut_sostenuto_insert_dig(SPACE));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    p.key_down(Key::Space, None);
    p.key_down(Key::KeyK, None);
    assert_eq!(p.key_up(Key::KeyJ), &[]);
    assert_eq!(p.key_up(Key::KeyK), &[Off { gut: 1 }]);
    assert_eq!(p.key_up(Key::Space), &[Off { gut: 0 }]);
}

#[test]
fn inverse_sustain_holds_notes_until_its_key_is_pressed() {
    let mut e = base();
    assert!(e.gut_inv_sustain_insert_dig(SPACE));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    assert_eq!(p.key_up(Key::KeyJ), &[]);
    assert_eq!(p.key_down(Key::Space, None), &[Off { gut: 0 }]);
}

#[test]
fn set_sustain_keeps_buttons_pressed() {
    let mut e = with_frets(base());
    assert!(e.f_multi_sustain_insert_dig(0, SPACE));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    p.key_down(Key::Digit2, None);
    p.key_down(Key::Space, None);
    assert_eq!(p.key_up(Key::Digit2), &[]);
    assert_eq!(p.voices()[0].index, 42);
    assert_eq!(p.key_up(Key::Space), &[change(0, 40)]);
}

#[test]
fn the_newest_press_wins_at_the_limit() {
    let mut e = base();
    assert!(e.gut_change_max_pressed(1));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    assert_eq!(p.key_down(Key::KeyK, None), &[Off { gut: 0 }, on(1, 45)]);
    // J is still held but was pushed out; it stays out until pressed again.
    assert_eq!(p.key_up(Key::KeyK), &[Off { gut: 1 }]);
    assert!(!p.voices()[0].sounding);
}

#[test]
fn radio_mode_keeps_exactly_one_member_active() {
    let mut e = base();
    assert!(e.gut_toggle_radio_mode());
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    assert_eq!(p.key_up(Key::KeyJ), &[]);
    assert_eq!(p.key_down(Key::KeyK, None), &[Off { gut: 0 }, on(1, 45)]);
}

#[test]
fn min_pressed_keeps_the_last_member_latched() {
    let mut e = base();
    assert!(e.gut_change_max_pressed(2));
    assert!(e.gut_change_min_pressed(1));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    assert_eq!(p.key_up(Key::KeyJ), &[]);
    assert_eq!(p.key_down(Key::KeyK, None), &[Off { gut: 0 }, on(1, 45)]);
}

#[test]
fn transpositions_are_momentary_or_latching() {
    let mut e = base();
    assert!(e.gut_insert_trnsp_t(0, 0));
    assert!(e.gut_insert_trnsp_dig(0, 0, BRACKET));
    assert!(e.gut_trnsp_change_i_deltas(0, 0, 12));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    assert_eq!(p.key_down(Key::BracketRight, None), &[change(0, 52)]);
    assert_eq!(p.key_up(Key::BracketRight), &[change(0, 40)]);

    assert!(e.gut_trnsp_set_latch(0, 0, true));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    assert_eq!(p.key_down(Key::BracketRight, None), &[change(0, 52)]);
    assert_eq!(p.key_up(Key::BracketRight), &[]);
    assert_eq!(p.key_down(Key::BracketRight, None), &[change(0, 40)]);
}

#[test]
fn set_transpositions_shift_every_gut_they_name() {
    let mut e = with_frets(base());
    assert!(e.f_multi_insert_trnsp_all_t(0, 0));
    assert!(e.f_multi_insert_trnsp_all_dig(0, 0, BRACKET));
    assert!(e.f_multi_trnsp_all_change_i_deltas(0, 0, vec![Some(12), None]));
    assert!(e.f_multi_insert_trnsp_one_t(0, 0, 0));
    assert!(e.f_multi_insert_trnsp_one_dig(0, 0, 0, A));
    assert!(e.f_multi_trnsp_one_change_i_deltas(0, 0, 0, vec![Some(7), Some(7)]));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    p.key_down(Key::KeyK, None);
    assert_eq!(p.key_down(Key::BracketRight, None), &[change(0, 52)]);
    // The per-button transposition only counts while its button is the one applying.
    assert_eq!(p.key_down(Key::KeyA, None), &[]);
    assert_eq!(p.key_down(Key::Digit1, None), &[change(0, 60), change(1, 53)]);
}

#[test]
fn a_button_fires_on_any_of_its_keys() {
    let mut e = base();
    assert!(e.gut_insert_dig(0, A));
    let mut p = player(&e);
    assert_eq!(p.key_down(Key::KeyJ, None), &[on(0, 40)]);
    assert_eq!(p.key_down(Key::KeyA, None), &[]);
    assert_eq!(p.key_up(Key::KeyJ), &[]);
    assert_eq!(p.key_up(Key::KeyA), &[Off { gut: 0 }]);
}

#[test]
fn delta_switches_turn_off_index_or_extra_changes() {
    let mut e = with_frets(base());
    assert!(e.gut_change_extra_delta_out(0, 0.5));
    let mut p = player(&e);
    assert_eq!(p.key_down(Key::KeyJ, None), &[On { gut: 0, index: 40, extra: 0.5, velocity: 1.0 }]);

    assert!(e.set_index_delta_bool(false));
    assert!(e.set_extra_delta_bool(false));
    let mut p = player(&e);
    assert_eq!(p.key_down(Key::KeyJ, None), &[on(0, 40)]);
    assert_eq!(p.key_down(Key::Digit2, None), &[]);
}

#[test]
fn an_index_can_go_below_zero_and_still_has_a_pitch() {
    let mut e = base();
    assert!(e.gut_change_index_delta_out(0, 2));
    assert!(e.v_multi_insert_set(0));
    assert!(e.v_multi_insert_btn_dig(0, 0, A));
    assert!(e.v_multi_change_i_deltas(0, 0, vec![Some(-5), Some(-5)]));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    assert_eq!(p.key_down(Key::KeyA, None), &[change(0, -3)]);

    let t = Temperament::new(None, None, None, None, None, None, None, None, None);
    let expected = 440.0 * 2f64.powf((-3.0 - 69.0) / 12.0);
    assert!((t.frequency(-3, 0.0).unwrap() - expected).abs() < 1e-9);
    assert!((t.frequency(-12, 0.0).unwrap() * 2.0 - t.frequency(0, 0.0).unwrap()).abs() < 1e-9);
    assert_eq!(t.note_name(-1).unwrap(), "B-2");
}

#[test]
fn keyboard_keys_play_at_full_velocity_and_midi_passes_its_own() {
    let mut e = base();
    assert!(e.dig_inputs_insert_k(Key::MidiNote(60)));
    let midi = e.dig_inputs().len() - 1;
    assert!(e.gut_insert_dig(1, midi));
    let mut p = player(&e);
    assert_eq!(p.key_down(Key::KeyJ, None), &[on(0, 40)]);
    assert_eq!(
        p.key_down(Key::MidiNote(60), Some(0.5)),
        &[On { gut: 1, index: 45, extra: 0.0, velocity: 0.5 }]
    );
    assert_eq!(p.voices()[1].velocity, 0.5);
}

// Gut 0 can also be plucked by the right trigger of a gamepad.
fn with_pad_trigger(mut e: Engine<Edit>) -> Engine<Edit> {
    assert!(e.dig_inputs_insert_k(Key::PadRightTrigger));
    let pad = e.dig_inputs().len() - 1;
    assert!(e.gut_insert_dig(0, pad));
    assert!(e.analog_inputs_insert(Analog::PadRightTrigger));
    assert!(e.analog_trigger_insert(0, 0));
    assert!(e.analog_trigger_insert_dig(0, pad));
    e
}

fn velocity_of(events: &[qop_core::NoteEvent]) -> f64 {
    match events {
        [On { velocity, .. }] => *velocity,
        other => panic!("expected one On event, got {other:?}"),
    }
}

#[test]
fn an_analog_trigger_plucks_with_a_velocity_from_its_speed() {
    let mut p = player(&with_pad_trigger(base()));

    // Straight to the top: full velocity.
    assert_eq!(velocity_of(p.analog(Analog::PadRightTrigger, 1.0, 0.0)), 1.0);
    assert_eq!(p.analog(Analog::PadRightTrigger, 0.9, 0.1), &[]);
    assert_eq!(p.analog(Analog::PadRightTrigger, 0.0, 0.2), &[Off { gut: 0 }]);

    // A slower pull is quieter.
    assert_eq!(p.analog(Analog::PadRightTrigger, 0.2, 1.0), &[]);
    let medium = velocity_of(p.analog(Analog::PadRightTrigger, 0.7, 1.1));
    assert!(0.0 < medium && medium < 1.0);
    p.analog(Analog::PadRightTrigger, 0.0, 1.5);

    // A very slow pull still sounds, at the quietest velocity.
    p.analog(Analog::PadRightTrigger, 0.2, 2.0);
    let slow = velocity_of(p.analog(Analog::PadRightTrigger, 0.7, 5.0));
    assert!(0.0 < slow && slow < medium);
}

#[test]
fn a_keyboard_and_a_gamepad_play_the_same_instrument_together() {
    let mut p = player(&with_frets(with_pad_trigger(base())));
    p.key_down(Key::Digit2, None);
    let events = p.analog(Analog::PadRightTrigger, 1.0, 0.0).to_vec();
    assert_eq!(events, vec![on(0, 42)]);
    assert_eq!(p.key_down(Key::KeyK, None), &[on(1, 47)]);
}

#[test]
fn an_analog_mod_bends_extra() {
    let mut e = base();
    assert!(e.analog_inputs_insert(Analog::MidiPitchBend));
    assert!(e.analog_mod_insert(0, 0));
    assert!(e.analog_mod_change_x_deltas(0, vec![Some(2.0), None]));
    let mut p = player(&e);
    p.key_down(Key::KeyJ, None);
    p.key_down(Key::KeyK, None);
    assert_eq!(p.analog(Analog::MidiPitchBend, 0.5, 0.0), &[Change { gut: 0, index: 40, extra: 1.0 }]);
    assert_eq!(p.analog(Analog::MidiPitchBend, -1.0, 0.1), &[Change { gut: 0, index: 40, extra: -2.0 }]);
    assert_eq!(p.analog(Analog::MidiCc(1), 1.0, 0.2), &[]);
}

#[test]
fn analog_definitions_follow_gut_and_input_edits() {
    let mut e = with_pad_trigger(base());
    assert!(e.analog_inputs_insert(Analog::MidiPitchBend));
    assert!(e.analog_mod_insert(0, 1));
    assert!(e.gut_insert_g(2));
    assert_eq!(e.analog_mods()[0].x_delta().len(), 3);
    assert!(e.validate().is_ok());

    // Removing the first analog input drops its trigger and renumbers the mod.
    assert!(e.analog_inputs_purge(Analog::PadRightTrigger));
    assert!(e.analog_triggers().is_empty());
    assert_eq!(e.analog_mods()[0].input(), 0);
    assert!(e.validate().is_ok());

    assert!(!e.analog_trigger_insert(0, 5));
    assert!(e.analog_trigger_insert(0, 0));
    assert!(!e.analog_trigger_set_thresholds(0, 0.8, 0.2));
    assert!(e.analog_trigger_set_thresholds(0, 0.2, 0.8));
}

#[test]
fn all_off_releases_held_and_latched_state() {
    let mut e = base();
    assert!(e.gut_inv_sustain_insert_dig(SPACE));
    assert!(e.gut_insert_trnsp_t(0, 0));
    assert!(e.gut_insert_trnsp_dig(0, 0, BRACKET));
    assert!(e.gut_trnsp_change_i_deltas(0, 0, 12));
    assert!(e.gut_trnsp_set_latch(0, 0, true));
    let mut p = player(&e);
    p.key_down(Key::BracketRight, None);
    p.key_up(Key::BracketRight);
    p.key_down(Key::KeyJ, None);
    assert_eq!(p.voices()[0].index, 52);
    assert_eq!(p.all_off(), &[Off { gut: 0 }]);
    assert_eq!(p.key_down(Key::KeyJ, None), &[on(0, 40)]);
}

#[test]
fn a_played_note_has_a_frequency_and_presets_round_trip() {
    let e = with_frets(with_pad_trigger(base()));
    let json = serde_json::to_string(&e).unwrap();
    let back: Engine<Edit> = serde_json::from_str(&json).unwrap();
    assert!(back.validate().is_ok());

    let t = Temperament::new(None, None, None, None, None, None, None, None, None);
    let mut p = player(&back);
    p.key_down(Key::Digit2, None);
    match p.key_down(Key::KeyK, None) {
        [On { index, extra, .. }] => {
            // Gut 1 is index 45 (A2) and the second fret raises it to B2.
            assert_eq!(t.note_name(*index).unwrap(), "B2");
            assert!((t.frequency(*index, *extra).unwrap() - 123.4708).abs() < 1e-3);
        }
        other => panic!("expected one On event, got {other:?}"),
    }
}
