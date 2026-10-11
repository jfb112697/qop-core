use crate::engine::{_edit::*, *};
use duplicate::duplicate_item;

#[duplicate_item(
    multifield deltafield multi_insert_trnsp_all_t     multi_remove_trnsp_all_t     multi_insert_trnsp_all_dig     multi_remove_trnsp_all_dig     multi_insert_trnsp_one_t     multi_remove_trnsp_one_t     multi_insert_trnsp_one_dig     multi_remove_trnsp_one_dig;
    [v_multi]  [buttons]  [v_multi_insert_trnsp_all_t] [v_multi_remove_trnsp_all_t] [v_multi_insert_trnsp_all_dig] [v_multi_remove_trnsp_all_dig] [v_multi_insert_trnsp_one_t] [v_multi_remove_trnsp_one_t] [v_multi_insert_trnsp_one_dig] [v_multi_remove_trnsp_one_dig];
    [f_multi]  [buttons]  [f_multi_insert_trnsp_all_t] [f_multi_remove_trnsp_all_t] [f_multi_insert_trnsp_all_dig] [f_multi_remove_trnsp_all_dig] [f_multi_insert_trnsp_one_t] [f_multi_remove_trnsp_one_t] [f_multi_insert_trnsp_one_dig] [f_multi_remove_trnsp_one_dig];
    [c_multi]  [combos]   [c_multi_insert_trnsp_all_t] [c_multi_remove_trnsp_all_t] [c_multi_insert_trnsp_all_dig] [c_multi_remove_trnsp_all_dig] [c_multi_insert_trnsp_one_t] [c_multi_remove_trnsp_one_t] [c_multi_insert_trnsp_one_dig] [c_multi_remove_trnsp_one_dig];
)]
impl Engine<Edit> {
    pub fn multi_insert_trnsp_all_t(&mut self, set_idx: usize, trnsp_idx: usize) -> bool {
        if set_idx < self.multifield.len() && trnsp_idx <= self.multifield[set_idx].trnsp_all.len() {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            self.multifield[set_idx].trnsp_all_insert_t(trnsp_idx, &mut n);
            true
        } else {
            false
        }
    }
    pub fn multi_remove_trnsp_all_t(&mut self, set_idx: usize, trnsp_idx: usize) -> bool {
        if set_idx < self.multifield.len() && trnsp_idx < self.multifield[set_idx].trnsp_all.len() {
            self.multifield[set_idx].trnsp_all_remove_t(trnsp_idx);
            true
        } else {
            false
        }
    }
    pub fn multi_insert_trnsp_all_dig(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) -> bool {
        if set_idx < self.multifield.len()
            && trnsp_idx < self.multifield[set_idx].trnsp_all.len()
            && key_idx_val < self.dig_inputs.len()
        {
            self.multifield[set_idx].trnsp_all_insert_dig(trnsp_idx, key_idx_val);
            true
        } else {
            false
        }
    }
    pub fn multi_remove_trnsp_all_dig(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) -> bool {
        if set_idx < self.multifield.len() && trnsp_idx < self.multifield[set_idx].trnsp_all.len() {
            self.multifield[set_idx].trnsp_all_remove_dig(trnsp_idx, key_idx_val);
            true
        } else {
            false
        }
    }

    pub fn multi_insert_trnsp_one_t(&mut self, set_idx: usize, del_idx: usize, trnsp_idx: usize,) -> bool {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx <= self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
        {
            let mut n: NewEnginePartParams = NewEnginePartParams::new(&self);
            self.multifield[set_idx].trnsp_one_insert_t(del_idx, trnsp_idx, &mut n);
            true
        } else {
            false
        }
    }
    pub fn multi_remove_trnsp_one_t(&mut self, set_idx: usize, del_idx: usize, trnsp_idx: usize,) -> bool {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx < self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
        {
            self.multifield[set_idx].trnsp_one_remove_t(del_idx, trnsp_idx);
            true
        } else {
            false
        }
    }
    pub fn multi_insert_trnsp_one_dig(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) -> bool {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx < self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
            && key_idx_val < self.dig_inputs.len()
        {
            self.multifield[set_idx].trnsp_one_insert_dig(del_idx, trnsp_idx, key_idx_val);
            true
        } else {
            false
        }
    }
    pub fn multi_remove_trnsp_one_dig(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) -> bool {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx < self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
        {
            self.multifield[set_idx].trnsp_one_remove_dig(del_idx, trnsp_idx, key_idx_val);
            true
        } else {
            false
        }
    }
}

#[duplicate_item(
    multifield deltafield d_del_vec   del_type_vec       multi_trnsp_all_change_deltas       multi_trnsp_one_change_deltas       trnsp_all_change_deltas     trnsp_one_change_deltas;
    [v_multi]  [buttons]  [i_del_vec] [Vec<Option<i32>>] [v_multi_trnsp_all_change_i_deltas] [v_multi_trnsp_one_change_i_deltas] [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas];
    [f_multi]  [buttons]  [i_del_vec] [Vec<Option<i32>>] [f_multi_trnsp_all_change_i_deltas] [f_multi_trnsp_one_change_i_deltas] [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas];
    [c_multi]  [combos]   [i_del_vec] [Vec<Option<i32>>] [c_multi_trnsp_all_change_i_deltas] [c_multi_trnsp_one_change_i_deltas] [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas];
    [v_multi]  [buttons]  [x_del_vec] [Vec<Option<f64>>] [v_multi_trnsp_all_change_x_deltas] [v_multi_trnsp_one_change_x_deltas] [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas];
    [f_multi]  [buttons]  [x_del_vec] [Vec<Option<f64>>] [f_multi_trnsp_all_change_x_deltas] [f_multi_trnsp_one_change_x_deltas] [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas];
    [c_multi]  [combos]   [x_del_vec] [Vec<Option<f64>>] [c_multi_trnsp_all_change_x_deltas] [c_multi_trnsp_one_change_x_deltas] [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas];
)]
impl Engine<Edit> {
    pub fn multi_trnsp_all_change_deltas(
        &mut self,
        set_idx: usize,
        trnsp_idx: usize,
        d_del_vec: del_type_vec,
    ) -> bool {
        if set_idx < self.multifield.len()
            && d_del_vec.len() == self.guts.len()
            && trnsp_idx < self.multifield[set_idx].trnsp_all.len()
        {
            self.multifield[set_idx].trnsp_all_change_deltas(trnsp_idx, d_del_vec);
            true
        } else {
            false
        }
    }
    pub fn multi_trnsp_one_change_deltas(
        &mut self,
        set_idx: usize,
        del_idx: usize,
        trnsp_idx: usize,
        d_del_vec: del_type_vec,
    ) -> bool {
        if set_idx < self.multifield.len()
            && del_idx < self.multifield[set_idx].deltafield.len()
            && trnsp_idx < self.multifield[set_idx].deltafield[del_idx].trnsp_one.len()
            && d_del_vec.len() == self.guts.len()
        {
            self.multifield[set_idx].trnsp_one_change_deltas(del_idx, trnsp_idx, d_del_vec);
            true
        } else {
            false
        }
    }
}

#[duplicate_item(
    SetType    deltafield;
    [VFSet]    [buttons];
    [ComboSet] [combos];
)]
impl SetType {
    pub(crate) fn trnsp_all_insert_t(&mut self, trnsp_idx: usize, n: &mut NewEnginePartParams) {
        self.trnsp_all.insert(trnsp_idx, MulTrnsp::new(n))
    }
    pub(crate) fn trnsp_all_remove_t(&mut self, trnsp_idx: usize) {
        self.trnsp_all.remove(trnsp_idx);
    }
    pub(crate) fn trnsp_all_insert_dig(&mut self, trnsp_idx: usize, key_idx_val: usize) {
        if !self.trnsp_all[trnsp_idx].triggers.contains(&key_idx_val) {
            self.trnsp_all[trnsp_idx].triggers.push(key_idx_val);
        }
    }
    pub(crate) fn trnsp_all_remove_dig(&mut self, trnsp_idx: usize, key_idx_val: usize) {
        self.trnsp_all[trnsp_idx]
            .triggers
            .retain(|&idx| idx != key_idx_val);
    }

    pub(crate) fn trnsp_one_insert_t(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        n: &mut NewEnginePartParams,
    ) {
        self.deltafield[del_idx]
            .trnsp_one
            .insert(trnsp_idx, MulTrnsp::new(n))
    }
    pub(crate) fn trnsp_one_remove_t(&mut self, del_idx: usize, trnsp_idx: usize) {
        self.deltafield[del_idx].trnsp_one.remove(trnsp_idx);
    }
    pub(crate) fn trnsp_one_insert_dig(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        if !self.deltafield[del_idx].trnsp_one[trnsp_idx]
            .triggers
            .contains(&key_idx_val)
        {
            self.deltafield[del_idx].trnsp_one[trnsp_idx]
                .triggers
                .push(key_idx_val)
        }
    }
    pub(crate) fn trnsp_one_remove_dig(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        key_idx_val: usize,
    ) {
        self.deltafield[del_idx].trnsp_one[trnsp_idx]
            .triggers
            .retain(|&idx| idx != key_idx_val);
    }
}

#[duplicate_item(
    SetType    deltafield trnsp_all_change_deltas     trnsp_one_change_deltas     d_field   d_del_val   del_type;
    [VFSet]    [buttons]  [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas] [i_delta] [i_del_val] [Vec<Option<i32>>];
    [VFSet]    [buttons]  [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas] [x_delta] [x_del_val] [Vec<Option<f64>>];
    [ComboSet] [combos]   [trnsp_all_change_i_deltas] [trnsp_one_change_i_deltas] [i_delta] [i_del_val] [Vec<Option<i32>>];
    [ComboSet] [combos]   [trnsp_all_change_x_deltas] [trnsp_one_change_x_deltas] [x_delta] [x_del_val] [Vec<Option<f64>>];
)]
impl SetType {
    pub(crate) fn trnsp_all_change_deltas(&mut self, trnsp_idx: usize, d_del_vec: del_type) {
        for d in 0..d_del_vec.len() {
            if let Some(i_val) = d_del_vec[d] {
                self.trnsp_all[trnsp_idx].d_field[d] = i_val;
            }
        }
    }

    pub(crate) fn trnsp_one_change_deltas(
        &mut self,
        del_idx: usize,
        trnsp_idx: usize,
        d_del_vec: del_type,
    ) {
        for d in 0..d_del_vec.len() {
            if let Some(d_val) = d_del_vec[d] {
                self.deltafield[del_idx].trnsp_one[trnsp_idx].d_field[d] = d_val;
            }
        }
    }
}
