use crate::{init_game::GameCamera, *};
use bevy::{prelude::*, ecs::world};
use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
};

use super::mainmenu::*;
use super::chooseworld::*;
// use super::ingame::*;
use super::setting::*;
use super::pause::*;
use super::*;

/**
The enum that represents the state of the in-game UI. This is a global resource.
 */
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum InGameUIState {
    #[default]
    None,
    Pause,
}


// Below are the group identifiers of the buttons, texts, etc.
/// A "name" for the bottom-left text area in-game UI.
#[derive(Component)]
pub struct InGameUIBottomLeftText;

/// A "name" for the center cursor in-game UI.
#[derive(Component)]
pub struct InGameUICenterCursor;


// Below are the behaviors when state changes.

// Below are behaviors that will be taken during the state.
/**
Initialize the in-game UI text at the bottom-left corner of the screen.
 */
pub fn init_in_game_ui_text(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        InGameUIBottomLeftText,
    ));
}

/**
Update the text at the bottom-left corner of the screen.
Currently it contains some debug information.
 */
pub fn update_in_game_ui_text(
    mut query_uitext: Query<&mut Text, With<InGameUIBottomLeftText>>,
    query_player: Query<
        (&entities::EntityStatusPointer, &GlobalTransform),
        With<player::MainPlayer>,
    >,
    query_camera: Query<(&Transform, &GlobalTransform), With<init_game::GameCamera>>,
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

/**
Add cursor in the center of the screen.
 */
pub fn update_in_game_ui_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut cursor: Query<&mut Picture, With<InGameUICenterCursor>>,
    query_player: Query<
        (&entities::EntityStatusPointer, &GlobalTransform),
        With<player::MainPlayer>,
    >,
    query_camera: Query<(&Transform, &GlobalTransform), With<init_game::GameCamera>>,
) {
    commands.spawn((
        MainMenuIndexUI,
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "+",
            TextStyle {
                font: asset_server.load("fonts/msyh.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Percent(50.),
                top: Val::Percent(50.),
                ..default()
            },
            size: Size::new(Val::Percent(40.), Val::Percent(20.)),
            ..default()
        }),
    ));
}


// Below is how to react to clicks.
/// From in_game state to pause state
pub fn in_game_pause_reaction(key: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if key.pressed(KeyCode::Escape) {
        game_state.set(GameState::Pause);
    }
}