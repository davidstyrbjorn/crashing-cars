use crate::prelude::*;

fn player_outside(position: Vec3) -> bool {
    return position.x < (-WINDOW_WIDTH / 2.0) - PLAYER_SIZE.x
        || position.x > (WINDOW_WIDTH / 2.0) + PLAYER_SIZE.x;
}

pub fn player_move(
    time: Res<Time>,
    mut query: Query<
        (
            &ActionState<Action>,
            &mut ExternalForce,
            &mut Transform,
            &mut Velocity,
            &Player,
        ),
        With<Player>,
    >,
) {
    for (input, mut _force, mut transform, mut velocity, player) in query.iter_mut() {
        if input.pressed(Action::Move) {
            let axis_pair = input.clamped_axis_pair(Action::Move).unwrap();
            // force.force.x += MOVE_SPEED * axis_pair.x() * time.delta_seconds();
            // force.force.y += MOVE_SPEED * axis_pair.y() * time.delta_seconds();
            // force.force = force.force.clamp_length(0.0, TERMINAL_FORCE);
            velocity.linvel =
                Vec2::new(axis_pair.x(), axis_pair.y()) * MOVE_SPEED * time.delta_seconds();

            if player_outside(transform.translation) {
                transform.translation = player.spawn_position;
                velocity.linvel = Vec2::new(0.0, 0.0);
            }
        }
        // else {
        //     force.force = Vec2::ZERO;
        // }
    }
}
