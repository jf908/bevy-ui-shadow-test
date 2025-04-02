//! This example shows how to create a node with a shadow

use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use bevy::text::{FontFamily, FontStyle};
use bevy::winit::WinitSettings;

fn shadow() -> BoxShadow {
    BoxShadow(vec![
        ShadowStyle {
            color: Srgba::BLACK.with_alpha(0.22).into(),
            x_offset: Val::Px(0.),
            y_offset: Val::Px(1.),
            blur_radius: Val::Px(3.),
            spread_radius: Val::Px(0.),
        },
        ShadowStyle {
            color: Srgba::BLACK.with_alpha(0.22).into(),
            x_offset: Val::Px(0.),
            y_offset: Val::Px(1.),
            blur_radius: Val::Px(2.),
            spread_radius: Val::Px(-1.),
        },
    ])
}

fn shadow_md() -> BoxShadow {
    BoxShadow(vec![
        ShadowStyle {
            color: Srgba::BLACK.with_alpha(0.22).into(),
            x_offset: Val::Px(0.),
            y_offset: Val::Px(4.),
            blur_radius: Val::Px(6.),
            spread_radius: Val::Px(-1.),
        },
        ShadowStyle {
            color: Srgba::BLACK.with_alpha(0.22).into(),
            x_offset: Val::Px(0.),
            y_offset: Val::Px(2.),
            blur_radius: Val::Px(4.),
            spread_radius: Val::Px(-2.),
        },
    ])
}

fn shadow_lg() -> BoxShadow {
    BoxShadow(vec![
        ShadowStyle {
            color: Srgba::BLACK.with_alpha(0.22).into(),
            x_offset: Val::Px(0.),
            y_offset: Val::Px(10.),
            blur_radius: Val::Px(15.),
            spread_radius: Val::Px(-3.),
        },
        ShadowStyle {
            color: Srgba::BLACK.with_alpha(0.22).into(),
            x_offset: Val::Px(0.),
            y_offset: Val::Px(4.),
            blur_radius: Val::Px(6.),
            spread_radius: Val::Px(-4.),
        },
    ])
}

fn shadow_xl() -> BoxShadow {
    BoxShadow(vec![
        ShadowStyle {
            color: Srgba::BLACK.with_alpha(0.22).into(),
            x_offset: Val::Px(0.),
            y_offset: Val::Px(20.),
            blur_radius: Val::Px(25.),
            spread_radius: Val::Px(-5.),
        },
        ShadowStyle {
            color: Srgba::BLACK.with_alpha(0.22).into(),
            x_offset: Val::Px(0.),
            y_offset: Val::Px(8.),
            blur_radius: Val::Px(10.),
            spread_radius: Val::Px(-6.),
        },
    ])
}

/// `box_shadow` example
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .run();
}

// fn generate_text(
//     text: &str,
//     palette: Vec<Color>,
//     font: TextFont,
// ) -> Vec<(Text, TextShadow, TextFont)> {
//     // Children::spawn(generate_text(
//     //     "TEXT SHADOWS",
//     //     vec![
//     //         Srgba::hex("4a6e91").unwrap().into(),
//     //         Srgba::hex("ffc14d").unwrap().into(),
//     //         Srgba::hex("7db56e").unwrap().into(),
//     //     ],
//     //     TextFont {
//     //         font: font.clone(),
//     //         font_size: 140.0,
//     //         ..default()
//     //     },
//     // )),

//     text.chars()
//         .enumerate()
//         .map(|(i, char)| {
//             (
//                 Text(char.to_string()),
//                 TextShadow {
//                     color: palette[i % palette.len()],
//                     offset: Vec2::new(6.0, 10.0),
//                 },
//                 font.clone(),
//             )
//         })
//         .collect()
// }

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // ui camera
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

    // commands
    //     .spawn((
    //         Node {
    //             width: Val::Percent(100.0),
    //             height: Val::Percent(100.0),
    //             padding: UiRect::all(Val::Px(30.)),
    //             column_gap: Val::Px(30.),
    //             flex_wrap: FlexWrap::Wrap,
    //             ..default()
    //         },
    //         BackgroundColor(GRAY_300.into()),
    //     ))
    //     .with_children(|commands| {
    //         commands.spawn((
    //             Node {
    //                 width: Val::Px(80.),
    //                 height: Val::Px(80.),
    //                 ..default()
    //             },
    //             BorderRadius::all(Val::Px(4.)),
    //             BackgroundColor(GRAY_100.into()),
    //             shadow(),
    //         ));

    //         commands.spawn((
    //             Node {
    //                 width: Val::Px(80.),
    //                 height: Val::Px(80.),
    //                 ..default()
    //             },
    //             BorderRadius::all(Val::Px(4.)),
    //             BackgroundColor(GRAY_100.into()),
    //             shadow_md(),
    //         ));

    //         commands.spawn((
    //             Node {
    //                 width: Val::Px(80.),
    //                 height: Val::Px(80.),
    //                 ..default()
    //             },
    //             BorderRadius::all(Val::Px(4.)),
    //             BackgroundColor(GRAY_100.into()),
    //             shadow_lg(),
    //         ));

    //         commands.spawn((
    //             Node {
    //                 width: Val::Px(80.),
    //                 height: Val::Px(80.),
    //                 ..default()
    //             },
    //             BorderRadius::all(Val::Px(4.)),
    //             BackgroundColor(GRAY_100.into()),
    //             shadow_xl(),
    //         ));

    //         commands.spawn((
    //             Node {
    //                 width: Val::Px(80.),
    //                 height: Val::Px(80.),
    //                 ..default()
    //             },
    //             BorderRadius::all(Val::Px(4.)),
    //             BackgroundColor(GRAY_100.into()),
    //             BoxShadow(vec![
    //                 ShadowStyle {
    //                     color: Srgba::BLUE.into(),
    //                     x_offset: Val::Px(12.),
    //                     y_offset: Val::Px(12.),
    //                     blur_radius: Val::Px(1.),
    //                     spread_radius: Val::Px(0.),
    //                 },
    //                 ShadowStyle {
    //                     color: Srgba::GREEN.into(),
    //                     x_offset: Val::Px(8.),
    //                     y_offset: Val::Px(8.),
    //                     blur_radius: Val::Px(1.),
    //                     spread_radius: Val::Px(0.),
    //                 },
    //                 ShadowStyle {
    //                     color: Srgba::RED.into(),
    //                     x_offset: Val::Px(4.),
    //                     y_offset: Val::Px(4.),
    //                     blur_radius: Val::Px(1.),
    //                     spread_radius: Val::Px(0.),
    //                 },
    //             ]),
    //         ));
    //     });
}
