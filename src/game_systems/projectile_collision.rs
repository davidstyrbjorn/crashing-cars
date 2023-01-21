use crate::prelude::*;

pub fn projectile_collision_player(
    rapier_context: Res<RapierContext>,
    players: Res<Players>,
    query: Query<Entity, With<Projectile>>,
    mut event_writer: EventWriter<ProjectileEvent>,
) {
    if players.0.len() == 0 {
        return;
    }

    for entity in query.iter() {
        players.0.iter().for_each(|player| {
            if let Some(_) = rapier_context.contact_pair(*player, entity) {
                event_writer.send(ProjectileEvent {
                    hit_who: *player,
                    projectile: entity,
                });
            }
        });
    }
}
