use crate::{prelude::*, GameState, RoundTimerConfig};

pub fn player_move_system(
    time: Res<Time>,
    mut query: Query<(&ActionState<Action>, &mut ExternalForce), With<Player>>,
) {
    for (input, mut force) in query.iter_mut() {
        if input.pressed(Action::Move) {
            let axis_pair = input.clamped_axis_pair(Action::Move).unwrap();
            force.force.x += MOVE_SPEED * axis_pair.x() * time.delta_seconds();
            force.force.y += MOVE_SPEED * axis_pair.y() * time.delta_seconds();
            force.force = force.force.clamp_length(0.0, TERMINAL_FORCE);
        } else {
            force.force = Vec2::ZERO;
        }
    }
}

pub fn physics_debug_system(query: Query<&ExternalForce, With<Player>>) {
    for rb in query.iter() {
        println!("Force X: {}", rb.force.x)
    }
}

// Ticks the round timer and updates UI
pub fn round_text_system(
    mut query: Query<&mut Text, With<TimerText>>,
    mut timer_config: ResMut<RoundTimerConfig>,
    time: Res<Time>,
) {
    timer_config.timer.tick(time.delta());

    let mut text = query.single_mut();
    text.sections[1].value = format!(
        "{}",
        ((timer_config.timer.duration().as_secs() as f32) - timer_config.timer.elapsed_secs())
            .floor()
    )
    .into();
}

pub fn main_menu_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn title text
    commands.spawn_bundle(TextBundle::from_section(
        TextSection::new(
            "TIME: ".into(),
            TextStyle {
                font: asset_server.load("fonts/Modern_Mono.otf"),
                font_size: 45.0,
                color: Color::WHITE,
            },
        ),
        Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
    ));
}

pub fn main_menu_cleanup_system() {
    println!("RUNNING CLEANUP ON ISLE 5");
}

// Handle game start
pub fn menu_ui_system(mut app_state: ResMut<State<GameState>>) {
    //app_state.set(GameState::InGame).unwrap();
}

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle {
        ..Default::default()
    });

    let entity = spawn_player(
        &mut commands,
        VirtualDPad::arrow_keys(),
        Vec3::new(-WINDOW_WIDTH / 4.0, 0.0, 0.0),
    );
    commands.entity(entity).insert(PhysicsDebug);
    spawn_player(
        &mut commands,
        VirtualDPad::wasd(),
        Vec3::new(WINDOW_WIDTH / 4.0, 0.0, 0.0),
    );

    spawn_level(&mut commands);

    // Spawn round timer text
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "TIME: ",
                    TextStyle {
                        font: asset_server.load("fonts/Modern_Mono.otf"),
                        font_size: 45.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/Modern_Mono.otf"),
                    font_size: 45.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            }),
        )
        .insert(TimerText);
}
