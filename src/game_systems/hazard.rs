use crate::prelude::*;

// Spin the hazard and check for collision with players
pub fn hazard(
    players: Res<Players>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, Entity), With<Hazard>>,
    rapier_context: Res<RapierContext>,
    mut event_writer: EventWriter<ProjectileEvent>,
) {
    for (mut transform, entity) in query.iter_mut() {
        transform.rotate_z(HAZARD_ROTATION_SPEED * time.delta_seconds());

        players.0.iter().for_each(|player| {
            if let Some(_) = rapier_context.contact_pair(entity, *player) {
                event_writer.send(ProjectileEvent {
                    hit_who: *player,
                    damage: 99,
                    projectile: None,
                });
            }
        });
    }
}
