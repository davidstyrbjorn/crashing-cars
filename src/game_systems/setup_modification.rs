use crate::prelude::*;

pub fn setup_modification(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Modern_Mono.otf");
    let text_alignment = TextAlignment::CENTER;

    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands
        .spawn(NodeBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                display: Display::Flex,
                align_items: AlignItems::FlexStart,
                position: UiRect {
                    left: Val::Percent(10.0),
                    top: Val::Percent(10.0),
                    ..Default::default()
                },
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                Text2dBundle {
                    text: Text::from_section("Modification", text_style.clone())
                        .with_alignment(text_alignment),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),

                    ..default()
                }, // TextBundle::from_section("Modification", text_style.clone()).with_style(Style {
                   //     justify_content: JustifyContent::Center,
                   //     ..default()
                   // }),
            );
        });
}
