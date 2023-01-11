use crate::prelude::*;

pub fn sliding_window(time: Res<Time>, mut query: Query<(&mut Style, &mut SlidingWindow)>) {
    for (mut style, mut sliding_window) in query.iter_mut() {
        sliding_window.0 += 100.0 * time.delta_seconds();

        style.position.top = Val::Percent(f32::min(sliding_window.0, 10.0));
    }
}
