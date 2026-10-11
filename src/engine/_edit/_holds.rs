use crate::engine::*;
use duplicate::duplicate_item;

#[duplicate_item(
    holdfield       gut_hold_dig                    operation                          ;
    [sustain]       [gut_sustain_insert_dig]        [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [inv_sustain]   [gut_inv_sustain_insert_dig]    [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [sostenuto]     [gut_sostenuto_insert_dig]      [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [inv_sostenuto] [gut_inv_sostenuto_insert_dig]  [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [sustain]       [gut_sustain_remove_dig]        [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [inv_sustain]   [gut_inv_sustain_remove_dig]    [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [sostenuto]     [gut_sostenuto_remove_dig]      [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [inv_sostenuto] [gut_inv_sostenuto_remove_dig]  [togs.retain(|&idx| idx != key_idx_val)]                   ;
)]
impl Engine<Edit> {
    pub fn gut_hold_dig(&mut self, key_idx_val: usize) -> bool {
        if key_idx_val < self.dig_inputs.len() {
            let togs = &mut self.gut_holds.holdfield.togs;
            operation;
            true
        } else {
            false
        }
    }
}

#[duplicate_item(
    multifield holdfield       multi_hold_dig                     operation                          ;
    [v_multi]  [sustain]       [v_multi_sustain_insert_dig]       [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [v_multi]  [inv_sustain]   [v_multi_inv_sustain_insert_dig]   [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [v_multi]  [sostenuto]     [v_multi_sostenuto_insert_dig]     [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [v_multi]  [inv_sostenuto] [v_multi_inv_sostenuto_insert_dig] [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [f_multi]  [sustain]       [f_multi_sustain_insert_dig]       [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [f_multi]  [inv_sustain]   [f_multi_inv_sustain_insert_dig]   [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [f_multi]  [sostenuto]     [f_multi_sostenuto_insert_dig]     [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [f_multi]  [inv_sostenuto] [f_multi_inv_sostenuto_insert_dig] [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [c_multi]  [sustain]       [c_multi_sustain_insert_dig]       [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [c_multi]  [inv_sustain]   [c_multi_inv_sustain_insert_dig]   [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [c_multi]  [sostenuto]     [c_multi_sostenuto_insert_dig]     [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [c_multi]  [inv_sostenuto] [c_multi_inv_sostenuto_insert_dig] [if !togs.contains(&key_idx_val) { togs.push(key_idx_val) }];
    [v_multi]  [sustain]       [v_multi_sustain_remove_dig]       [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [v_multi]  [inv_sustain]   [v_multi_inv_sustain_remove_dig]   [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [v_multi]  [sostenuto]     [v_multi_sostenuto_remove_dig]     [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [v_multi]  [inv_sostenuto] [v_multi_inv_sostenuto_remove_dig] [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [f_multi]  [sustain]       [f_multi_sustain_remove_dig]       [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [f_multi]  [inv_sustain]   [f_multi_inv_sustain_remove_dig]   [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [f_multi]  [sostenuto]     [f_multi_sostenuto_remove_dig]     [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [f_multi]  [inv_sostenuto] [f_multi_inv_sostenuto_remove_dig] [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [c_multi]  [sustain]       [c_multi_sustain_remove_dig]       [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [c_multi]  [inv_sustain]   [c_multi_inv_sustain_remove_dig]   [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [c_multi]  [sostenuto]     [c_multi_sostenuto_remove_dig]     [togs.retain(|&idx| idx != key_idx_val)]                   ;
    [c_multi]  [inv_sostenuto] [c_multi_inv_sostenuto_remove_dig] [togs.retain(|&idx| idx != key_idx_val)]                   ;
)]
impl Engine<Edit> {
    pub fn multi_hold_dig(&mut self, set_idx: usize, key_idx_val: usize) -> bool {
        if set_idx < self.multifield.len() && key_idx_val < self.dig_inputs.len() {
            let togs = &mut self.multifield[set_idx].holds.holdfield.togs;
            operation;
            true
        } else {
            false
        }
    }
}

#[duplicate_item(
    holdfield       textfield     gut_hold_set_text;
    [sustain] [name] [gut_sustain_set_name];
    [sustain] [description] [gut_sustain_set_description];
    [inv_sustain] [name] [gut_inv_sustain_set_name];
    [inv_sustain] [description] [gut_inv_sustain_set_description];
    [sostenuto] [name] [gut_sostenuto_set_name];
    [sostenuto] [description] [gut_sostenuto_set_description];
    [inv_sostenuto] [name] [gut_inv_sostenuto_set_name];
    [inv_sostenuto] [description] [gut_inv_sostenuto_set_description];
)]
impl Engine<Edit> {
    pub fn gut_hold_set_text(&mut self, value: String) -> bool {
        self.gut_holds.holdfield.textfield = value;
        true
    }
}

#[duplicate_item(
    multifield holdfield       textfield     multi_hold_set_text;
    [v_multi] [sustain] [name] [v_multi_sustain_set_name];
    [v_multi] [sustain] [description] [v_multi_sustain_set_description];
    [v_multi] [inv_sustain] [name] [v_multi_inv_sustain_set_name];
    [v_multi] [inv_sustain] [description] [v_multi_inv_sustain_set_description];
    [v_multi] [sostenuto] [name] [v_multi_sostenuto_set_name];
    [v_multi] [sostenuto] [description] [v_multi_sostenuto_set_description];
    [v_multi] [inv_sostenuto] [name] [v_multi_inv_sostenuto_set_name];
    [v_multi] [inv_sostenuto] [description] [v_multi_inv_sostenuto_set_description];
    [f_multi] [sustain] [name] [f_multi_sustain_set_name];
    [f_multi] [sustain] [description] [f_multi_sustain_set_description];
    [f_multi] [inv_sustain] [name] [f_multi_inv_sustain_set_name];
    [f_multi] [inv_sustain] [description] [f_multi_inv_sustain_set_description];
    [f_multi] [sostenuto] [name] [f_multi_sostenuto_set_name];
    [f_multi] [sostenuto] [description] [f_multi_sostenuto_set_description];
    [f_multi] [inv_sostenuto] [name] [f_multi_inv_sostenuto_set_name];
    [f_multi] [inv_sostenuto] [description] [f_multi_inv_sostenuto_set_description];
    [c_multi] [sustain] [name] [c_multi_sustain_set_name];
    [c_multi] [sustain] [description] [c_multi_sustain_set_description];
    [c_multi] [inv_sustain] [name] [c_multi_inv_sustain_set_name];
    [c_multi] [inv_sustain] [description] [c_multi_inv_sustain_set_description];
    [c_multi] [sostenuto] [name] [c_multi_sostenuto_set_name];
    [c_multi] [sostenuto] [description] [c_multi_sostenuto_set_description];
    [c_multi] [inv_sostenuto] [name] [c_multi_inv_sostenuto_set_name];
    [c_multi] [inv_sostenuto] [description] [c_multi_inv_sostenuto_set_description];
)]
impl Engine<Edit> {
    pub fn multi_hold_set_text(&mut self, set_idx: usize, value: String) -> bool {
        if set_idx < self.multifield.len() {
            self.multifield[set_idx].holds.holdfield.textfield = value;
            true
        } else {
            false
        }
    }
}
