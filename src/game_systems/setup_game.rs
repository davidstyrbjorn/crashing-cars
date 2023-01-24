use crate::prelude::*;

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut startup_flags: ResMut<StartupFlags>,
    mut players: ResMut<Players>,
) {
    if startup_flags.in_game {
        return;
    }
    startup_flags.in_game = true;

    let animal_textures = vec![
        "chick.png",
        "duck.png",
        "frog.png",
        "owl.png",
        "parrot.png",
        "sloth.png",
    ];

    let e1 = spawn_player(
        &mut commands,
        &asset_server,
        (KeyCode::A, KeyCode::D),
        (KeyCode::W, KeyCode::S),
        KeyCode::Space,
        KeyCode::E,
        Vec3::new(-WINDOW_WIDTH / 4.0, 0.0, 0.0),
        Team::Red,
        animal_textures[0],
    );
    let e2 = spawn_player(
        &mut commands,
        &asset_server,
        (KeyCode::Left, KeyCode::Right),
        (KeyCode::Up, KeyCode::Down),
        KeyCode::Space,
        KeyCode::RShift,
        Vec3::new(WINDOW_WIDTH / 4.0, 0.0, 0.0),
        Team::Blue,
        animal_textures[1],
    );

    // Fill our lil array
    players.0 = vec![e1, e2];

    spawn_ball(&mut commands, &asset_server);
    spawn_level_box(&mut commands);

    // spawn_hazard(&mut commands);
    //spawn_modify_field(&mut commands, 0);
    // spawn_boost_pickups(&mut commands, &asset_server);

    // Spawn round timer text
    commands
        .spawn(
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

    // Spawn score text
    commands
        .spawn(
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

    // Spawn prepare round text
    commands.spawn((
        Text2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 150.0, 0.0)),
            text: Text::from_section(
                "0",
                TextStyle {
                    color: Color::WHITE,
                    font: asset_server.load("fonts/Modern_Mono.otf"),
                    font_size: 90.0,
                },
            )
            .with_alignment(TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Center,
            }),
            ..default()
        },
        PrepareTimerText,
    ));
}
