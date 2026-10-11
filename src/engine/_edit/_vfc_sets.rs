use crate::engine::{_edit::*, *};
use duplicate::duplicate_item;

#[duplicate_item(
    SetType    multifield multi_insert_set     multi_remove_set     multi_remove_btn     multi_toggle_radio_mode    ;
    [VFSet]    [v_multi]  [v_multi_insert_set] [v_multi_remove_set] [v_multi_remove_btn] [v_multi_toggle_radio_mode];
    [VFSet]    [f_multi]  [f_multi_insert_set] [f_multi_remove_set] [f_multi_remove_btn] [f_multi_toggle_radio_mode];
    [ComboSet] [c_multi]  [c_multi_insert_set] [c_multi_remove_set] [c_multi_remove_btn] [c_multi_toggle_radio_mode];
)]
impl Engine<Edit> {
    pub fn multi_insert_set(&mut self, set_idx: usize) -> bool {
        if set_idx <= self.multifield.len() {
            self.multifield.insert(set_idx, SetType::new(&mut NewEnginePartParams::new(&self)));
            true
        } else {
            false
        }
    }
    pub fn multi_remove_set(&mut self, set_idx: usize) -> bool {
        if set_idx < self.multifield.len()
            && self.multifield.len() > 0
        {
            self.multifield.remove(set_idx);
            true
        } else {
            false
        }
    }
    pub fn multi_remove_btn(&mut self, set_idx: usize, btn_idx: usize) -> bool {
        if set_idx < self.multifield.len()
            && btn_idx < self.multifield[set_idx].buttons.len()
            && self.multifield[set_idx].buttons.len() > 1
        {
            self.multifield[set_idx].remove_btn(btn_idx);
            true
        } else {
            false
        }
    }
    pub fn multi_toggle_radio_mode(&mut self, set_idx: usize) -> bool {
        if set_idx < self.multifield.len() {
            self.multifield[set_idx].radio_mode = !self.multifield[set_idx].radio_mode;
            true
        } else {
            false
        }
    }
}

#[duplicate_item(
    SetType    multifield multi_change_minmax_pressed   minmaxval minmax_field  iscomparedto                             multi_insertremove_btn_dig btn_insertremove_dig;
    [VFSet]    [v_multi]  [v_multi_change_min_pressed]  [min_val] [min_pressed] [le(&self.v_multi[set_idx].max_pressed)] [v_multi_insert_btn_dig]   [btn_insert_dig]    ;
    [VFSet]    [f_multi]  [f_multi_change_min_pressed]  [min_val] [min_pressed] [le(&self.f_multi[set_idx].max_pressed)] [f_multi_insert_btn_dig]   [btn_insert_dig]    ;
    [ComboSet] [c_multi]  [c_multi_change_min_pressed]  [min_val] [min_pressed] [le(&self.c_multi[set_idx].max_pressed)] [c_multi_insert_btn_dig]   [btn_insert_dig]    ;
    [VFSet]    [v_multi]  [v_multi_change_max_pressed]  [max_val] [max_pressed] [ge(&self.v_multi[set_idx].min_pressed)] [v_multi_remove_btn_dig]   [btn_remove_dig]    ;
    [VFSet]    [f_multi]  [f_multi_change_max_pressed]  [max_val] [max_pressed] [ge(&self.f_multi[set_idx].min_pressed)] [f_multi_remove_btn_dig]   [btn_remove_dig]    ;
    [ComboSet] [c_multi]  [c_multi_change_max_pressed]  [max_val] [max_pressed] [ge(&self.c_multi[set_idx].min_pressed)] [c_multi_remove_btn_dig]   [btn_remove_dig]    ;
)]
impl Engine<Edit> {
    pub fn multi_change_minmax_pressed(&mut self, set_idx: usize, minmaxval: usize) -> bool {
        if set_idx < self.multifield.len()
            && minmaxval.iscomparedto
        {
            self.multifield[set_idx].minmax_field = minmaxval;
            true
        } else {
            false
        }
    }
    pub fn multi_insertremove_btn_dig(&mut self, set_idx: usize, btn_idx: usize, key_idx_val: usize) -> bool {
        if set_idx < self.multifield.len()
            && btn_idx < self.multifield[set_idx].buttons.len()
            && key_idx_val < self.dig_inputs.len()
        {
            self.multifield[set_idx].btn_insertremove_dig(btn_idx, key_idx_val);
            true
        } else {
            false
        }
    }
}

#[duplicate_item(
    multifield vf_multi_insert_btn ;
    [v_multi]  [v_multi_insert_btn];
    [f_multi]  [f_multi_insert_btn];
)]
impl Engine<Edit> {
    pub fn vf_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) -> bool {
        if set_idx < self.multifield.len() && btn_idx <= self.multifield[set_idx].buttons.len() {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            self.multifield[set_idx].insert_btn(btn_idx, &mut n);
            true
        } else {
            false
        }
    }
}

impl Engine<Edit> {
    pub fn c_multi_insert_btn(&mut self, set_idx: usize, btn_idx: usize) -> bool {
        if set_idx < self.c_multi.len()
            && btn_idx <= self.c_multi[set_idx].buttons.len()
        {
            self.c_multi[set_idx].insert_btn(btn_idx);
            true
        } else {
            false
        }
    }
    pub fn c_multi_insert_combo(&mut self, set_idx: usize, c_idx: usize) -> bool {
        if set_idx < self.c_multi.len()
            && c_idx <= self.c_multi[set_idx].combos.len() 
        {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            n.c_btn_len = self.c_multi[set_idx].buttons.len();
            self.c_multi[set_idx].insert_combo(c_idx, &mut n);
            true
        } else {
            false
        }
    }
    pub fn c_multi_remove_combo(&mut self, set_idx: usize, c_idx: usize) -> bool {
        if set_idx < self.c_multi.len()
            && c_idx < self.c_multi[set_idx].combos.len()
        {
            self.c_multi[set_idx].remove_combo(c_idx);
            true
        } else {
            false
        }
    }
}

#[duplicate_item(
    multifield deltafield multi_change_deltas       d_field   d_del_vec   del_type_vec;
    [v_multi]  [buttons]  [v_multi_change_i_deltas] [i_delta] [i_del_vec] [Vec<Option<i32>>];
    [f_multi]  [buttons]  [f_multi_change_i_deltas] [i_delta] [i_del_vec] [Vec<Option<i32>>];
    [c_multi]  [combos]   [c_multi_change_i_deltas] [i_delta] [i_del_vec] [Vec<Option<i32>>];
    [v_multi]  [buttons]  [v_multi_change_x_deltas] [x_delta] [x_del_vec] [Vec<Option<f64>>];
    [f_multi]  [buttons]  [f_multi_change_x_deltas] [x_delta] [x_del_vec] [Vec<Option<f64>>];
    [c_multi]  [combos]   [c_multi_change_x_deltas] [x_delta] [x_del_vec] [Vec<Option<f64>>];
)]
impl Engine<Edit> {
    pub fn multi_change_deltas(&mut self, set_idx: usize, del_idx: usize, d_del_vec: del_type_vec) -> bool {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && d_del_vec.len() == self.guts.len()
        {
            for (d, &d_del) in d_del_vec.iter().enumerate() {
                if let Some(d_val) = d_del {
                    if set_idx < self.multifield.len() {
                        self.multifield[set_idx].deltafield[del_idx].d_field[d] = d_val;
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

impl VFSet {
    pub(crate) fn insert_btn(&mut self, btn_idx: usize, n: &mut NewEnginePartParams) {
        self.buttons.insert(btn_idx, VFBtn::new(n));
        self.pressed.insert(btn_idx, false);
    }
    pub(crate) fn remove_btn(&mut self, btn_idx: usize) {
        self.buttons.remove(btn_idx);
        self.pressed.remove(btn_idx);
    }
}

impl ComboSet {
    pub(crate) fn insert_btn(&mut self, btn_idx: usize) {
        self.buttons.insert(btn_idx, ComboTog::default());
        self.pressed.insert(btn_idx, false);
        for c in 0..self.combos.len() {
            self.combos[c].combo.insert(btn_idx, false);
        }
    }
    pub(crate) fn remove_btn(&mut self, btn_idx: usize) {
        self.buttons.remove(btn_idx);
        self.pressed.remove(btn_idx);
        for c in 0..self.combos.len() {
            self.combos[c].combo.remove(btn_idx);
        }
    }
    pub(crate) fn insert_combo(&mut self, c_idx: usize, n: &mut NewEnginePartParams) {
        self.combos.insert(c_idx, Combo::new(n));
    }
    pub(crate) fn remove_combo(&mut self, c_idx: usize) {
        self.combos.remove(c_idx);
    }
}

#[duplicate_item(
    SetType     btn_dig          operation                          ;
    [VFSet]     [btn_insert_dig] [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [ComboSet]  [btn_insert_dig] [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [VFSet]     [btn_remove_dig] [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [ComboSet]  [btn_remove_dig] [togs.retain(|&idx| idx != key_idx_val)]                   ;
)]
impl SetType {
    pub(crate) fn btn_dig(&mut self, btn_idx: usize, key_idx_val: usize) {
        let togs = &mut self.buttons[btn_idx].togs;
        operation;
    }
}

#[duplicate_item(
    multifield textfield     multi_set_text              multi_btn_set_text             ;
    [v_multi]  [name]        [v_multi_set_name]          [v_multi_btn_set_name]         ;
    [f_multi]  [name]        [f_multi_set_name]          [f_multi_btn_set_name]         ;
    [c_multi]  [name]        [c_multi_set_name]          [c_multi_btn_set_name]         ;
    [v_multi]  [description] [v_multi_set_description]   [v_multi_btn_set_description]  ;
    [f_multi]  [description] [f_multi_set_description]   [f_multi_btn_set_description]  ;
    [c_multi]  [description] [c_multi_set_description]   [c_multi_btn_set_description]  ;
)]
impl Engine<Edit> {
    pub fn multi_set_text(&mut self, set_idx: usize, value: String) -> bool {
        if set_idx < self.multifield.len() {
            self.multifield[set_idx].textfield = value;
            true
        } else {
            false
        }
    }
    pub fn multi_btn_set_text(&mut self, set_idx: usize, btn_idx: usize, value: String) -> bool {
        if set_idx < self.multifield.len() && btn_idx < self.multifield[set_idx].buttons.len() {
            self.multifield[set_idx].buttons[btn_idx].textfield = value;
            true
        } else {
            false
        }
    }
}

#[duplicate_item(
    textfield     c_multi_combo_set_text         ;
    [name]        [c_multi_combo_set_name]       ;
    [description] [c_multi_combo_set_description];
)]
impl Engine<Edit> {
    pub fn c_multi_combo_set_text(&mut self, set_idx: usize, c_idx: usize, value: String) -> bool {
        if set_idx < self.c_multi.len() && c_idx < self.c_multi[set_idx].combos.len() {
            self.c_multi[set_idx].combos[c_idx].textfield = value;
            true
        } else {
            false
        }
    }
}

impl Engine<Edit> {
    pub fn c_multi_change_combo(&mut self, set_idx: usize, c_idx: usize, pattern: &[bool]) -> bool {
        if set_idx < self.c_multi.len()
            && c_idx < self.c_multi[set_idx].combos.len()
            && pattern.len() == self.c_multi[set_idx].buttons.len()
        {
            self.c_multi[set_idx].combos[c_idx].combo = pattern.to_vec();
            true
        } else {
            false
        }
    }
}
