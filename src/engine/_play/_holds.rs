use std::default::Default;
use crate::engine::{_play::_dig_inputs::any_down, *};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum LimitPolicy {
    // Pressing past max_pressed releases the oldest active member.
    NewestWins,
    // Pressing past max_pressed does nothing until something else is released.
    IgnoreNew,
}

// The one place that decides what happens at a press limit. Both policies are implemented and tested;
// change this constant to switch.
pub(crate) const LIMIT_POLICY: LimitPolicy = LimitPolicy::NewestWins;

// Press state for one group of members that share holds and press limits:
// the guts of an engine, or the buttons of one set.
#[derive(Debug, Clone, Default)]
pub(crate) struct Layer {
    // A key assigned to the member is down.
    pub(crate) pressed: Vec<bool>,
    // The member counts as pressed after holds and limits are applied.
    pub(crate) active: Vec<bool>,
    // Pressed, but pushed out by the press limit until it is released.
    stolen: Vec<bool>,
    // Active only because min_pressed needs it.
    floating: Vec<bool>,
    // Active when sostenuto engaged.
    captured: Vec<bool>,
    seq: Vec<u64>,
    pub(crate) sustain_on: bool,
    pub(crate) sostenuto_on: bool,
}

impl Layer {
    pub(crate) fn new(len: usize) -> Self {
        Layer {
            pressed: vec![false; len],
            active: vec![false; len],
            stolen: vec![false; len],
            floating: vec![false; len],
            captured: vec![false; len],
            seq: vec![0; len],
            sustain_on: false,
            sostenuto_on: false,
        }
    }

    pub(crate) fn clear(&mut self) {
        for m in 0..self.active.len() {
            self.pressed[m] = false;
            self.active[m] = false;
            self.stolen[m] = false;
            self.floating[m] = false;
            self.captured[m] = false;
        }
        self.sustain_on = false;
        self.sostenuto_on = false;
    }

    // Returns true when the member has just gone from released to pressed.
    pub(crate) fn set_pressed(&mut self, m: usize, down: bool, seq_counter: &mut u64) -> bool {
        let rising = down && !self.pressed[m];
        if rising {
            *seq_counter += 1;
            self.seq[m] = *seq_counter;
        }
        if rising || !down {
            self.stolen[m] = false;
        }
        self.pressed[m] = down;
        rising
    }

    // Sustain keeps every active member active. Sostenuto keeps only the members that were active when it engaged.
    // An inverse hold with keys assigned is engaged while those keys are up.
    pub(crate) fn update_holds(&mut self, holds: &HoldBtns, keys_down: &[bool]) {
        let inv_engaged = |hold: &HoldTog| !hold.togs.is_empty() && !any_down(&hold.togs, keys_down);
        self.sustain_on = any_down(&holds.sustain.togs, keys_down) || inv_engaged(&holds.inv_sustain);
        let sostenuto_on = any_down(&holds.sostenuto.togs, keys_down) || inv_engaged(&holds.inv_sostenuto);
        if sostenuto_on && !self.sostenuto_on {
            for m in 0..self.active.len() {
                self.captured[m] = self.active[m];
            }
        }
        if !sostenuto_on {
            for m in 0..self.captured.len() {
                self.captured[m] = false;
            }
        }
        self.sostenuto_on = sostenuto_on;
    }

    // Works out `active` from the pressed flags, the holds, and the press limits.
    // max_pressed == 0 means no limit. Radio mode is exactly one active member.
    pub(crate) fn resolve(&mut self, max_pressed: usize, min_pressed: usize, radio_mode: bool, policy: LimitPolicy) {
        let (max, min) = if radio_mode { (1, 1) } else { (max_pressed, min_pressed) };

        let mut wanted = 0usize;
        for m in 0..self.active.len() {
            let held = (self.sustain_on && self.active[m]) || (self.sostenuto_on && self.captured[m]);
            let want = (self.pressed[m] && !self.stolen[m]) || held;
            self.floating[m] = self.active[m] && !want;
            self.active[m] = want || self.floating[m];
            if want {
                wanted += 1;
            }
        }

        while max > 0 && wanted > max {
            let mut victim: Option<usize> = None;
            for m in 0..self.active.len() {
                if self.active[m] && !self.floating[m] {
                    let better = match (victim, policy) {
                        (None, _) => true,
                        (Some(v), LimitPolicy::NewestWins) => self.seq[m] < self.seq[v],
                        (Some(v), LimitPolicy::IgnoreNew) => self.seq[m] > self.seq[v],
                    };
                    if better {
                        victim = Some(m);
                    }
                }
            }
            let Some(v) = victim else { break };
            self.active[v] = false;
            self.captured[v] = false;
            self.stolen[v] = self.pressed[v];
            wanted -= 1;
        }

        let keep = min.saturating_sub(wanted);
        let mut floating_count = self.floating.iter().filter(|&&f| f).count();
        while floating_count > keep {
            let mut oldest: Option<usize> = None;
            for m in 0..self.floating.len() {
                if self.floating[m] && oldest.is_none_or(|o| self.seq[m] < self.seq[o]) {
                    oldest = Some(m);
                }
            }
            let Some(o) = oldest else { break };
            self.floating[o] = false;
            self.active[o] = false;
            floating_count -= 1;
        }
    }
}
