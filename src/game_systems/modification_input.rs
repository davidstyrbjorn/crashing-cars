use crate::prelude::*;

pub fn modification_input(
    mut commands: Commands,
    mut draft_resource: ResMut<DraftResource>,
    mut query: Query<(&ActionState<Action>, Entity), With<CurrentlyPicking>>,
) {
    if query.is_empty() {
        return;
    }

    let (input, entity) = query.single_mut();
    if input.just_pressed(Action::Rotate) {
        let axis_pair = input.axis_pair(Action::Rotate).unwrap();
        if axis_pair.x() > 0.0 {
            if (*draft_resource).current_idx < MODIFICATION_COUNT - 1 {
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
        // Command for draft pick
        commands.spawn(DraftPick {
            modification: draft_resource.modifications[draft_resource.current_idx].clone(),
            who: entity,
        });

        // And remove from current picker
        commands.entity(entity).remove::<CurrentlyPicking>();
    }
}
