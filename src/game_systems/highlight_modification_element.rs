use crate::prelude::*;

pub fn highlight_modification_element(
    draft_resource: Res<DraftResource>,
    mut query: Query<(&ModificationElement, &mut BackgroundColor)>,
) {
    query
        .iter_mut()
        .for_each(|(modification_element, mut background_color)| {
            let selected = draft_resource.current_idx == modification_element.order;
            if selected {
                *background_color = Color::rgb(0.4, 0.3, 0.3).into();
            } else {
                *background_color = Color::rgb(0.95, 0.95, 0.95).into();
            }
        });
}
