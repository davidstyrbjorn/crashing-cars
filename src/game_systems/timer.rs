use crate::prelude::*;

// Handles everything to do with the timer
pub fn timer(
    mut query: Query<&mut Text, With<TimerText>>,
    mut timer_config: ResMut<RoundTimerConfig>,
    prepare_timer: Res<PrepareTimerResource>,
    mut app_state: ResMut<State<GameState>>,
    time: Res<Time>,
) {
    // Dont tick this timer if the prepare timer is running
    if !prepare_timer.0.finished() {
        return;
    }

    // Tick timer
    timer_config.timer.tick(time.delta());

    if timer_config.timer.finished() {
        // Swtich state to modification
        // TODO: Dangerous unwrap
        app_state.set(GameState::InModification).unwrap();
    }

    // Update UI
    let mut text = query.single_mut();
    text.sections[1].value = format!(
        "{}",
        ((timer_config.timer.duration().as_secs() as f32) - timer_config.timer.elapsed_secs())
            .floor()
    )
    .into();
}

pub fn prepare_timer(
    time: Res<Time>,
    mut prepare_timer: ResMut<PrepareTimerResource>,
    mut query: Query<&mut Text, With<PrepareTimerText>>,
) {
    prepare_timer.0.tick(time.delta());

    // Update UI
    let mut text = query.single_mut();
    let time_left = prepare_timer.0.duration().as_secs_f32() - prepare_timer.0.elapsed_secs();

    if prepare_timer.0.finished() {
        text.sections[0].value = String::from("");
    } else {
        text.sections[0].value = format!("{}", time_left.ceil());
    }
}
