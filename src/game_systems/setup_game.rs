use crate::prelude::*;

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_player(
        &mut commands,
        (KeyCode::A, KeyCode::D),
        (KeyCode::W, KeyCode::S),
        KeyCode::Space,
        Vec3::new(WINDOW_WIDTH / 4.0, 0.0, 0.0),
        Team::Blue,
    );
    spawn_player(
        &mut commands,
        (KeyCode::Left, KeyCode::Right),
        (KeyCode::Up, KeyCode::Down),
        KeyCode::Space,
        Vec3::new(-WINDOW_WIDTH / 4.0, 0.0, 0.0),
        Team::Red,
    );

    spawn_ball(&mut commands, &mut meshes, &mut materials);
    spawn_level_box(&mut commands);

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

    // Spawn round timer text
    commands
        .spawn_bundle(
            TextBundle::from_sections([TextSection::new(
                "TIME: ",
                TextStyle {
                    font: asset_server.load("fonts/Modern_Mono.otf"),
                    font_size: 45.0,
                    color: Color::WHITE,
                },
            )])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(0.0),
                    right: Val::Percent(47.5),
                    ..default()
                },
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..default()
                },
                ..default()
            }),
        )
        .insert(ScoreText);
}
