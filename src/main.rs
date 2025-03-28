use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((Camera2d, BoxShadowSamples(4)));

    let baloo: Handle<Font> = assets.load("BalooBhaijaan2-Bold.ttf");
    let fira: Handle<Font> = assets.load("FiraCode-Medium.ttf");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Node {
                    width: Val::Percent(100.),
                    flex_grow: 1.,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor::from(Srgba::hex("0088ff").unwrap()),
                children![
                    (
                        Text("TEXT SHADOW".to_string()),
                        TextShadow {
                            color: Srgba::hex("2b2c2f").unwrap().into(),
                            offset: Vec2::new(6.0, 0.0),
                        },
                        TextColor(Srgba::hex("ffc14d").unwrap().into()),
                        TextFont {
                            font: baloo.clone(),
                            font_size: 98.0,
                            ..default()
                        },
                    ),
                    (
                        Text("x offset →".to_string()),
                        TextColor(Srgba::hex("2b2c2f").unwrap().into()),
                        TextFont {
                            font: fira.clone(),
                            ..default()
                        },
                    )
                ]
            ),
            (
                Node {
                    width: Val::Percent(100.),
                    flex_grow: 1.,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                children![
                    (
                        Text("TEXT SHADOW".to_string()),
                        TextShadow {
                            color: Srgba::hex("ff6666").unwrap().into(),
                            offset: Vec2::new(0.0, 6.0),
                        },
                        TextFont {
                            font: baloo.clone(),
                            font_size: 98.0,
                            ..default()
                        },
                    ),
                    (
                        Text("y offset ↓".to_string()),
                        TextFont {
                            font: fira.clone(),
                            ..default()
                        },
                    )
                ]
            ),
            (
                Node {
                    width: Val::Percent(100.),
                    flex_grow: 1.,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor::from(Srgba::hex("ececec").unwrap()),
                children![
                    (
                        Text("TEXT SHADOW".to_string()),
                        TextShadow {
                            color: Srgba::hex("b2b2b2").unwrap().into(),
                            offset: Vec2::new(4.0, 4.0),
                        },
                        TextColor(Srgba::hex("232326").unwrap().into()),
                        TextFont {
                            font: baloo.clone(),
                            font_size: 98.0,
                            ..default()
                        },
                    ),
                    (
                        Text("x & y offset ↘".to_string()),
                        TextColor(Srgba::hex("232326").unwrap().into()),
                        TextFont {
                            font: fira.clone(),
                            ..default()
                        },
                    )
                ]
            ),
        ],
    ));
}
