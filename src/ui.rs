use crate::*;
use bevy::prelude::*;
use std::f32::consts::PI;

/// A "tag" component for a section of UI-text.
#[derive(Component)]
struct UIText;

/// Plugin responsible for in-game UI.
/// Currently it only shows some debug information.
pub struct GameUIPlugin;
impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_game_ui_text);
        app.add_system(update_game_ui_text);
    }
}

/**
Initialize the text in the bottom-left corner of the screen.
 */
fn init_game_ui_text(mut commands: Commands, asset_server: Res<AssetServer>) {
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

/**
Update the text in the bottom-left corner of the screen.
Currently it contains some debug information.
 */
fn update_game_ui_text(
    mut query_uitext: Query<&mut Text, With<UIText>>,
    query_player: Query<
        (&entities::EntityStatusPointer, &GlobalTransform),
        With<player::MainPlayer>,
    >,
    query_camera: Query<(&Transform, &GlobalTransform), With<render::GameCamera>>,
) {
    let (status_pointer, global_transform) =
        &query_player.get_single().expect("Not exactly one player!");
    let player_status = status_pointer.pointer.lock().unwrap();
    let (camera_transform, camera_global_transform) =
        &query_camera.get_single().expect("Not exactly one camera!");
    for mut text in &mut query_uitext {
        text.sections[0].value = format!(
            "Player position: {}
Player rotation (around Y-axis): {:.4} degrees
Player velocity: {}
Camera position: {}
Camera rotation (vertical, around X-axis): {:.4} degrees",
            player_status.position,
            player_status.rotation * 180. / PI,
            player_status.velocity,
            camera_global_transform.translation(),
            camera_transform.rotation.to_euler(EulerRot::XYZ).0 * 180. / PI
        );
    }
}
