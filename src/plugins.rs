use crate::prelude::*;

pub struct GamePlugin;
pub struct MenuPlugin;
pub struct ModificationPlugin;
pub struct SharedPlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_move)
                .with_system(timer)
                .with_system(ball)
                .with_system(score_text),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(setup_game)
                .with_system(despawn_entities::<OnMainMenu>),
        );
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::MainMenu).with_system(press_to_start_game),
        )
        .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu));
    }
}

impl Plugin for ModificationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InModification).with_system(setup_modification),
        );
    }
}

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_shared);
    }
}
