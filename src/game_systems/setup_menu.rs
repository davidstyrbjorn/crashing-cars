use crate::prelude::*;

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Modern_Mono.otf");
    let text_alignment = TextAlignment::CENTER;
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    let font_sub = asset_server.load("fonts/Modern_Mono.otf");
    let text_style_sub = TextStyle {
        font: font_sub,
        font_size: 45.0,
        color: Color::WHITE,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Crash Cars", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        },
        OnMainMenu,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Press any key to start", text_style_sub.clone())
                .with_alignment(text_alignment),
            transform: Transform::from_translation(Vec3::new(0.0, -100.0, 0.0)),
            ..default()
        },
        OnMainMenu,
    ));

    commands.spawn((
        InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(VirtualDPad::arrow_keys(), Action::StartGame)
                .insert(VirtualDPad::dpad(), Action::StartGame)
                .insert(VirtualDPad::gamepad_face_buttons(), Action::StartGame)
                .build(),
        },
        OnMainMenu,
    ));
}
