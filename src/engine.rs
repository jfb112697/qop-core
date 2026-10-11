mod _access;
mod _edit;
mod _play;

pub use _play::{NoteEvent, Player, Voice};

use crate::*;
use better_default::Default;
use nestify::nest;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};
use crate::key::{Analog, Key};

nest! {
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]*
    pub struct Engine<Mode = Edit> {
        pub(crate) _mode: PhantomData<Mode>,
        pub name: String,
        pub description: String,
        pub(crate) dig_inputs: Vec<Key>,
        pub(crate) analog_inputs: Vec<Analog>,
        pub(crate) analog_triggers: Vec<
            pub struct AnalogTrigger {
                pub name: String,
                pub description: String,
                pub(crate) input: usize,
                pub(crate) togs: Vec<usize>,
                #[default(0.1f64)]
                pub(crate) low: f64,
                #[default(0.6f64)]
                pub(crate) high: f64,
                #[default(0.005f64)]
                pub(crate) fast_secs: f64,
                #[default(0.25f64)]
                pub(crate) slow_secs: f64,
            }
        >,
        pub(crate) analog_mods: Vec<
            pub struct AnalogMod {
                pub name: String,
                pub description: String,
                pub(crate) input: usize,
                pub(crate) x_delta: Vec<f64>,
            }
        >,
        pub(crate) analog_levels: Vec<
            pub struct AnalogLevel {
                pub name: String,
                pub description: String,
                pub(crate) input: usize,
                pub(crate) depth: Vec<f64>,
            }
        >,
        #[default(true)]
        pub(crate) index_delta_bool: bool,
        #[default(true)]
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
                pub(crate) index_out: i32,
                pub(crate) retrigger: bool,
                pub(crate) extra_out: f64,
                pub(crate) trnsp_gut: Vec<
                    pub struct Trnsp {
                        pub(crate) triggers: Vec<usize>,
                        pub(crate) latch: bool,
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
                                pub(crate) latch: bool,
                                pub(crate) i_delta: Vec<i32>,
                                pub(crate) x_delta: Vec<f64>,
                            }
                        >,
                    }
                >,
                pub(crate) trnsp_all: Vec<MulTrnsp>,
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
                pub(crate) combos: Vec<
                    pub struct Combo {
                        pub name: String,
                        pub description: String,
                        pub(crate) combo: Vec<bool>,
                        pub(crate) i_delta: Vec<i32>,
                        pub(crate) x_delta: Vec<f64>,
                        pub(crate) trnsp_one: Vec<MulTrnsp>,
                    }
                >,
                pub(crate) holds: HoldBtns,
                pub(crate) trnsp_all: Vec<MulTrnsp>,
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
        self.check_combo_lengths()?;
        self.check_analog_invariants()?;
        self.check_digitalref_invariants()
    }

    pub fn to_play(&self) -> Result<Engine<Play>, String> {
        self.validate()?;

        Ok(Engine {
            _mode: PhantomData,
            name: self.name.clone(),
            description: self.description.clone(),
            dig_inputs: self.dig_inputs.clone(),
            analog_inputs: self.analog_inputs.clone(),
            analog_triggers: self.analog_triggers.clone(),
            analog_mods: self.analog_mods.clone(),
            analog_levels: self.analog_levels.clone(),
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
        })
    }
}
