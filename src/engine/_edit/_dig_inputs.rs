use crate::engine::*;
use duplicate::duplicate_item;
use std::cmp::Ordering::{Equal, Greater, Less};

macro_rules! assert_lt_expr {
    ($left:expr, $right:expr) => {
        if !($left < $right) {
            return Err(format!(
                "Assertion failed: `{}` < `{}`\n(left: `{:?}`, right: `{:?}`)",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

impl Engine<Edit> {
    pub fn dig_inputs_insert_k(&mut self, key_code: Key) -> bool {
        if !self.dig_inputs.contains(&key_code) {
            self.dig_inputs.push(key_code);
            true
        } else {
            false
        }
    }
    fn dig_inputs_global_vec_manip(&mut self, operation: impl Fn(&mut Vec<usize>)) {
        for g in 0..self.guts.len() {
            operation(&mut self.guts[g].togs);
            for tg in 0..self.guts[g].trnsp_gut.len() {
                operation(&mut self.guts[g].trnsp_gut[tg].triggers);
            }
        }
        operation(&mut self.gut_holds.sustain.togs);
        operation(&mut self.gut_holds.inv_sustain.togs);
        operation(&mut self.gut_holds.sostenuto.togs);
        operation(&mut self.gut_holds.inv_sostenuto.togs);
        for set in 0..self.v_multi.len() {
            self.v_multi[set].all_dig_idx_vecs(&operation);
        }
        for set in 0..self.f_multi.len() {
            self.f_multi[set].all_dig_idx_vecs(&operation);
        }
        for set in 0..self.c_multi.len() {
            self.c_multi[set].all_dig_idx_vecs(&operation);
        }
    }
    pub fn dig_inputs_purge_dig(&mut self, key_code: Key) -> bool {
        if let Some(i) = self.dig_inputs.iter().position(|&key| key == key_code) {
            let dig_idx_purge = |key_idx_vec: &mut Vec<usize>| {
                key_idx_vec.retain_mut(|k: &mut usize| match (*k).cmp(&i) {
                    Less => true,
                    Equal => false,
                    Greater => {
                        *k -= 1;
                        true
                    }
                })
            };
            Engine::dig_inputs_global_vec_manip(self, dig_idx_purge);
            self.dig_inputs.remove(i);
            true
        } else {
            false
        }
    }
    pub fn dig_inputs_swap_idxs(&mut self, kc1: Key, kc2: Key, swap_all_fields: bool) -> bool {
        // This function swaps 2 digital input values for 2 existing inputs in the dig_inputs field.
        // swap_all_fields == true -> swaps the nodes of 2 existing keys in all of rest of the Qop's fields,  so all of those affected usize values would be now pointing to the same keys as before.
        // swap_all_fields == false -> leaves all of the rest of the Qop's fields alone, so all of those affected usize values would be now pointing to swapped keys.
        let i1 = self.dig_inputs.iter().position(|&key| key == kc1);
        let i2 = self.dig_inputs.iter().position(|&key| key == kc2);
        if let (Some(i1), Some(i2)) = (i1, i2) {
            self.dig_inputs.swap(i1, i2);
            if swap_all_fields {
                let k_idxs_swap = |k_idx_vec: &mut Vec<usize>| {
                    k_idx_vec.iter_mut().for_each(|k: &mut usize| {
                        match ((*k).cmp(&i1), (*k).cmp(&i2)) {
                            (Equal, _) => *k = i2,
                            (_, Equal) => *k = i1,
                            (_, _) => {}
                        }
                    });
                };
                Engine::dig_inputs_global_vec_manip(self, k_idxs_swap);
            }
            true
        } else {
            false
        }
    }
    pub fn dig_inputs_change_idx_to(&mut self, kc_old: Key, kc_new: Key) -> bool {
        let i1 = self.dig_inputs.iter().position(|&key| key == kc_old);
        let i2 = self.dig_inputs.iter().position(|&key| key == kc_new);
        if let (Some(i1), Some(i2)) = (i1, i2) {
            let k_idx_update = |k_idx_vec: &mut Vec<usize>| {
                k_idx_vec.iter_mut().for_each(|k: &mut usize| {
                    if *k == i1 {
                        *k = i2;
                    }
                });
                k_idx_vec.sort();
                k_idx_vec.dedup();
            };
            Engine::dig_inputs_global_vec_manip(self, k_idx_update);
            true
        } else {
            false
        }
    }
    pub(crate) fn check_digitalref_invariants(&self) -> Result<(), String> {
        for g in 0..self.guts.len() {
            for t in 0..self.guts[g].togs.len() {
                assert_lt_expr!(self.guts[g].togs[t], self.dig_inputs.len())
            }
            for tg in 0..self.guts[g].trnsp_gut.len() {
                for t in 0..self.guts[g].trnsp_gut[tg].triggers.len() {
                    assert_lt_expr!(
                        self.guts[g].trnsp_gut[tg].triggers[t], self.dig_inputs.len()
                    )
                }
            }
        }

        self.gut_holds.check_digitalref_invariants(self.dig_inputs.len())?;

        for set in 0..self.v_multi.len() {
            self.v_multi[set].check_digitalref_invariants(self.dig_inputs.len())?;
        }
        for set in 0..self.f_multi.len() {
            self.f_multi[set].check_digitalref_invariants(self.dig_inputs.len())?;
        }
        for set in 0..self.c_multi.len() {
            self.c_multi[set].check_digitalref_invariants(self.dig_inputs.len())?;
        }
        Ok(())
    }
}

#[duplicate_item(
    SetType    tofield;
    [VFSet]    [buttons];
    [ComboSet] [combos];
)]
impl SetType {
    pub(crate) fn all_dig_idx_vecs(&mut self, vec_closure: impl Fn(&mut Vec<usize>)) {
        for b in 0..self.buttons.len() {
            vec_closure(&mut self.buttons[b].togs);
        }
        for c in 0..self.tofield.len() {
            for to in 0..self.tofield[c].trnsp_one.len() {
                vec_closure(&mut self.tofield[c].trnsp_one[to].triggers);
            }
        }
        for ta in 0..self.trnsp_all.len() {
            vec_closure(&mut self.trnsp_all[ta].triggers);
        }
        vec_closure(&mut self.holds.sustain.togs);
        vec_closure(&mut self.holds.inv_sustain.togs);
        vec_closure(&mut self.holds.sostenuto.togs);
        vec_closure(&mut self.holds.inv_sostenuto.togs);
    }
    pub(crate) fn check_digitalref_invariants(&self, dig_vec_len: usize) -> Result<(), String> {
        for b in 0..self.buttons.len() {
            for t in 0..self.buttons[b].togs.len() {
                assert_lt_expr!(self.buttons[b].togs[t], dig_vec_len)
            }
        }
        for c in 0..self.tofield.len() {
            for to in 0..self.tofield[c].trnsp_one.len() {
                for t in 0..self.tofield[c].trnsp_one[to].triggers.len() {
                    assert_lt_expr!(self.tofield[c].trnsp_one[to].triggers[t], dig_vec_len)
                }
            }
        }
        for ta in 0..self.trnsp_all.len() {
            for t in 0..self.trnsp_all[ta].triggers.len() {
                assert_lt_expr!(self.trnsp_all[ta].triggers[t], dig_vec_len)
            }
        }
        return self.holds.check_digitalref_invariants(dig_vec_len);
    }
}

impl HoldBtns {
    pub(crate) fn check_digitalref_invariants(&self, dig_vec_len: usize) -> Result<(), String> {
        for sus in 0..self.sustain.togs.len() {
            assert_lt_expr!(self.sustain.togs[sus], dig_vec_len)
        }
        for isus in 0..self.inv_sustain.togs.len() {
            assert_lt_expr!(self.inv_sustain.togs[isus], dig_vec_len)
        }
        for sos in 0..self.sostenuto.togs.len() {
            assert_lt_expr!(self.sostenuto.togs[sos], dig_vec_len)
        }
        for isos in 0..self.inv_sostenuto.togs.len() {
            assert_lt_expr!(self.inv_sostenuto.togs[isos], dig_vec_len)
        }
        Ok(())
    }
}
