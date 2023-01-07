mod components;
mod constants;
mod game_systems;
mod input;
mod spawner;

use std::time::Duration;

mod prelude {
    pub use crate::components::*;
    pub use crate::constants::*;
    pub use crate::game_systems::*;
    pub use crate::input::*;
    pub use crate::spawner::*;
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub enum GameState {
        MainMenu,
        InGame,
        InModification,
        Paused,
    }
    pub struct RoundTimerConfig {
        pub timer: Timer,
    }
    pub struct ScoreCounter {
        pub score: (u8, u8),
    }
    pub use bevy::prelude::*;
    pub use bevy::time::FixedTimestep;
    pub use bevy::window::*;
    pub use bevy_rapier2d::prelude::*;
    pub use leafwing_input_manager::prelude::*;
}

pub use prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Crashing Cars".into(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    })
    .add_state(GameState::InGame)
    .insert_resource(ClearColor(Color::DARK_GRAY))
    .insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..Default::default()
    })
    .insert_resource(RoundTimerConfig {
        timer: Timer::new(Duration::from_secs(30), false),
    })
    .insert_resource(ScoreCounter { score: (0, 0) })
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        PIXELS_PER_METER,
    ))
    .add_plugin(RapierDebugRenderPlugin::default())
    // This plugin maps inputs to an input-type agnostic action-state
    // We need to provide it with an enum which stores the possible actions a player could take
    .add_plugin(InputManagerPlugin::<Action>::default());

    // setup_systems_for_menu(&mut app);
    setup_systems_for_game(&mut app);

    app.run();
}

// fn setup_systems_for_menu(app: &mut App) {
//     app.add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(menu_ui_system))
//         .add_system_set(
//             SystemSet::on_enter(GameState::MainMenu).with_system(main_menu_setup_system),
//         )
//         .add_system_set(
//             SystemSet::on_exit(GameState::MainMenu).with_system(main_menu_cleanup_system),
//         );
// }
