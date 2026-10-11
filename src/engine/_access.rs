use crate::engine::*;

// Read-only access to the engine tree for code outside the crate.
// Fields stay pub(crate) so that every mutation goes through the Engine<Edit> methods and the invariants hold.
macro_rules! getters {
    (impl[$($generics:tt)*] $ty:ty { $( ref $r:ident: $rt:ty, )* $( copy $c:ident: $ct:ty, )* }) => {
        impl<$($generics)*> $ty {
            $( pub fn $r(&self) -> &$rt { &self.$r } )*
            $( pub fn $c(&self) -> $ct { self.$c } )*
        }
    };
}

getters!(impl[Mode] Engine<Mode> {
    ref dig_inputs: [Key],
    ref analog_inputs: [Analog],
    ref analog_triggers: [AnalogTrigger],
    ref analog_mods: [AnalogMod],
    ref analog_levels: [AnalogLevel],
    ref gut_holds: HoldBtns,
    ref guts: [Gut],
    ref v_multi: [VFSet],
    ref f_multi: [VFSet],
    ref c_multi: [ComboSet],
    copy index_delta_bool: bool,
    copy extra_delta_bool: bool,
    copy gut_max_pressed: usize,
    copy gut_min_pressed: usize,
    copy gut_radio_mode: bool,
});

getters!(impl[] HoldBtns {
    ref sustain: HoldTog,
    ref inv_sustain: HoldTog,
    ref sostenuto: HoldTog,
    ref inv_sostenuto: HoldTog,
});

getters!(impl[] HoldTog {
    ref togs: [usize],
});

getters!(impl[] Gut {
    ref togs: [usize],
    ref trnsp_gut: [Trnsp],
    copy index_out: i32,
    copy retrigger: bool,
    copy extra_out: f64,
});

getters!(impl[] Trnsp {
    ref triggers: [usize],
    copy latch: bool,
    copy i_delta: i32,
    copy x_delta: f64,
});

getters!(impl[] MulTrnsp {
    ref triggers: [usize],
    ref i_delta: [i32],
    ref x_delta: [f64],
    copy latch: bool,
});

getters!(impl[] VFSet {
    ref buttons: [VFBtn],
    ref trnsp_all: [MulTrnsp],
    ref holds: HoldBtns,
    copy max_pressed: usize,
    copy min_pressed: usize,
    copy radio_mode: bool,
});

getters!(impl[] VFBtn {
    ref togs: [usize],
    ref i_delta: [i32],
    ref x_delta: [f64],
    ref trnsp_one: [MulTrnsp],
});

getters!(impl[] ComboSet {
    ref buttons: [ComboTog],
    ref combos: [Combo],
    ref trnsp_all: [MulTrnsp],
    ref holds: HoldBtns,
    copy max_pressed: usize,
    copy min_pressed: usize,
    copy radio_mode: bool,
});

getters!(impl[] ComboTog {
    ref togs: [usize],
});

getters!(impl[] Combo {
    ref combo: [bool],
    ref i_delta: [i32],
    ref x_delta: [f64],
    ref trnsp_one: [MulTrnsp],
});

getters!(impl[] AnalogTrigger {
    ref togs: [usize],
    copy input: usize,
    copy low: f64,
    copy high: f64,
    copy fast_secs: f64,
    copy slow_secs: f64,
});

getters!(impl[] AnalogMod {
    ref x_delta: [f64],
    copy input: usize,
});

getters!(impl[] AnalogLevel {
    ref depth: [f64],
    copy input: usize,
});
