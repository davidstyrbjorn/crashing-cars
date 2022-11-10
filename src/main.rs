mod components;
mod constants;
mod input;
mod spawner;
mod systems;

use std::{
    borrow::{Borrow, BorrowMut},
    time::Duration,
};

mod prelude {
    pub use crate::components::*;
    pub use crate::constants::*;
    pub use crate::input::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use bevy::prelude::*;
    pub use bevy::time::FixedTimestep;
    pub use bevy::window::*;
    pub use bevy_rapier2d::prelude::*;
    pub use leafwing_input_manager::prelude::*;
}

use bevy::window::close_on_esc;
use prelude::*;

// 71x131

pub struct RoundTimerConfig {
    timer: Timer,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    InModification,
    Paused,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Crashing Cars".into(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    })
    .add_state(GameState::MainMenu)
    .insert_resource(ClearColor(Color::DARK_GRAY))
    .insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..Default::default()
    })
    .insert_resource(RoundTimerConfig {
        timer: Timer::new(Duration::from_secs(30), false),
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        PIXELS_PER_METER,
    ))
    .add_plugin(RapierDebugRenderPlugin::default())
    // This plugin maps inputs to an input-type agnostic action-state
    // We need to provide it with an enum which stores the possible actions a player could take
    .add_plugin(InputManagerPlugin::<Action>::default());

    setup_systems_for_menu(&mut app);
    setup_systems_for_game(&mut app);

    app.run();
}

fn setup_systems_for_menu(app: &mut App) -> &mut App {
    app.add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(menu_ui_system))
        .add_system_set(
            SystemSet::on_enter(GameState::MainMenu).with_system(main_menu_setup_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::MainMenu).with_system(main_menu_cleanup_system),
        )
}

fn setup_systems_for_game(app: &mut App) -> &mut App {
    app.add_system_set(
        SystemSet::on_update(GameState::InGame)
            .with_system(player_move_system)
            .with_system(round_text_system),
    )
    .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_game))
}
