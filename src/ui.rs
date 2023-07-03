use crate::*;
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
struct UIText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_ui_text);
        app.add_system(update_ui_text);
    }
}

fn init_ui_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                font: asset_server.load("fonts/指尖隶书体.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Left)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        UIText,
    ));
}

fn update_ui_text(
    mut query_uitext: Query<&mut Text, With<UIText>>,
    query_player: Query<
        (&entities::EntityStatusPointer, &GlobalTransform),
        With<player::MainPlayer>,
    >,
    query_camera: Query<&GlobalTransform, With<render::GameCamera>>,
) {
    let (status_pointer, global_transform) =
        &query_player.get_single().expect("Not exactly one player!");
    let player_status = status_pointer.pointer.lock().unwrap();
    let camera_transform = &query_camera.get_single().expect("Not exactly one camera!");
    for mut text in &mut query_uitext {
        text.sections[0].value = format!(
            "Player position: {}\nPlayer rotation: {}\nPlayer velocity: {}\nCamera transform: {}",
            player_status.position,
            player_status.rotation / PI,
            player_status.velocity,
            camera_transform.translation()
        );
    }
}
