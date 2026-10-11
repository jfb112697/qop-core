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
    copy pressed: bool,
});

getters!(impl[] Gut {
    ref togs: [usize],
    ref trnsp_gut: [Trnsp],
    copy pressed: bool,
    copy index_out: usize,
    copy extra_out: f64,
    copy i_mem: i32,
    copy x_mem: f64,
});

getters!(impl[] Trnsp {
    ref triggers: [usize],
    copy i_delta: i32,
    copy x_delta: f64,
});

getters!(impl[] MulTrnsp {
    ref triggers: [usize],
    ref i_delta: [i32],
    ref x_delta: [f64],
});

getters!(impl[] VFSet {
    ref buttons: [VFBtn],
    ref pressed: [bool],
    ref trnsp_all: [MulTrnsp],
    ref i_mem: [i32],
    ref x_mem: [f64],
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
    ref i_mem: [i32],
    ref x_mem: [f64],
});

getters!(impl[] ComboSet {
    ref buttons: [ComboTog],
    ref pressed: [bool],
    ref combos: [Combo],
    ref trnsp_all: [MulTrnsp],
    ref i_mem: [i32],
    ref x_mem: [f64],
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
    ref i_mem: [i32],
    ref x_mem: [f64],
});
