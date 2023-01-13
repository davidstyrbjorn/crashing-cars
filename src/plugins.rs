use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
enum Label {
    Pre,
    Apply,
    Main,
}

pub struct GamePlugin;
pub struct MenuPlugin;
pub struct ModificationPlugin;
pub struct SharedPlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(Label::Pre)
                .before(Label::Main)
                .with_system(reset_base_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(Label::Apply)
                .before(Label::Main)
                .after(Label::Pre)
                .with_system(modifier_angular_speed)
                .with_system(modifier_angular_degrade)
                .with_system(modifier_linear_speed),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(Label::Main)
                .after(Label::Apply)
                .with_system(ball)
                .with_system(player_move)
                .with_system(score_text)
                .with_system(timer),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(setup_game)
                .with_system(despawn_entities::<OnMainMenu>),
        )
        .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(on_round_end));
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
        )
        .add_system_set(
            SystemSet::on_update(GameState::InModification)
                .with_system(sliding_window)
                .with_system(modification_input)
                .with_system(highlight_modification_element)
                .with_system(draft_pick),
        );
    }
}

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_shared);
    }
}
