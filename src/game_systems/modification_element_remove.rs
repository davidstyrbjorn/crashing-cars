use crate::prelude::*;

pub fn modification_element_remove(
    mut events: EventReader<DraftPickEvent>,
    mut query: Query<(&mut ModificationElement, Entity)>,
    mut commands: Commands,
) {
    for draft_pick in events.iter() {
        let idx = draft_pick.idx;

        // remove the entity that was picked (UI element)
        let entity = query
            .iter()
            .filter(|(me, _)| me.order == idx)
            .for_each(|(_, entity)| {
                println!("ME AT BALLS");
                commands.entity(entity).despawn_recursive();
            });

        // decrement step, we want cards above the picked one to move on order down
        println!("{}", idx);
        query
            .iter_mut()
            .filter(|(modification_element, _)| modification_element.order > idx)
            .for_each(|(mut me, _)| {
                me.order -= 1;
            });
    }
}
