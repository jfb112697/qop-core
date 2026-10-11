mod _access;
mod _edit;
mod _play;

use crate::*;
use better_default::Default;
use duplicate::duplicate_item;
use nestify::nest;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};
use crate::key::Key;

nest! {
    #[repr(C)]*
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]*
    pub struct Engine<Mode = Edit> {
        pub(crate) _mode: PhantomData<Mode>,
        pub name: String,
        pub description: String,
        pub(crate) dig_inputs: Vec<Key>,
        pub(crate) index_delta_bool: bool,
        pub(crate) extra_delta_bool: bool,
        pub(crate) gut_max_pressed: usize,
        pub(crate) gut_min_pressed: usize,
        pub(crate) gut_radio_mode: bool,
        pub(crate) gut_holds:
            pub struct HoldBtns {
                pub(crate) sustain:
                    pub struct HoldTog {
                        pub name: String,
                        pub description: String,
                        pub(crate) togs: Vec<usize>,
                        pub(crate) pressed: bool
                    },
                pub(crate) inv_sustain: HoldTog,
                pub(crate) sostenuto: HoldTog,
                pub(crate) inv_sostenuto: HoldTog,
            },
        #[default(vec![Gut::default()])]
        pub(crate) guts: Vec<
            pub struct Gut {
                pub name: String,
                pub description: String,
                pub(crate) togs: Vec<usize>,
                pub(crate) pressed: bool,
                pub(crate) index_out: usize,
                pub(crate) extra_out: f64,
                pub(crate) i_mem: i32,
                pub(crate) x_mem: f64,
                pub(crate) trnsp_gut: Vec<
                    pub struct Trnsp {
                        pub(crate) triggers: Vec<usize>,
                        pub(crate) i_delta: i32,
                        pub(crate) x_delta: f64,
                    }
                >
            }
        >,
        pub(crate) v_multi: Vec<
            pub struct VFSet {
                pub name: String,
                pub description: String,
                pub(crate) buttons: Vec<
                    pub struct VFBtn {
                        pub name: String,
                        pub description: String,
                        pub(crate) togs: Vec<usize>,
                        pub(crate) i_delta: Vec<i32>,
                        pub(crate) x_delta: Vec<f64>,
                        pub(crate) trnsp_one: Vec<
                            pub struct MulTrnsp {
                                pub(crate) triggers: Vec<usize>,
                                pub(crate) i_delta: Vec<i32>,
                                pub(crate) x_delta: Vec<f64>,
                            }
                        >,
                        pub(crate) i_mem: Vec<i32>,
                        pub(crate) x_mem: Vec<f64>,
                    }
                >,
                #[default(vec![false])]
                pub(crate) pressed: Vec<bool>,
                pub(crate) trnsp_all: Vec<MulTrnsp>,
                pub(crate) i_mem: Vec<i32>,
                pub(crate) x_mem: Vec<f64>,
                pub(crate) holds: HoldBtns,
                pub(crate) max_pressed: usize,
                pub(crate) min_pressed: usize,
                pub(crate) radio_mode: bool,
            }
        >,
        pub(crate) f_multi: Vec<VFSet>,
        pub(crate) c_multi: Vec<
            pub struct ComboSet {
                pub name: String,
                pub description: String,
                #[default(vec![ComboTog::default()])]
                pub(crate) buttons: Vec<
                    pub struct ComboTog {
                        pub name: String,
                        pub description: String,
                        pub(crate) togs: Vec<usize>,
                    }
                >,
                #[default(vec![false])]
                pub(crate) pressed: Vec<bool>,
                pub(crate) combos: Vec<
                    pub struct Combo {
                        pub name: String,
                        pub description: String,
                        pub(crate) combo: Vec<bool>,
                        pub(crate) i_delta: Vec<i32>,
                        pub(crate) x_delta: Vec<f64>,
                        pub(crate) trnsp_one: Vec<MulTrnsp>,
                        pub(crate) i_mem: Vec<i32>,
                        pub(crate) x_mem: Vec<f64>,
                    }
                >,
                pub(crate) holds: HoldBtns,
                pub(crate) trnsp_all: Vec<MulTrnsp>,
                pub(crate) i_mem: Vec<i32>,
                pub(crate) x_mem: Vec<f64>,
                pub(crate) max_pressed: usize,
                pub(crate) min_pressed: usize,
                pub(crate) radio_mode: bool,
            }
        >,
    }
}

impl Engine<Edit> {
    pub fn validate(&self) -> Result<(), String> {
        self.check_multi_delta_lengths()?;
        self.check_pressed_lengths()?;
        self.check_digitalref_invariants()
    }

    pub fn to_play(&self) -> Result<Engine<Play>, String> {
        self.validate()?;

        let mut play = Engine {
            _mode: PhantomData,
            name: self.name.clone(),
            description: self.description.clone(),
            dig_inputs: self.dig_inputs.clone(),
            gut_max_pressed: self.gut_max_pressed.clone(),
            gut_min_pressed: self.gut_min_pressed.clone(),
            gut_radio_mode: self.gut_radio_mode.clone(),
            gut_holds: self.gut_holds.clone(),
            guts: self.guts.clone(),
            v_multi: self.v_multi.clone(),
            f_multi: self.f_multi.clone(),
            c_multi: self.c_multi.clone(),
            index_delta_bool: self.index_delta_bool.clone(),
            extra_delta_bool: self.extra_delta_bool.clone(),
        };
        play.reset_runtime_state();
        Ok(play)
    }
}

impl<Mode> Engine<Mode> {
    // Clears everything that changes while playing (pressed flags, memories) and resizes it to fit the definition.
    pub fn reset_runtime_state(&mut self) {
        let guts_len = self.guts.len();
        self.gut_holds.reset_runtime_state();
        for gut in self.guts.iter_mut() {
            gut.pressed = false;
            gut.i_mem = 0;
            gut.x_mem = 0.0;
        }
        for set in self.v_multi.iter_mut() {
            set.reset_runtime_state(guts_len);
        }
        for set in self.f_multi.iter_mut() {
            set.reset_runtime_state(guts_len);
        }
        for set in self.c_multi.iter_mut() {
            set.reset_runtime_state(guts_len);
        }
    }
}

impl HoldBtns {
    pub(crate) fn reset_runtime_state(&mut self) {
        self.sustain.pressed = false;
        self.inv_sustain.pressed = false;
        self.sostenuto.pressed = false;
        self.inv_sostenuto.pressed = false;
    }
}

#[duplicate_item(
    SetType    deltafield;
    [VFSet]    [buttons];
    [ComboSet] [combos];
)]
impl SetType {
    pub(crate) fn reset_runtime_state(&mut self, guts_len: usize) {
        self.pressed = vec![false; self.buttons.len()];
        self.i_mem = vec![0; guts_len];
        self.x_mem = vec![0.0; guts_len];
        self.holds.reset_runtime_state();
        for d in self.deltafield.iter_mut() {
            d.i_mem = vec![0; guts_len];
            d.x_mem = vec![0.0; guts_len];
        }
    }
}
