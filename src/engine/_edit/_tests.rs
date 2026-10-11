use crate::engine::*;
use crate::temperament::Temperament;
use winit::keyboard::KeyCode;

fn engine_with_keys() -> Engine<Edit> {
    let mut e: Engine<Edit> = Engine::default();
    e.dig_inputs_insert_k(KeyCode::KeyA);
    e.dig_inputs_insert_k(KeyCode::KeyB);
    e.dig_inputs_insert_k(KeyCode::KeyC);
    e
}

#[test]
fn gut_max_pressed_can_be_lowered_but_not_below_min() {
    let mut e = engine_with_keys();
    e.gut_change_max_pressed(3);
    e.gut_change_min_pressed(1);
    e.gut_change_max_pressed(2);
    assert_eq!(e.gut_max_pressed, 2);
    e.gut_change_max_pressed(0);
    assert_eq!(e.gut_max_pressed, 2);
}

#[test]
fn vf_insert_btn_appends_and_keeps_pressed_in_sync() {
    let mut e = engine_with_keys();
    e.v_multi_insert_set(0);
    e.f_multi_insert_set(0);
    e.v_multi_insert_btn(0, 1);
    e.f_multi_insert_btn(0, 0);
    assert_eq!(e.v_multi[0].buttons.len(), 2);
    assert_eq!(e.v_multi[0].pressed.len(), 2);
    assert_eq!(e.f_multi[0].buttons.len(), 2);
    assert_eq!(e.f_multi[0].pressed.len(), 2);
    assert!(e.to_play().is_ok());
}

#[test]
fn trnsp_all_can_be_inserted_into_empty_set() {
    let mut e = engine_with_keys();
    e.v_multi_insert_set(0);
    e.c_multi_insert_set(0);
    e.v_multi_insert_trnsp_all_t(0, 0);
    e.c_multi_insert_trnsp_all_t(0, 0);
    assert_eq!(e.v_multi[0].trnsp_all.len(), 1);
    assert_eq!(e.c_multi[0].trnsp_all.len(), 1);
    assert!(e.to_play().is_ok());
}

#[test]
fn trnsp_one_remove_out_of_range_is_a_no_op() {
    let mut e = engine_with_keys();
    e.v_multi_insert_set(0);
    e.c_multi_insert_set(0);
    e.v_multi_remove_trnsp_one_t(0, 0, 0);
    e.v_multi_remove_trnsp_one_t(0, 9, 0);
    e.c_multi_remove_trnsp_one_t(0, 9, 9);
    e.v_multi_insert_trnsp_one_t(0, 0, 0);
    e.v_multi_remove_trnsp_one_t(0, 0, 0);
    assert!(e.v_multi[0].buttons[0].trnsp_one.is_empty());
}

#[test]
fn key_assignment_rejects_unknown_keys_and_duplicates() {
    let mut e = engine_with_keys();
    e.v_multi_insert_set(0);
    e.v_multi_insert_trnsp_all_t(0, 0);
    e.v_multi_insert_trnsp_one_t(0, 0, 0);
    e.gut_insert_trnsp_t(0, 0);

    e.gut_sustain_insert_dig(99);
    e.v_multi_sostenuto_insert_dig(0, 99);
    e.v_multi_insert_trnsp_all_dig(0, 0, 99);
    e.v_multi_insert_trnsp_one_dig(0, 0, 0, 99);
    e.gut_insert_trnsp_dig(0, 0, 99);
    assert!(e.gut_holds.sustain.togs.is_empty());
    assert!(e.v_multi[0].holds.sostenuto.togs.is_empty());
    assert!(e.v_multi[0].trnsp_all[0].triggers.is_empty());
    assert!(e.v_multi[0].buttons[0].trnsp_one[0].triggers.is_empty());
    assert!(e.guts[0].trnsp_gut[0].triggers.is_empty());

    e.gut_sustain_insert_dig(0);
    e.gut_sustain_insert_dig(1);
    e.gut_sustain_insert_dig(0);
    assert_eq!(e.gut_holds.sustain.togs, vec![0, 1]);
    e.v_multi_insert_btn_dig(0, 0, 0);
    e.v_multi_insert_btn_dig(0, 0, 1);
    e.v_multi_insert_btn_dig(0, 0, 0);
    assert_eq!(e.v_multi[0].buttons[0].togs, vec![0, 1]);
    e.gut_sustain_remove_dig(0);
    assert_eq!(e.gut_holds.sustain.togs, vec![1]);
    assert!(e.to_play().is_ok());
}

#[test]
fn temperament_new_falls_back_to_struct_defaults() {
    let t = Temperament::new(None, None, None, None, None, None, None, None, None);
    assert_eq!(t.reference_note, 69);
    assert_eq!(t.tuning_hz, 440.0);
    assert_eq!(t.octave_scalar_factor, 2.0);
    assert_eq!(t.note_class_set.len(), 12);
    assert_eq!(t.intervals.len(), 12);
}
