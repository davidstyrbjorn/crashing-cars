use crate::prelude::*;

pub fn inverted_controls_degrade(
    mut commands: Commands,
    mut query: Query<(&mut InvertedControls, Entity)>,
) {
    for (mut inverted_controls, entity) in query.iter_mut() {
        // Decrement internal counter and remove component if zero
        inverted_controls.0 -= 1;
        if inverted_controls.0 == 0 {
            commands.entity(entity).remove::<InvertedControls>();
        }
    }
}
