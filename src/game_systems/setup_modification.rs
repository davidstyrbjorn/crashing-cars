use crate::prelude::*;

pub fn create_modification_element(parent: &mut ChildBuilder) -> Entity {
    parent
        .spawn((NodeBundle {
            background_color: Color::rgb(0.95, 0.95, 0.95).into(),
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Px(300.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },))
        .id()
}

pub fn setup_modification(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut draft_resource: ResMut<DraftResource>,
    mut modifications: ResMut<Modifications>,
) {
    // We'd want to save these modifications into a resource
    draft_resource.modifications = vec![
        modifications.get_modification(),
        modifications.get_modification(),
        modifications.get_modification(),
    ];

    let font = asset_server.load("fonts/Modern_Mono.otf");
    let font_card = asset_server.load("fonts/Modern_Mono.otf");
    let font_card_description = asset_server.load("fonts/Modern_Mono.otf");

    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let card_text_style = TextStyle {
        font: font_card,
        font_size: 50.0,
        color: Color::BLACK,
    };
    let card_text_description_style = TextStyle {
        font: font_card_description,
        font_size: 25.0,
        color: Color::BLACK,
    };

    let mut card_entities = Vec::new();
    let start_offset = -150.0;

    commands
        .spawn((
            NodeBundle {
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
                        top: Val::Percent(10.0 - start_offset),
                        ..Default::default()
                    },
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            SlidingWindow(start_offset),
            OnModification,
        ))
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
                    card_entities.push(create_modification_element(parent));
                    card_entities.push(create_modification_element(parent));
                    card_entities.push(create_modification_element(parent));
                });
        });

    card_entities.iter().enumerate().for_each(|(idx, entity)| {
        let title = draft_resource.modifications[idx].title.clone();
        let description = draft_resource.modifications[idx].description.clone();

        commands
            .entity(*entity)
            .insert(ModificationElement { order: idx });

        commands.entity(*entity).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(title, card_text_style.clone()).with_style(Style {
                    margin: UiRect {
                        top: Val::Px(20.0),
                        ..Default::default()
                    },
                    justify_content: JustifyContent::Center,
                    max_size: Size {
                        width: Val::Px(300.0),
                        height: Val::Px(300.0),
                    },
                    ..default()
                }),
            );
            parent.spawn(
                TextBundle::from_section(description, card_text_description_style.clone())
                    .with_style(Style {
                        margin: UiRect {
                            top: Val::Px(20.0),
                            ..Default::default()
                        },
                        justify_content: JustifyContent::Center,
                        max_size: Size {
                            width: Val::Px(250.0),
                            height: Val::Px(300.0),
                        },
                        ..default()
                    }),
            );
        });
    });
}
