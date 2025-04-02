use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct RootNode;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((Camera2d, BoxShadowSamples(4)));

    scene2(&mut commands, &assets);
}

fn scene2(commands: &mut Commands, assets: &AssetServer) {
    let baloo: Handle<Font> = assets.load("BalooBhaijaan2-Bold.ttf");
    let amarante: Handle<Font> = assets.load("Amarante-Regular.ttf");
    let fjalla: Handle<Font> = assets.load("FjallaOne-Regular.ttf");
    let poppins: Handle<Font> = assets.load("Poppins-Bold.ttf");

    let yellow: Color = Srgba::hex("FFC14D").unwrap().into();
    let blue: Color = Srgba::hex("4D84FF").unwrap().into();
    let red: Color = Srgba::hex("FF6666").unwrap().into();
    let light: Color = Srgba::hex("ECECEC").unwrap().into();
    let dark: Color = Srgba::hex("232326").unwrap().into();

    commands.spawn((
        RootNode,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Grid,
            grid_template_columns: vec![RepeatedGridTrack::fr(2, 1.0)],
            ..default()
        },
        children![
            (
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor::from(blue),
                children![(
                    Text("TEXT SHADOWS".to_string()),
                    TextShadow {
                        color: dark,
                        offset: Vec2::new(6.0, 0.0),
                    },
                    TextColor(yellow),
                    TextFont {
                        font: fjalla.clone(),
                        font_size: 88.0,
                        ..default()
                    },
                ),]
            ),
            (
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor::from(light),
                children![(
                    Text("TEXT SHADOWS".to_string()),
                    TextShadow {
                        color: dark.with_alpha(0.5),
                        offset: Vec2::new(4.0, 4.0),
                    },
                    TextColor(dark),
                    TextFont {
                        font: amarante.clone(),
                        font_size: 72.0,
                        ..default()
                    },
                ),]
            ),
            (
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor::from(dark),
                children![(
                    Text("TEXT SHADOWS".to_string()),
                    TextShadow {
                        color: red,
                        offset: Vec2::new(-4.0, -2.0),
                    },
                    TextColor(light),
                    TextFont {
                        font: baloo.clone(),
                        font_size: 72.0,
                        ..default()
                    },
                ),]
            ),
            (
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor::from(red),
                children![(
                    Text("TEXT SHADOWS".to_string()),
                    TextShadow {
                        color: dark,
                        offset: Vec2::new(8.0, 6.0),
                    },
                    TextColor(light),
                    TextFont {
                        font: poppins.clone(),
                        font_size: 64.0,
                        ..default()
                    },
                ),]
            )
        ],
    ));
}
