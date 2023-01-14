use crate::prelude::*;

pub fn modification_input(
    mut commands: Commands,
    mut draft_resource: ResMut<DraftResource>,
    mut draft_pick_events: EventWriter<DraftPickEvent>,
    mut query: Query<(&ActionState<Action>, &Player, Entity), With<CurrentlyPicking>>,
) {
    if query.is_empty() {
        return;
    }

    let (input, player, entity) = query.single_mut();
    if input.just_pressed(Action::Rotate) {
        let axis_pair = input.axis_pair(Action::Rotate).unwrap();
        if axis_pair.x() > 0.0 {
            if (*draft_resource).current_idx < draft_resource.modifications.len() - 1 {
                draft_resource.current_idx += 1;
            }
        } else {
            if (*draft_resource).current_idx != 0 {
                draft_resource.current_idx -= 1;
            }
        }
    }

    // Select input?
    if input.just_pressed(Action::Select) {
        draft_pick_events.send(DraftPickEvent {
            modification: draft_resource.modifications[draft_resource.current_idx].clone(),
            who: entity,
            idx: draft_resource.current_idx,
            who_player: (*player).clone(),
        });

        // Remove from list
        let idx = draft_resource.current_idx;
        draft_resource.modifications.remove(idx);

        // And remove from current picker
        commands.entity(entity).remove::<CurrentlyPicking>();
    }
}
