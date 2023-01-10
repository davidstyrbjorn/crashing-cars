use crate::prelude::*;

pub fn setup_shared(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}
