use crate::prelude::*;

pub fn projectile(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Projectile, &mut Transform, Entity)>,
) {
    for (projectile, mut transform, entity) in query.iter_mut() {
        transform.translation += projectile.direction * PROJECTILE_SPEED * time.delta_seconds();
        if (time.elapsed().as_secs() - projectile.spawned_at_time) > PROJECTILE_LIFE_LENGTH {
            commands.entity(entity).despawn();
        }
    }
}
