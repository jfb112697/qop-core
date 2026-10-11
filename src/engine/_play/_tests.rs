use crate::engine::_play::_holds::{Layer, LimitPolicy};

fn press(layer: &mut Layer, m: usize, seq: &mut u64, max: usize, policy: LimitPolicy) {
    layer.set_pressed(m, true, seq);
    layer.resolve(max, 0, false, policy);
}

#[test]
fn newest_wins_releases_the_oldest_at_the_limit() {
    let (mut layer, mut seq) = (Layer::new(3), 0);
    press(&mut layer, 0, &mut seq, 2, LimitPolicy::NewestWins);
    press(&mut layer, 1, &mut seq, 2, LimitPolicy::NewestWins);
    press(&mut layer, 2, &mut seq, 2, LimitPolicy::NewestWins);
    assert_eq!(layer.active, vec![false, true, true]);
}

#[test]
fn ignore_new_keeps_the_oldest_at_the_limit() {
    let (mut layer, mut seq) = (Layer::new(3), 0);
    press(&mut layer, 0, &mut seq, 2, LimitPolicy::IgnoreNew);
    press(&mut layer, 1, &mut seq, 2, LimitPolicy::IgnoreNew);
    press(&mut layer, 2, &mut seq, 2, LimitPolicy::IgnoreNew);
    assert_eq!(layer.active, vec![true, true, false]);

    // The ignored press stays ignored until it is released and pressed again.
    layer.set_pressed(0, false, &mut seq);
    layer.resolve(2, 0, false, LimitPolicy::IgnoreNew);
    assert_eq!(layer.active, vec![false, true, false]);
    layer.set_pressed(2, false, &mut seq);
    press(&mut layer, 2, &mut seq, 2, LimitPolicy::IgnoreNew);
    assert_eq!(layer.active, vec![false, true, true]);
}
