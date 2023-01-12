use crate::prelude::*;

// Query for the signal component DraftPick and apply the effect
pub fn draft_pick(
    mut commands: Commands,
    query: Query<(&DraftPick, Entity)>,
    mut draft_resource: ResMut<DraftResource>,
) {
    let (draft_pick, entity) = query.single();

    // Process
    let modification_type = match draft_pick.modification.title.as_str() {
        "Outsorcing" => Some(ModificationType::GoalKeeper { to: entity }),
        "Furiously Fast" => Some(ModificationType::IncreaseSpeed { to: entity }),
        "Snappy Hamster" => Some(ModificationType::DecreaseDegrade { to: entity }),
        "Lock n' Load" => Some(ModificationType::Turret { to: entity }),
        "Mind Fuck" => Some(ModificationType::Inverted {
            to: entity,
            number_of_rounds: 2,
        }),
        // TODO: Counter should be some good value
        "Mutated Grounds" => Some(ModificationType::ModifyField { counter: 0 }),
        "Torture Chamber" => Some(ModificationType::AddHazard { counter: 0 }),
        "That MTV Show" => Some(ModificationType::ModifyCar {
            to: entity,
            counter: 0,
        }),
        _ => None,
    };

    // Apply
    if let Some(modification_type) = modification_type {
        Modifications::modification_picked(modification_type, commands);
    }

    let next = draft_resource.pick_order.pop_back();
    if let Some(next) = next {
        commands.entity(next).insert(CurrentlyPicking);
    } else {
        // No more, draft is done, switch back to game
    }

    commands.entity(entity).remove::<DraftPick>();
}
