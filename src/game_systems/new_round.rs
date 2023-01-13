use crate::prelude::*;

pub fn new_round_player(
    mut events: EventReader<ModificationDone>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    // Reset player position
    for _ in events.iter() {
        query.iter_mut().for_each(|(mut transform, player)| {
            transform.translation = player.spawn_position;
        });
    }
}

pub fn new_round_ball(
    mut events: EventReader<ModificationDone>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    // Reset ball position and other such properties
    for _ in events.iter() {
        if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
            velocity.linvel = Vec2::new(0.0, 0.0);
            transform.translation = Vec3::new(0.0, 0.0, 0.0);
        }
    }
}

pub fn new_round_timer(
    mut events: EventReader<ModificationDone>,
    mut round_timer: ResMut<RoundTimerConfig>,
) {
    // Reset round timer
    for _ in events.iter() {
        round_timer.timer.reset();
    }
}

pub fn new_round_score_counter(
    mut events: EventReader<ModificationDone>,
    mut score_counter: ResMut<ScoreCounter>,
) {
    // Reset score
    for _ in events.iter() {
        score_counter.score = (0, 0);
    }
}