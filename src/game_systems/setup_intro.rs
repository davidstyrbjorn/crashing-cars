use crate::prelude::*;

pub fn setup_intro(mut commands: Commands, asset_server: Res<AssetServer>) {
    vec![
        "On average from 2010 over 10 thousand people\ndie every year from drunk driving accidents.",
        "The worst part is that the victims also include\npeople minding their own business on the street\nthat is objectively bad.",
        "You crazy fucks are having beers and such then\nyou put yourself in control of a metallic elephant\nwith turbo engines. Jesus christ.",
        "For the love of god\nplay this game instead to simulate the adrenaline flow of driving a blazer.",
        "I don't really mind saving lives you know\n fucking hell it might even be my destiny, to save lives.",
        "Anyway here's the game..."
    ].iter().enumerate().for_each(|(idx, part)| {
        let part = *part;
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(part, 
                    TextStyle {
                        font: asset_server.load("fonts/Modern_Mono.otf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ).with_alignment(TextAlignment { 
                    vertical: VerticalAlign::Center, 
                    horizontal: HorizontalAlign::Center, 
                }),
                ..Default::default()
            },
            IntroText {
                order: idx,
                time: 0
            })
        );
    });
}
