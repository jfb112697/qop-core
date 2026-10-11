use crate::engine::_play::*;
use duplicate::duplicate_item;

// Read-only access to what the Player is doing, for code that displays it.
// Every index that is out of range reads as false or as an empty slice.
impl Player {
    // One flag per entry of the engine's dig_inputs.
    pub fn keys_down(&self) -> &[bool] {
        &self.keys_down
    }
    // The last value received for each entry of the engine's analog_inputs.
    pub fn analog_values(&self) -> &[f64] {
        &self.analog_values
    }
    pub fn gut_sustain_on(&self) -> bool {
        self.gut_layer.sustain_on
    }
    pub fn gut_sostenuto_on(&self) -> bool {
        self.gut_layer.sostenuto_on
    }
    pub fn gut_trnsp_active(&self, g_idx: usize, trnsp_idx: usize) -> bool {
        self.gut_trnsp
            .get(g_idx)
            .and_then(|t| t.get(trnsp_idx))
            .is_some_and(|t| t.active)
    }
}

#[duplicate_item(
    states   multi_buttons_active     multi_sustain_on     multi_sostenuto_on     multi_trnsp_all_active     multi_trnsp_one_active    ;
    [v_sets] [v_multi_buttons_active] [v_multi_sustain_on] [v_multi_sostenuto_on] [v_multi_trnsp_all_active] [v_multi_trnsp_one_active];
    [f_sets] [f_multi_buttons_active] [f_multi_sustain_on] [f_multi_sostenuto_on] [f_multi_trnsp_all_active] [f_multi_trnsp_one_active];
    [c_sets] [c_multi_buttons_active] [c_multi_sustain_on] [c_multi_sostenuto_on] [c_multi_trnsp_all_active] [c_multi_trnsp_one_active];
)]
impl Player {
    // Which buttons of the set count as pressed after holds and limits.
    pub fn multi_buttons_active(&self, set_idx: usize) -> &[bool] {
        self.states.get(set_idx).map_or(&[], |s| &s.layer.active)
    }
    pub fn multi_sustain_on(&self, set_idx: usize) -> bool {
        self.states.get(set_idx).is_some_and(|s| s.layer.sustain_on)
    }
    pub fn multi_sostenuto_on(&self, set_idx: usize) -> bool {
        self.states.get(set_idx).is_some_and(|s| s.layer.sostenuto_on)
    }
    pub fn multi_trnsp_all_active(&self, set_idx: usize, trnsp_idx: usize) -> bool {
        self.states
            .get(set_idx)
            .and_then(|s| s.trnsp_all.get(trnsp_idx))
            .is_some_and(|t| t.active)
    }
    // `del_idx` is the button for v and f sets, the combo for c sets.
    pub fn multi_trnsp_one_active(&self, set_idx: usize, del_idx: usize, trnsp_idx: usize) -> bool {
        self.states
            .get(set_idx)
            .and_then(|s| s.trnsp_one.get(del_idx))
            .and_then(|d| d.get(trnsp_idx))
            .is_some_and(|t| t.active)
    }
}
