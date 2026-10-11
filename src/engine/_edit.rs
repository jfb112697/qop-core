mod _analog;
mod _dig_inputs;
mod _gut;
mod _holds;
#[cfg(test)]
mod _tests;
mod _trnsp;
mod _vfc_sets;

use crate::engine::*;
use better_default::Default;

#[derive(Default, Clone)]
pub(crate) struct NewEnginePartParams {
    pub(crate) guts_len: usize,
    pub(crate) g_idx: usize,
    pub(crate) set_idx: usize,
    pub(crate) del_idx: usize,
    pub(crate) trnsp_idx: usize,
    pub(crate) c_btn_len: usize,
}

impl NewEnginePartParams {
    pub(crate) fn new(engine: &Engine) -> Self {
        NewEnginePartParams {
            guts_len: engine.guts.len(),
            c_btn_len: 1,
            ..Self::default()
        }
    }
}

pub(crate) trait NewTrait: Default {
    fn new(n: &mut NewEnginePartParams) -> Self;
}

impl NewTrait for VFSet {
    fn new(n: &mut NewEnginePartParams) -> Self {
        VFSet {
            buttons: vec![VFBtn::new(n)],
            ..Default::default()
        }
    }
}

impl NewTrait for VFBtn {
    fn new(n: &mut NewEnginePartParams) -> Self {
        VFBtn {
            i_delta: vec![0; n.guts_len],
            x_delta: vec![0.0; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for ComboSet {
    fn new(n: &mut NewEnginePartParams) -> Self {
        n.c_btn_len = 1usize;
        ComboSet {
            combos: vec![Combo::new(n)],
            ..Default::default()
        }
    }
}

impl NewTrait for Combo {
    fn new(n: &mut NewEnginePartParams) -> Self {
        Combo {
            combo: vec![false; n.c_btn_len],
            i_delta: vec![0; n.guts_len],
            x_delta: vec![0.0; n.guts_len],
            ..Default::default()
        }
    }
}

impl NewTrait for Trnsp {
    fn new(_n: &mut NewEnginePartParams) -> Self {
        Self::default()
    }
}

impl NewTrait for MulTrnsp {
    fn new(n: &mut NewEnginePartParams) -> Self {
        MulTrnsp {
            i_delta: vec![0; n.guts_len],
            x_delta: vec![0.0; n.guts_len],
            ..Default::default()
        }
    }
}
