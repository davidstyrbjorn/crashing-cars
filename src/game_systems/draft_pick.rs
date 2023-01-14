use crate::prelude::*;

// On draft pick event, we need to actually add
pub fn draft_pick(
    mut draft_resource: ResMut<DraftResource>,
    mut events: EventReader<DraftPickEvent>,
    mut modification_events: EventWriter<ModificationDone>,
    mut app_state: ResMut<State<GameState>>,
    mut modifications: ResMut<Modifications>,
    mut commands: Commands,
) {
    for draft_pick in events.iter() {
        println!("RUNNING: {}", draft_pick.modification.title);
        // Process
        let modification_type = match draft_pick.modification.title.as_str() {
            "Outsourcing" => Some(ModificationType::GoalKeeper {
                team: draft_pick.who_player.team.clone(),
            }),
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

        // Apply modification to game
        if let Some(modification_type) = modification_type {
            Modifications::modification_picked(modification_type, &mut commands);
        } else {
            panic!("RUNNING A MODIFICATION WITH NO DATA...");
        }

        // Pick next
        let next = draft_resource.pick_order.pop_back();
        draft_resource.current_idx = 0;
        if let Some(next) = next {
            commands.entity(next).insert(CurrentlyPicking);
        } else {
            app_state.set(GameState::InGame).unwrap();
            // No more, draft is done, switch back to game
            modification_events.send(ModificationDone);
            draft_resource.modifications.iter().for_each(|mf| {
                modifications.modifications.push(mf.clone());
            });
        }
    }
}
