mod components;
mod constants;
mod events;
mod game_systems;
mod input;
mod modifications;
mod plugins;
mod spawner;

use std::{collections::VecDeque, time::Duration};

mod prelude {
    pub use crate::components::*;
    pub use crate::constants::*;
    pub use crate::events::*;
    pub use crate::game_systems::*;
    pub use crate::input::*;
    pub use crate::modifications::*;
    pub use crate::plugins::*;
    pub use crate::spawner::*;
    use std::collections::VecDeque;
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub enum GameState {
        Intro,
        MainMenu,
        InGame,
        InModification,
        Paused,
    }
    #[derive(Resource)]
    pub struct RoundTimerConfig {
        pub timer: Timer,
    }
    #[derive(Resource)]
    pub struct IntroTimerResource {
        pub timer: Timer,
    }
    #[derive(Resource)]
    pub struct PrepareTimerResource(pub Timer); // Resource used to count down the start of round
    #[derive(Resource)]
    pub struct ScoreCounter {
        pub score: (u8, u8),
    }
    #[derive(Resource)]
    pub struct CameraShakeResource(pub f32); // How long
    #[derive(Resource)]
    pub struct DraftResource {
        pub current_idx: usize,
        pub modifications: Vec<Modification>,
        pub pick_order: VecDeque<Entity>,
    }
    #[derive(Resource)]
    pub struct StartupFlags {
        pub in_game: bool,
    }
    #[derive(Resource)]
    pub struct Players(pub Vec<Entity>);
    pub struct ModificationResource {}
    pub use bevy::prelude::*;
    pub use bevy::time::FixedTimestep;
    pub use bevy::window::*;
    pub use bevy_rapier2d::prelude::*;
    pub use leafwing_input_manager::prelude::*;
}

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
pub use prelude::*;

fn main() {
    let mut app = App::new();

    app.add_state(GameState::InGame)
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .insert_resource(PrepareTimerResource(Timer::new(
            Duration::from_secs(3),
            TimerMode::Once,
        )))
        .insert_resource(RoundTimerConfig {
            timer: Timer::new(Duration::from_secs(20), TimerMode::Once),
        })
        .insert_resource(Modifications::load())
        .insert_resource(CameraShakeResource(0.0))
        .insert_resource(DraftResource {
            current_idx: 0,
            modifications: Vec::new(),
            pick_order: VecDeque::new(),
        })
        .insert_resource(ScoreCounter { score: (0, 0) })
        .insert_resource(StartupFlags { in_game: false })
        .insert_resource(Players(Vec::new()))
        .add_event::<DraftPickEvent>()
        .add_event::<ModificationDone>()
        .add_event::<ProjectileEvent>()
        .add_event::<GoalEvent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Crashing Cars".into(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_plugin(SharedPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(IntroPlugin)
        .add_plugin(ModificationPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        // This plugin maps inputs to an input-type agnostic action-state
        // We need to provide it with an enum which stores the possible actions a player could take
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default());

    app.run();
}
