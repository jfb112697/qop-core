mod _access;
mod _analog;
mod _dig_inputs;
mod _gut;
mod _holds;
#[cfg(test)]
mod _tests;
mod _trnsp;
mod _vfc_sets;

use crate::engine::*;
use _analog::{TriggerEdge, TriggerState};
use _dig_inputs::any_down;
use _holds::{LIMIT_POLICY, Layer};
use _trnsp::TrnspState;
use _vfc_sets::SetState;

// What a gut started, changed, or stopped doing as a result of one input.
// `index` and `extra` are what Temperament::frequency takes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoteEvent {
    On { gut: usize, index: i32, extra: f64, velocity: f64 },
    Change { gut: usize, index: i32, extra: f64 },
    // The gut's loudness scale changed. Sent whether or not the gut is sounding; it starts at 1.0.
    Level { gut: usize, level: f64 },
    Off { gut: usize },
}

// What a gut is doing now. `velocity` is that of the press that last triggered it;
// `level` is the continuous loudness scale from analog levels. Loudness is velocity * level.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Voice {
    pub sounding: bool,
    pub index: i32,
    pub extra: f64,
    pub velocity: f64,
    pub level: f64,
}

// Plays an Engine<Play>: feed it input, read back note events.
// All state that changes while playing lives here, and none of the input methods allocate.
#[derive(Debug, Clone)]
pub struct Player {
    engine: Engine<Play>,
    keys_down: Vec<bool>,
    analog_values: Vec<f64>,
    // False until the input has sent a value, so a device that is not there does not silence anything.
    analog_seen: Vec<bool>,
    triggers: Vec<TriggerState>,
    gut_layer: Layer,
    gut_trnsp: Vec<Vec<TrnspState>>,
    v_sets: Vec<SetState>,
    f_sets: Vec<SetState>,
    c_sets: Vec<SetState>,
    voices: Vec<Voice>,
    events: Vec<NoteEvent>,
    seq_counter: u64,
    press_velocity: f64,
}

impl Engine<Play> {
    pub fn into_player(self) -> Player {
        Player::new(self)
    }
}

impl Player {
    pub fn new(engine: Engine<Play>) -> Player {
        let guts_len = engine.guts.len();
        let most_trigger_keys = engine.analog_triggers.iter().map(|t| t.togs.len()).max().unwrap_or(0);
        let mut player = Player {
            keys_down: vec![false; engine.dig_inputs.len()],
            analog_values: vec![0.0; engine.analog_inputs.len()],
            analog_seen: vec![false; engine.analog_inputs.len()],
            triggers: vec![TriggerState::default(); engine.analog_triggers.len()],
            gut_layer: Layer::new(guts_len),
            gut_trnsp: engine
                .guts
                .iter()
                .map(|g| vec![TrnspState::default(); g.trnsp_gut.len()])
                .collect(),
            v_sets: engine.v_multi.iter().map(|s| s.new_state()).collect(),
            f_sets: engine.f_multi.iter().map(|s| s.new_state()).collect(),
            c_sets: engine.c_multi.iter().map(|s| s.new_state()).collect(),
            voices: engine
                .guts
                .iter()
                .map(|g| Voice { sounding: false, index: g.index_out, extra: g.extra_out, velocity: 1.0, level: 1.0 })
                .collect(),
            // One key changes each gut at most once, a change is at most Level + Off + On,
            // and one analog input can press several keys.
            events: Vec::with_capacity(3 * guts_len * (most_trigger_keys + 1)),
            seq_counter: 0,
            press_velocity: 1.0,
            engine,
        };
        player.refresh();
        player.events.clear();
        player
    }

    pub fn engine(&self) -> &Engine<Play> {
        &self.engine
    }

    pub fn voices(&self) -> &[Voice] {
        &self.voices
    }

    // `velocity` is 0.0..=1.0. Pass None for inputs that have none (a computer keyboard); they play at full velocity.
    pub fn key_down(&mut self, key: Key, velocity: Option<f64>) -> &[NoteEvent] {
        self.events.clear();
        if let Some(k) = self.engine.dig_idx(key) {
            self.press(k, velocity.unwrap_or(1.0));
        }
        &self.events
    }

    pub fn key_up(&mut self, key: Key) -> &[NoteEvent] {
        self.events.clear();
        if let Some(k) = self.engine.dig_idx(key) {
            self.release(k);
        }
        &self.events
    }

    // `value` is normalized by the caller (see `Analog`). `time_secs` is any steadily increasing clock;
    // analog triggers use it to measure how fast the value moved.
    pub fn analog(&mut self, input: Analog, value: f64, time_secs: f64) -> &[NoteEvent] {
        self.events.clear();
        let Some(a) = self.engine.analog_idx(input) else {
            return &self.events;
        };
        self.analog_values[a] = value;
        self.analog_seen[a] = true;
        for t in 0..self.engine.analog_triggers.len() {
            if self.engine.analog_triggers[t].input != a {
                continue;
            }
            let edge = self.engine.analog_triggers[t].update(&mut self.triggers[t], value, time_secs);
            for n in 0..self.engine.analog_triggers[t].togs.len() {
                let k = self.engine.analog_triggers[t].togs[n];
                match edge {
                    Some(TriggerEdge::Press(velocity)) => self.press(k, velocity),
                    Some(TriggerEdge::Release) => self.release(k),
                    None => {}
                }
            }
        }
        self.refresh();
        &self.events
    }

    // Releases everything, including latched transpositions and held notes.
    pub fn all_off(&mut self) -> &[NoteEvent] {
        self.events.clear();
        self.keys_down.iter_mut().for_each(|k| *k = false);
        self.triggers.iter_mut().for_each(|t| *t = TriggerState::default());
        self.gut_layer.clear();
        self.gut_trnsp.iter_mut().flatten().for_each(|t| *t = TrnspState::default());
        for set in self.v_sets.iter_mut().chain(self.f_sets.iter_mut()).chain(self.c_sets.iter_mut()) {
            set.layer.clear();
            set.trnsp_all.iter_mut().for_each(|t| *t = TrnspState::default());
            set.trnsp_one.iter_mut().flatten().for_each(|t| *t = TrnspState::default());
        }
        self.refresh();
        &self.events
    }

    fn press(&mut self, k: usize, velocity: f64) {
        if !self.keys_down[k] {
            self.keys_down[k] = true;
            self.press_velocity = velocity.clamp(0.0, 1.0);
            self.refresh();
        }
    }

    fn release(&mut self, k: usize) {
        if self.keys_down[k] {
            self.keys_down[k] = false;
            self.refresh();
        }
    }

    // Brings every layer up to date with `keys_down`, then reports what changed.
    fn refresh(&mut self) {
        for g in 0..self.engine.guts.len() {
            let gut = &self.engine.guts[g];
            let down = any_down(&gut.togs, &self.keys_down);
            if self.gut_layer.set_pressed(g, down, &mut self.seq_counter) {
                self.voices[g].velocity = self.press_velocity;
            }
            for t in 0..gut.trnsp_gut.len() {
                let trnsp = &gut.trnsp_gut[t];
                self.gut_trnsp[g][t].update(any_down(&trnsp.triggers, &self.keys_down), trnsp.latch);
            }
        }
        self.gut_layer.update_holds(&self.engine.gut_holds, &self.keys_down);
        self.gut_layer.resolve(
            self.engine.gut_max_pressed,
            self.engine.gut_min_pressed,
            self.engine.gut_radio_mode,
            LIMIT_POLICY,
        );

        for s in 0..self.engine.v_multi.len() {
            self.engine.v_multi[s].update_state(&mut self.v_sets[s], &self.keys_down, &mut self.seq_counter);
        }
        for s in 0..self.engine.f_multi.len() {
            self.engine.f_multi[s].update_state(&mut self.f_sets[s], &self.keys_down, &mut self.seq_counter);
        }
        for s in 0..self.engine.c_multi.len() {
            self.engine.c_multi[s].update_state(&mut self.c_sets[s], &self.keys_down, &mut self.seq_counter);
        }

        self.emit_changes();
    }
}
