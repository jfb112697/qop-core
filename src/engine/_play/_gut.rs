use crate::engine::_play::*;

impl Player {
    // The pitch gut `g` has right now: its base output plus everything currently shifting it.
    pub(crate) fn pitch(&self, g: usize) -> (i32, f64) {
        let engine = &self.engine;
        let gut = &engine.guts[g];
        let (mut i, mut x) = (0i64, 0f64);

        for t in 0..gut.trnsp_gut.len() {
            if self.gut_trnsp[g][t].active {
                i += gut.trnsp_gut[t].i_delta as i64;
                x += gut.trnsp_gut[t].x_delta;
            }
        }
        for s in 0..engine.v_multi.len() {
            engine.v_multi[s].add_summed(&self.v_sets[s], g, &mut i, &mut x);
        }
        for s in 0..engine.f_multi.len() {
            engine.f_multi[s].add_highest(&self.f_sets[s], g, &mut i, &mut x);
        }
        for s in 0..engine.c_multi.len() {
            engine.c_multi[s].add_matching(&self.c_sets[s], g, &mut i, &mut x);
        }
        for m in engine.analog_mods.iter() {
            x += self.analog_values[m.input] * m.x_delta[g];
        }

        let index = if engine.index_delta_bool {
            (gut.index_out as i64 + i).clamp(i32::MIN as i64, i32::MAX as i64) as i32
        } else {
            gut.index_out
        };
        let extra = if engine.extra_delta_bool { gut.extra_out + x } else { 0.0 };
        (index, extra)
    }

    // Compares every gut with what it was doing before and appends the events that describe the difference.
    pub(crate) fn emit_changes(&mut self) {
        for g in 0..self.voices.len() {
            let sounding = self.gut_layer.active[g];
            let (index, extra) = self.pitch(g);
            let before = self.voices[g];
            let velocity = before.velocity;
            let on = NoteEvent::On { gut: g, index, extra, velocity };

            if sounding && !before.sounding {
                self.events.push(on);
            } else if !sounding && before.sounding {
                self.events.push(NoteEvent::Off { gut: g });
            } else if sounding && (index != before.index || extra != before.extra) {
                if self.engine.guts[g].retrigger {
                    self.events.push(NoteEvent::Off { gut: g });
                    self.events.push(on);
                } else {
                    self.events.push(NoteEvent::Change { gut: g, index, extra });
                }
            }
            self.voices[g] = Voice { sounding, index, extra, velocity };
        }
    }
}
