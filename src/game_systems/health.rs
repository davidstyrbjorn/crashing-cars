use crate::prelude::*;

pub fn health_reset_position(mut query: Query<(&mut Transform, &mut Health, &Player)>) {
    // Reset transform of the health player
    for (mut transform, mut health, player) in query.iter_mut() {
        if health.0 <= 0 {
            health.0 = 10;
            transform.translation = player.spawn_position;
        }
    }
}
