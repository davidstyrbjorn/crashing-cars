use crate::prelude::*;

// Query for the signal component DraftPick and apply the effect
pub fn draft_pick(
    mut draft_resource: ResMut<DraftResource>,
    mut commands: Commands,
    query: Query<(&DraftPick, Entity)>,
) {
    if query.is_empty() {
        return;
    }
    let (draft_pick, entity) = query.single();

    // Process
    let modification_type = match draft_pick.modification.title.as_str() {
        "Outsorcing" => Some(ModificationType::GoalKeeper { to: draft_pick.who }),
        "Furiously Fast" => Some(ModificationType::IncreaseSpeed { to: draft_pick.who }),
        "Snappy Hamster" => Some(ModificationType::DecreaseDegrade { to: draft_pick.who }),
        "Lock n' Load" => Some(ModificationType::Turret { to: draft_pick.who }),
        "Mind Fuck" => Some(ModificationType::Inverted {
            to: draft_pick.who,
            number_of_rounds: 2,
        }),
        // TODO: Counter should be some good value
        "Mutated Grounds" => Some(ModificationType::ModifyField { counter: 0 }),
        "Torture Chamber" => Some(ModificationType::AddHazard { counter: 0 }),
        "That MTV Show" => Some(ModificationType::ModifyCar {
            to: draft_pick.who,
            counter: 0,
        }),
        _ => None,
    };

    // Apply
    if let Some(modification_type) = modification_type {
        Modifications::modification_picked(modification_type, &mut commands);
    }

    let next = draft_resource.pick_order.pop_back();
    if let Some(next) = next {
        commands.entity(next).insert(CurrentlyPicking);
    } else {
        // No more, draft is done, switch back to game
        println!("NO MORE");
    }

    commands.entity(entity).remove::<DraftPick>();
}
