use crate::engine::*;
use std::cmp::Ordering::{Equal, Greater, Less};

macro_rules! assert_expr {
    ($cond:expr) => {
        if !($cond) {
            return Err(format!("Assertion failed: `{}`", stringify!($cond)));
        }
    };
}

impl Engine<Edit> {
    pub fn analog_inputs_insert(&mut self, analog: Analog) -> bool {
        if !self.analog_inputs.contains(&analog) {
            self.analog_inputs.push(analog);
            true
        } else {
            false
        }
    }
    // Removing an analog input also removes every trigger and mod that reads it.
    pub fn analog_inputs_purge(&mut self, analog: Analog) -> bool {
        if let Some(i) = self.analog_inputs.iter().position(|&a| a == analog) {
            self.analog_triggers.retain(|t| t.input != i);
            self.analog_mods.retain(|m| m.input != i);
            for t in self.analog_triggers.iter_mut() {
                if t.input.cmp(&i) == Greater {
                    t.input -= 1;
                }
            }
            for m in self.analog_mods.iter_mut() {
                match m.input.cmp(&i) {
                    Greater => m.input -= 1,
                    Less | Equal => {}
                }
            }
            self.analog_inputs.remove(i);
            true
        } else {
            false
        }
    }

    pub fn analog_trigger_insert(&mut self, trig_idx: usize, input_idx: usize) -> bool {
        if trig_idx <= self.analog_triggers.len() && input_idx < self.analog_inputs.len() {
            self.analog_triggers.insert(
                trig_idx,
                AnalogTrigger {
                    input: input_idx,
                    ..Default::default()
                },
            );
            true
        } else {
            false
        }
    }
    pub fn analog_trigger_remove(&mut self, trig_idx: usize) -> bool {
        if trig_idx < self.analog_triggers.len() {
            self.analog_triggers.remove(trig_idx);
            true
        } else {
            false
        }
    }
    pub fn analog_trigger_set_input(&mut self, trig_idx: usize, input_idx: usize) -> bool {
        if trig_idx < self.analog_triggers.len() && input_idx < self.analog_inputs.len() {
            self.analog_triggers[trig_idx].input = input_idx;
            true
        } else {
            false
        }
    }
    pub fn analog_trigger_insert_dig(&mut self, trig_idx: usize, key_idx_val: usize) -> bool {
        if trig_idx < self.analog_triggers.len() && key_idx_val < self.dig_inputs.len() {
            let togs = &mut self.analog_triggers[trig_idx].togs;
            if !togs.contains(&key_idx_val) {
                togs.push(key_idx_val);
            }
            true
        } else {
            false
        }
    }
    pub fn analog_trigger_remove_dig(&mut self, trig_idx: usize, key_idx_val: usize) -> bool {
        if trig_idx < self.analog_triggers.len() && key_idx_val < self.dig_inputs.len() {
            self.analog_triggers[trig_idx]
                .togs
                .retain(|&idx| idx != key_idx_val);
            true
        } else {
            false
        }
    }
    pub fn analog_trigger_set_thresholds(&mut self, trig_idx: usize, low: f64, high: f64) -> bool {
        if trig_idx < self.analog_triggers.len() && low.is_finite() && high.is_finite() && low < high {
            self.analog_triggers[trig_idx].low = low;
            self.analog_triggers[trig_idx].high = high;
            true
        } else {
            false
        }
    }
    pub fn analog_trigger_set_times(&mut self, trig_idx: usize, fast_secs: f64, slow_secs: f64) -> bool {
        if trig_idx < self.analog_triggers.len()
            && fast_secs.is_finite()
            && slow_secs.is_finite()
            && 0.0 <= fast_secs
            && fast_secs < slow_secs
        {
            self.analog_triggers[trig_idx].fast_secs = fast_secs;
            self.analog_triggers[trig_idx].slow_secs = slow_secs;
            true
        } else {
            false
        }
    }
    pub fn analog_trigger_set_name(&mut self, trig_idx: usize, value: String) -> bool {
        if trig_idx < self.analog_triggers.len() {
            self.analog_triggers[trig_idx].name = value;
            true
        } else {
            false
        }
    }

    pub fn analog_mod_insert(&mut self, mod_idx: usize, input_idx: usize) -> bool {
        if mod_idx <= self.analog_mods.len() && input_idx < self.analog_inputs.len() {
            self.analog_mods.insert(
                mod_idx,
                AnalogMod {
                    input: input_idx,
                    x_delta: vec![0.0; self.guts.len()],
                    ..Default::default()
                },
            );
            true
        } else {
            false
        }
    }
    pub fn analog_mod_remove(&mut self, mod_idx: usize) -> bool {
        if mod_idx < self.analog_mods.len() {
            self.analog_mods.remove(mod_idx);
            true
        } else {
            false
        }
    }
    pub fn analog_mod_set_input(&mut self, mod_idx: usize, input_idx: usize) -> bool {
        if mod_idx < self.analog_mods.len() && input_idx < self.analog_inputs.len() {
            self.analog_mods[mod_idx].input = input_idx;
            true
        } else {
            false
        }
    }
    pub fn analog_mod_change_x_deltas(&mut self, mod_idx: usize, x_del_vec: Vec<Option<f64>>) -> bool {
        if mod_idx < self.analog_mods.len() && x_del_vec.len() == self.guts.len() {
            for (d, &x_del) in x_del_vec.iter().enumerate() {
                if let Some(x_val) = x_del {
                    self.analog_mods[mod_idx].x_delta[d] = x_val;
                }
            }
            true
        } else {
            false
        }
    }
    pub fn analog_mod_set_name(&mut self, mod_idx: usize, value: String) -> bool {
        if mod_idx < self.analog_mods.len() {
            self.analog_mods[mod_idx].name = value;
            true
        } else {
            false
        }
    }

    pub(crate) fn check_analog_invariants(&self) -> Result<(), String> {
        for t in 0..self.analog_triggers.len() {
            assert_expr!(self.analog_triggers[t].input < self.analog_inputs.len());
            assert_expr!(self.analog_triggers[t].low < self.analog_triggers[t].high);
            assert_expr!(self.analog_triggers[t].fast_secs < self.analog_triggers[t].slow_secs);
        }
        for m in 0..self.analog_mods.len() {
            assert_expr!(self.analog_mods[m].input < self.analog_inputs.len());
            assert_expr!(self.analog_mods[m].x_delta.len() == self.guts.len());
        }
        Ok(())
    }
}
