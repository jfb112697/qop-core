// A plugin calls the Player from its audio callback, where allocating is not allowed.
// This counts heap allocations made while playing.
use qop_core::{Analog, Edit, Engine, Key};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct Counting;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::SeqCst);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

#[test]
fn playing_does_not_allocate() {
    let mut e: Engine<Edit> = Engine::default();
    for key in [Key::KeyJ, Key::KeyK, Key::Digit1, Key::Space, Key::BracketRight, Key::PadRightTrigger] {
        e.dig_inputs_insert_k(key);
    }
    e.gut_insert_g(1);
    e.gut_insert_dig(0, 0);
    e.gut_insert_dig(1, 1);
    e.gut_insert_dig(0, 5);
    e.gut_insert_dig(1, 5);
    e.gut_set_retrigger(1, true);
    e.gut_sustain_insert_dig(3);
    e.gut_change_max_pressed(1);
    e.f_multi_insert_set(0);
    e.f_multi_insert_btn_dig(0, 0, 2);
    e.f_multi_change_i_deltas(0, 0, vec![Some(1), Some(1)]);
    e.c_multi_insert_set(0);
    e.gut_insert_trnsp_t(0, 0);
    e.gut_insert_trnsp_dig(0, 0, 4);
    e.analog_inputs_insert(Analog::PadRightTrigger);
    e.analog_inputs_insert(Analog::MidiPitchBend);
    e.analog_trigger_insert(0, 0);
    e.analog_trigger_insert_dig(0, 5);
    e.analog_mod_insert(0, 1);
    e.analog_mod_change_x_deltas(0, vec![Some(2.0), Some(2.0)]);
    let mut p = e.to_play().unwrap().into_player();

    let mut events = 0usize;
    let before = ALLOCATIONS.load(Ordering::SeqCst);
    for round in 0..100 {
        let t = round as f64;
        events += p.key_down(Key::KeyJ, None).len();
        events += p.key_down(Key::KeyK, Some(0.5)).len();
        events += p.key_down(Key::Digit1, None).len();
        events += p.key_down(Key::Space, None).len();
        events += p.key_down(Key::BracketRight, None).len();
        events += p.analog(Analog::MidiPitchBend, 0.25, t).len();
        events += p.analog(Analog::PadRightTrigger, 1.0, t + 0.1).len();
        events += p.analog(Analog::PadRightTrigger, 0.0, t + 0.2).len();
        events += p.key_up(Key::KeyJ).len();
        events += p.key_up(Key::KeyK).len();
        events += p.key_up(Key::Digit1).len();
        events += p.key_up(Key::BracketRight).len();
        events += p.key_up(Key::Space).len();
        events += p.all_off().len();
    }
    let after = ALLOCATIONS.load(Ordering::SeqCst);

    assert!(events > 0);
    assert_eq!(after - before, 0);
}
