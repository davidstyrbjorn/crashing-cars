use crate::prelude::*;

pub fn health_reset_position(
    mut query: Query<(&mut Transform, &mut Velocity, &mut Health, &Player)>,
) {
    // Reset transform of the health player
    for (mut transform, mut velocity, mut health, player) in query.iter_mut() {
        if health.0 <= 0 {
            // Reset health, transform and physics
            health.0 = 10;
            transform.translation = player.spawn_position;
            transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, player.spawn_rotation);
            velocity.angvel = 0.0;
            velocity.linvel = Vec2::ZERO;
        }
    }
}
