#[derive(Debug, Clone, Default)]
pub(crate) struct TrnspState {
    pub(crate) active: bool,
    down: bool,
}

impl TrnspState {
    // Momentary: active while any trigger is held. Latching: each new press flips it.
    pub(crate) fn update(&mut self, down: bool, latch: bool) {
        if latch {
            if down && !self.down {
                self.active = !self.active;
            }
        } else {
            self.active = down;
        }
        self.down = down;
    }
}
