use bevy::input::ButtonState;
use leafwing_input_manager::{action_state::ActionData, axislike::DualAxisData};

use crate::prelude::*;

fn player_outside(position: Vec3) -> bool {
    return position.x < (-WINDOW_WIDTH / 2.0) - PLAYER_SIZE.x
        || position.x > (WINDOW_WIDTH / 2.0) + PLAYER_SIZE.x;
}

pub fn player_move(
    time: Res<Time>,
    prepare_timer: Res<PrepareTimerResource>,
    mut query: Query<
        (
            &ActionState<Action>,
            &mut Transform,
            &mut Velocity,
            &Player,
            &BaseStats,
            Option<&InvertedControls>,
            Option<&Boost>,
            Option<&CharliesWildcard>,
        ),
        With<Player>,
    >,
) {
    if prepare_timer.0.finished() {
        for (
            input,
            mut transform,
            mut velocity,
            player,
            base_stat,
            inverted_controls,
            boost,
            charlies_wildcard,
        ) in query.iter_mut()
        {
            let invert_controls = inverted_controls.is_some();
            let boost = boost.is_some();
            let charlies_wildcard = charlies_wildcard.is_some();
            if input.pressed(Action::Rotate) {
                let axis_pair = input.axis_pair(Action::Rotate).unwrap();
                let mut delta = axis_pair.x() * base_stat.angular_speed * time.delta_seconds();
                if invert_controls {
                    delta *= -1.0;
                }
                velocity.angvel -= delta;
                velocity.angvel = velocity.angvel.clamp(-5.0, 5.0);
            } else {
                velocity.angvel = velocity.angvel * base_stat.angular_degrade;
            }
            if input.pressed(Action::Move) || charlies_wildcard {
                let axis_pair = input.axis_pair(Action::Move).unwrap();
                let mut axis_pair = axis_pair.y();
                if charlies_wildcard {
                    axis_pair = 1.0;
                }
                let movement_direction = transform.rotation * Vec3::Y;
                let mut x =
                    movement_direction * axis_pair * time.delta_seconds() * base_stat.linear_speed;
                if boost {
                    x *= 2.0;
                }
                velocity.linvel += Vec2::new(x.x, x.y);
            } else {
                velocity.linvel = velocity.linvel * 0.9;
            }

            if player_outside(transform.translation) {
                transform.translation = player.spawn_position;
                velocity.linvel = Vec2::new(0.0, 0.0);
            }
        }
    }
}
