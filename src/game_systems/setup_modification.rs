use crate::prelude::*;

pub fn create_modification_box(parent: &mut ChildBuilder) {
    let font = asset_server.load("fonts/Modern_Mono.otf");

    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    parent
        .spawn(NodeBundle {
            background_color: Color::rgb(0.95, 0.95, 0.95).into(),
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Px(300.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section("Modification", text_style.clone()).with_style(Style {
                    margin: UiRect {
                        top: Val::Px(50.0),
                        ..Default::default()
                    },
                    justify_content: JustifyContent::Center,
                    ..default()
                }),
            );
        });
}

pub fn setup_modification(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Modern_Mono.otf");

    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                justify_content: JustifyContent::FlexStart,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                display: Display::Flex,
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..Default::default()
                },
                position: UiRect {
                    left: Val::Percent(10.0),
                    top: Val::Percent(10.0),
                    ..Default::default()
                },
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section("Modification", text_style.clone()).with_style(Style {
                    margin: UiRect {
                        top: Val::Px(50.0),
                        ..Default::default()
                    },
                    justify_content: JustifyContent::Center,
                    ..default()
                }),
            );
            parent
                .spawn(NodeBundle {
                    background_color: Color::rgba(1.0, 0.0, 0.0, 0.0).into(),
                    style: Style {
                        margin: UiRect {
                            top: Val::Px(60.0),
                            ..default()
                        },
                        display: Display::Flex,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Percent(80.0), Val::Percent(70.0)),
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    create_modification_box(parent);
                    create_modification_box(parent);
                    create_modification_box(parent);
                });
        });
}
