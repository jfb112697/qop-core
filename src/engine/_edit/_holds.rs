use crate::engine::{_edit::*, *};
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
    pub fn gut_hold_dig(&mut self, key_idx_val: usize) {
        if key_idx_val < self.dig_inputs.len() {
            let togs = &mut self.gut_holds.holdfield.togs;
            operation;
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
    pub fn multi_hold_dig(&mut self, set_idx: usize, key_idx_val: usize) {
        if set_idx < self.multifield.len() && key_idx_val < self.dig_inputs.len() {
            let togs = &mut self.multifield[set_idx].holds.holdfield.togs;
            operation;
        }
    }
}
