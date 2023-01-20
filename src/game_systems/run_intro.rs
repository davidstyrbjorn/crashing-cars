use crate::prelude::*;

// New game state called Intro
// We have a timer
// We can query for all the IntroTextPart entities
// We need a resource to let us know which text part we are on or we can just grab the lowest
// wait for timer to run to X then reset again and when there are no more in the query we switch state on the last

// return x === 1 ? 1 : 1 - Math.pow(2, -10 * x);
// sqrt(-4 * (t - 0.5)^2 + 1)
// at t = 0 we get 0 and t = 1 we get 0, in between we get some nice interpolated values

fn easer(t: f32) -> f32 {
    f32::sqrt(-4.0 * f32::powf(t - 0.5, 2.0) + 1.0)
}

pub fn run_intro(
    mut commands: Commands,
    mut query: Query<(&mut Text, &IntroText, Entity)>,
    mut intro_timer: ResMut<IntroTimerResource>,
    mut app_state: ResMut<State<GameState>>,
    time: Res<Time>,
) {
    if query.is_empty() {
        app_state.set(GameState::MainMenu).unwrap();
        return;
    }

    let t = intro_timer.timer.percent_left();

    // Find IntroText with the lowest order value
    let mut lowest_and_index: (usize, usize) = (1000, 0);
    let mut lowest_entity: Option<Entity> = None;
    query.iter().enumerate().for_each(|(index, entry)| {
        if entry.1.order < lowest_and_index.0 {
            lowest_and_index = (entry.1.order, index);
            lowest_entity = Some(entry.2);
        }
    });

    query.iter_mut().enumerate().for_each(|(idx, mut entry)| {
        if idx == lowest_and_index.1 {
            let a = easer(t);
            entry.0.sections[0].style.color = Color::rgba(1.0, 1.0, 1.0, a);
        } else {
            entry.0.sections[0].style.color = Color::rgba(0.0, 0.0, 0.0, 0.0);
        }
    });

    intro_timer.timer.tick(time.delta());
    if intro_timer.timer.finished() {
        if let Some(lowest_entity) = lowest_entity {
            commands.entity(lowest_entity).despawn();
        }
        intro_timer.timer.reset();
    }
}
