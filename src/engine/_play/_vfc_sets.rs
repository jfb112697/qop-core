use std::default::Default;
use crate::engine::{
    _play::{
        _dig_inputs::any_down,
        _holds::{LIMIT_POLICY, Layer},
        _trnsp::TrnspState,
    },
    *,
};
use duplicate::duplicate_item;

#[derive(Debug, Clone, Default)]
pub(crate) struct SetState {
    pub(crate) layer: Layer,
    pub(crate) trnsp_all: Vec<TrnspState>,
    pub(crate) trnsp_one: Vec<Vec<TrnspState>>,
}

fn add_deltas(
    i_delta: &[i32],
    x_delta: &[f64],
    trnsp: &[MulTrnsp],
    trnsp_state: &[TrnspState],
    g: usize,
    i: &mut i64,
    x: &mut f64,
) {
    *i += i_delta[g] as i64;
    *x += x_delta[g];
    add_active_trnsp(trnsp, trnsp_state, g, i, x);
}

fn add_active_trnsp(trnsp: &[MulTrnsp], trnsp_state: &[TrnspState], g: usize, i: &mut i64, x: &mut f64) {
    for t in 0..trnsp.len() {
        if trnsp_state[t].active {
            *i += trnsp[t].i_delta[g] as i64;
            *x += trnsp[t].x_delta[g];
        }
    }
}

#[duplicate_item(
    SetType    deltafield;
    [VFSet]    [buttons];
    [ComboSet] [combos];
)]
impl SetType {
    pub(crate) fn new_state(&self) -> SetState {
        SetState {
            layer: Layer::new(self.buttons.len()),
            trnsp_all: vec![TrnspState::default(); self.trnsp_all.len()],
            trnsp_one: self
                .deltafield
                .iter()
                .map(|d| vec![TrnspState::default(); d.trnsp_one.len()])
                .collect(),
        }
    }

    pub(crate) fn update_state(&self, state: &mut SetState, keys_down: &[bool], seq_counter: &mut u64) {
        for b in 0..self.buttons.len() {
            state
                .layer
                .set_pressed(b, any_down(&self.buttons[b].togs, keys_down), seq_counter);
        }
        state.layer.update_holds(&self.holds, keys_down);
        state
            .layer
            .resolve(self.max_pressed, self.min_pressed, self.radio_mode, LIMIT_POLICY);
        for t in 0..self.trnsp_all.len() {
            state.trnsp_all[t].update(any_down(&self.trnsp_all[t].triggers, keys_down), self.trnsp_all[t].latch);
        }
        for d in 0..self.deltafield.len() {
            for t in 0..self.deltafield[d].trnsp_one.len() {
                let trnsp = &self.deltafield[d].trnsp_one[t];
                state.trnsp_one[d][t].update(any_down(&trnsp.triggers, keys_down), trnsp.latch);
            }
        }
    }
}

impl VFSet {
    // Valve-style: every active button shifts the gut, and the shifts add up.
    pub(crate) fn add_summed(&self, state: &SetState, g: usize, i: &mut i64, x: &mut f64) {
        for b in 0..self.buttons.len() {
            if state.layer.active[b] {
                let btn = &self.buttons[b];
                add_deltas(&btn.i_delta, &btn.x_delta, &btn.trnsp_one, &state.trnsp_one[b], g, i, x);
            }
        }
        add_active_trnsp(&self.trnsp_all, &state.trnsp_all, g, i, x);
    }

    // Fret-style: only the active button that raises this gut the most applies.
    // Ties go to the larger x_delta, then to the later button.
    pub(crate) fn add_highest(&self, state: &SetState, g: usize, i: &mut i64, x: &mut f64) {
        let mut best: Option<usize> = None;
        for b in 0..self.buttons.len() {
            if state.layer.active[b] {
                let higher = best.is_none_or(|o| {
                    let (new, old) = (&self.buttons[b], &self.buttons[o]);
                    new.i_delta[g] > old.i_delta[g]
                        || (new.i_delta[g] == old.i_delta[g] && new.x_delta[g] >= old.x_delta[g])
                });
                if higher {
                    best = Some(b);
                }
            }
        }
        if let Some(b) = best {
            let btn = &self.buttons[b];
            add_deltas(&btn.i_delta, &btn.x_delta, &btn.trnsp_one, &state.trnsp_one[b], g, i, x);
        }
        add_active_trnsp(&self.trnsp_all, &state.trnsp_all, g, i, x);
    }
}

impl ComboSet {
    // Fingering-chart style: the first combo whose pattern equals the active buttons exactly applies.
    pub(crate) fn add_matching(&self, state: &SetState, g: usize, i: &mut i64, x: &mut f64) {
        let matching = self
            .combos
            .iter()
            .position(|c| c.combo.iter().zip(state.layer.active.iter()).all(|(a, b)| a == b));
        if let Some(c) = matching {
            let combo = &self.combos[c];
            add_deltas(&combo.i_delta, &combo.x_delta, &combo.trnsp_one, &state.trnsp_one[c], g, i, x);
        }
        add_active_trnsp(&self.trnsp_all, &state.trnsp_all, g, i, x);
    }
}
