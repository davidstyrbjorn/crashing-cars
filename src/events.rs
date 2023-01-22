use crate::prelude::*;

// Message for when draft pick was made
pub struct DraftPickEvent {
    pub modification: Modification,
    pub who: Entity,
    pub idx: usize,
    pub who_player: Player,
}

// Message for when modification state is done with its shit
pub struct ModificationDone;

pub struct ProjectileEvent {
    pub hit_who: Entity,
    pub projectile: Option<Entity>,
    pub damage: usize,
}

pub struct GoalEvent {
    pub team: Team,
}
