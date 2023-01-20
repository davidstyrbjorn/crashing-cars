use crate::prelude::*;

pub fn projectile_event(
    mut commands: Commands,
    mut event_reader: EventReader<ProjectileEvent>,
    mut query: Query<(&mut Health, Entity), With<Player>>,
) {
    for projectile_event in event_reader.iter() {
        // kill the projectile entity
        commands.entity(projectile_event.projectile).despawn();

        // remove hp from victim
        let victim = query
            .iter_mut()
            .find(|(_, entity)| *entity == projectile_event.hit_who);
        if let Some((mut hp, _)) = victim {
            if hp.0 > 0 {
                hp.0 -= 1;
            }
        }
    }
}
