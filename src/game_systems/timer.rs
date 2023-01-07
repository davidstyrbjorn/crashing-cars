use crate::prelude::*;

// Handles everything to do with the timer
pub fn timer(
    mut query: Query<&mut Text, With<TimerText>>,
    mut timer_config: ResMut<RoundTimerConfig>,
    time: Res<Time>,
) {
    // Tick timer
    timer_config.timer.tick(time.delta());

    // Update UI
    let mut text = query.single_mut();
    text.sections[1].value = format!(
        "{}",
        ((timer_config.timer.duration().as_secs() as f32) - timer_config.timer.elapsed_secs())
            .floor()
    )
    .into();
}
