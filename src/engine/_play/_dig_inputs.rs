use crate::engine::*;

pub(crate) fn any_down(togs: &[usize], keys_down: &[bool]) -> bool {
    togs.iter().any(|&k| keys_down[k])
}

impl<Mode> Engine<Mode> {
    pub(crate) fn dig_idx(&self, key: Key) -> Option<usize> {
        self.dig_inputs.iter().position(|&k| k == key)
    }
    pub(crate) fn analog_idx(&self, analog: Analog) -> Option<usize> {
        self.analog_inputs.iter().position(|&a| a == analog)
    }
}
