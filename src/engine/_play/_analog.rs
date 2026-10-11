use std::default::Default;
use crate::engine::*;

// The quietest MIDI note-on, so a slow pull still sounds.
const MIN_VELOCITY: f64 = 1.0 / 127.0;

#[derive(Debug, Clone, Default)]
pub(crate) struct TriggerState {
    pub(crate) down: bool,
    armed_at: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum TriggerEdge {
    Press(f64),
    Release,
}

impl AnalogTrigger {
    // Turns a continuous value into a key press with a velocity, the way a velocity-sensitive keyboard does:
    // the faster the value travels from `low` to `high`, the higher the velocity.
    pub(crate) fn update(&self, state: &mut TriggerState, value: f64, time_secs: f64) -> Option<TriggerEdge> {
        if state.down {
            if value < self.low {
                state.down = false;
                state.armed_at = None;
                return Some(TriggerEdge::Release);
            }
            return None;
        }
        if value < self.low {
            state.armed_at = None;
            return None;
        }
        let armed_at = *state.armed_at.get_or_insert(time_secs);
        if value >= self.high {
            state.down = true;
            return Some(TriggerEdge::Press(self.velocity(time_secs - armed_at)));
        }
        None
    }

    fn velocity(&self, travel_secs: f64) -> f64 {
        if travel_secs <= self.fast_secs {
            1.0
        } else if travel_secs >= self.slow_secs {
            MIN_VELOCITY
        } else {
            let slowness = (travel_secs - self.fast_secs) / (self.slow_secs - self.fast_secs);
            1.0 - slowness * (1.0 - MIN_VELOCITY)
        }
    }
}
