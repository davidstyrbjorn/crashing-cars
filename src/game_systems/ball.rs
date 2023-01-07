use crate::prelude::*;

fn ball_scored(position: Vec3) -> (bool, bool) {
    let bounds = (
        position.x > WINDOW_WIDTH / 2.0,
        position.x < -WINDOW_WIDTH / 2.0,
    );
    bounds
}

pub fn ball(
    mut score_counter: ResMut<ScoreCounter>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    // Check if ball transform is valid goal position
    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        let bounds = ball_scored(transform.translation);
        if bounds.0 || bounds.1 {
            velocity.linvel = Vec2::new(0.0, 0.0);
            transform.translation = Vec3::new(0.0, 0.0, 0.0);

            // Increment score
            score_counter.score.0 += bounds.0 as u8;
            score_counter.score.1 += bounds.1 as u8;

            println!("{} - {}", score_counter.score.0, score_counter.score.1);
        }
    }
}
