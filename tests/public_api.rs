// Exercises the crate the way an application would: only through the public API.
use qop_core::{Edit, Engine, Play};
use qop_core::{Key, Temperament};

fn two_string_instrument() -> Engine<Edit> {
    let mut e: Engine<Edit> = Engine::default();
    for key in [Key::KeyJ, Key::KeyK, Key::Digit1, Key::Digit2, Key::Space] {
        assert!(e.dig_inputs_insert_k(key));
    }
    assert!(e.gut_insert_g(1));
    assert!(e.gut_set_name(0, "Low".to_string()));
    assert!(e.gut_change_index_delta_out(0, 40));
    assert!(e.gut_change_index_delta_out(1, 45));
    assert!(e.gut_insert_dig(0, 0));
    assert!(e.gut_insert_dig(1, 1));
    assert!(e.gut_sustain_insert_dig(4));

    assert!(e.f_multi_insert_set(0));
    assert!(e.f_multi_set_name(0, "Frets".to_string()));
    assert!(e.f_multi_insert_btn(0, 1));
    assert!(e.f_multi_insert_btn_dig(0, 0, 2));
    assert!(e.f_multi_insert_btn_dig(0, 1, 3));
    assert!(e.f_multi_change_i_deltas(0, 0, vec![Some(1), Some(1)]));
    assert!(e.f_multi_change_i_deltas(0, 1, vec![Some(2), Some(2)]));
    e
}

#[test]
fn build_read_and_convert_an_instrument() {
    let e = two_string_instrument();
    assert_eq!(e.dig_inputs().len(), 5);
    assert_eq!(e.guts().len(), 2);
    assert_eq!(e.guts()[0].name, "Low");
    assert_eq!(e.guts()[1].index_out(), 45);
    assert_eq!(e.guts()[1].togs(), &[1]);
    assert_eq!(e.gut_holds().sustain().togs(), &[4]);
    assert_eq!(e.f_multi()[0].name, "Frets");
    assert_eq!(e.f_multi()[0].buttons()[1].i_delta(), &[2, 2]);

    assert!(e.validate().is_ok());
    let p: Engine<Play> = e.to_play().unwrap();
    assert_eq!(p.guts().len(), 2);
}

#[test]
fn rejected_edits_return_false_and_change_nothing() {
    let mut e = two_string_instrument();
    assert!(!e.dig_inputs_insert_k(Key::KeyJ));
    assert!(!e.gut_insert_dig(7, 0));
    assert!(!e.gut_insert_dig(0, 99));
    assert!(!e.f_multi_insert_btn(3, 0));
    assert!(!e.f_multi_change_i_deltas(0, 0, vec![Some(1)]));
    assert!(!e.dig_inputs_purge_dig(Key::KeyZ));
    assert_eq!(e.guts()[0].togs(), &[0]);
    assert_eq!(e.f_multi()[0].buttons()[0].i_delta(), &[1, 1]);
}

#[test]
fn combo_patterns_can_be_set() {
    let mut e = two_string_instrument();
    assert!(e.c_multi_insert_set(0));
    assert!(e.c_multi_insert_btn(0, 1));
    assert!(e.c_multi_combo_set_name(0, 0, "Both".to_string()));
    assert!(e.c_multi_change_combo(0, 0, &[true, true]));
    assert!(!e.c_multi_change_combo(0, 0, &[true]));
    assert_eq!(e.c_multi()[0].combos()[0].combo(), &[true, true]);
    assert_eq!(e.c_multi()[0].combos()[0].name, "Both");
    assert!(e.validate().is_ok());
}

#[test]
fn instrument_survives_a_json_round_trip() {
    let e = two_string_instrument();
    let json = serde_json::to_string(&e).unwrap();
    let back: Engine<Edit> = serde_json::from_str(&json).unwrap();
    assert_eq!(back.dig_inputs(), e.dig_inputs());
    assert_eq!(back.guts()[0].name, "Low");
    assert_eq!(back.f_multi()[0].buttons()[1].i_delta(), &[2, 2]);
    assert!(back.validate().is_ok());
}

#[test]
fn key_other_covers_inputs_outside_the_named_set() {
    let mut e: Engine<Edit> = Engine::default();
    assert!(e.dig_inputs_insert_k(Key::Other(60)));
    assert_eq!(e.dig_inputs(), &[Key::Other(60)]);
}

#[test]
fn temperament_defaults_are_readable() {
    let t = Temperament::new(None, None, None, None, None, None, None, None, None);
    assert_eq!(t.reference_note(), 69);
    assert_eq!(t.tuning_hz(), 440.0);
    assert_eq!(t.octave_scalar_factor(), 2.0);
    assert_eq!(t.note_class_set().len(), 12);
    assert_eq!(t.intervals().len(), 12);
}

#[cfg(feature = "winit")]
#[test]
fn winit_key_codes_convert() {
    use winit::keyboard::KeyCode;
    assert_eq!(Key::from_winit(KeyCode::KeyJ), Some(Key::KeyJ));
    assert_eq!(Key::from_winit(KeyCode::Semicolon), Some(Key::Semicolon));
    assert_eq!(Key::from_winit(KeyCode::F24), None);
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

#[test]
fn default_temperament_is_twelve_tet_at_a440() {
    let t = Temperament::new(None, None, None, None, None, None, None, None, None);
    assert!(close(t.frequency(69, 0.0).unwrap(), 440.0));
    assert!(close(t.frequency(81, 0.0).unwrap(), 880.0));
    assert!(close(t.frequency(57, 0.0).unwrap(), 220.0));
    assert!((t.frequency(60, 0.0).unwrap() - 261.6256).abs() < 1e-3);
    assert!(close(t.frequency(69, 1.0).unwrap(), t.frequency(70, 0.0).unwrap()));
    assert_eq!(t.note_name(69).unwrap(), "A4");
    assert_eq!(t.note_name(60).unwrap(), "C4");
    assert_eq!(t.note_name(59).unwrap(), "B3");
}

#[test]
fn equal_temperament_generates_its_own_intervals() {
    use qop_core::temperament::TemperamentType;
    let t = Temperament::new(
        None, None, Some(TemperamentType::EqualTemperament(19.0, 2.0)), None, None, None, None, None, None,
    );
    assert_eq!(t.intervals().len(), 19);
    assert_eq!(t.note_class_set().len(), 19);
    let base = t.frequency(69, 0.0).unwrap();
    assert!(close(base, 440.0));
    assert!(close(t.frequency(69 + 19, 0.0).unwrap(), 880.0));
    assert!(close(t.frequency(70, 0.0).unwrap(), 440.0 * 2f64.powf(1.0 / 19.0)));

    let tritave = Temperament::new(
        None, None, Some(TemperamentType::EqualTemperament(13.0, 3.0)), None, None, None, None, None, None,
    );
    assert!(close(tritave.frequency(69 + 13, 0.0).unwrap(), 1320.0));
}

#[test]
fn prime_limit_and_arbitrary_keep_the_supplied_intervals() {
    use qop_core::temperament::TemperamentType;
    let just = vec![0.0, 3.8631, 7.0196];
    let t = Temperament::new(
        None, None, Some(TemperamentType::PrimeLimit(5)), Some(0), None, None, None, None, Some(just.clone()),
    );
    assert_eq!(t.intervals(), &just[..]);
    assert_eq!(t.note_class_set().len(), 3);
}

#[test]
fn temperament_setters_reject_bad_values() {
    let mut t = Temperament::new(None, None, None, None, None, None, None, None, None);
    assert!(t.set_tuning_hz(432.0));
    assert!(!t.set_tuning_hz(0.0));
    assert!(!t.set_tuning_hz(f64::NAN));
    assert!(close(t.frequency(69, 0.0).unwrap(), 432.0));
}

#[test]
fn hold_buttons_can_be_named() {
    let mut e = two_string_instrument();
    assert!(e.gut_sustain_set_name("Pedal".to_string()));
    assert!(e.f_multi_sostenuto_set_description(0, "Middle pedal".to_string()));
    assert!(!e.f_multi_sostenuto_set_name(5, "x".to_string()));
    assert_eq!(e.gut_holds().sustain().name, "Pedal");
    assert_eq!(e.f_multi()[0].holds().sostenuto().description, "Middle pedal");
}
