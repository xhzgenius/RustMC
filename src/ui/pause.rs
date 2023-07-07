use crate::{init_game::GameCamera, *};
use bevy::{prelude::*, ecs::world};
use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
};

use super::mainmenu;
use super::chooseworld;
use super::ingame;
use super::setting;
// use super::pause; 

/**
The enum that represents the state of the pause UI. This is a global resource.
 */
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub(crate) enum PauseUIState {
    #[default]
    None,
    Pause,
}


// Below are the group identifiers of the buttons, texts, etc.
/// A "tag" component for a section of pause UI on index page.
#[derive(Component)]
pub(crate) struct PauseIndexUI;
/// A "name" for the return game button on the pause index.
#[derive(Component)]
pub(crate) struct PauseIndexUIReturnButton;
/// A "name" for the main menu button on the pause index.
#[derive(Component)]
pub(crate) struct PauseIndexUIMainmenuButton;
/// A "name" for the exit button on the pause index.
#[derive(Component)]
pub(crate) struct PauseIndexUIExitButton;


// Below are the behaviors when state changes.
/**
    Initialize the UI camera for pause state
*/
pub(crate) fn enter_pause(mut commands: Commands, mut pause_state: ResMut<NextState<PauseUIState>>) {
    pause_state.set(PauseUIState::Pause);
    //commands.spawn((UICamera1, Camera2dBundle { ..default() }));
}

/**
   Clears the UI camera for pause state, and set the UI state to None.
*/
pub(crate) fn exit_pause(
    mut commands: Commands,
    mut pause_state: ResMut<NextState<PauseUIState>>,
    query_camera: Query<Entity, With<GameCamera>>,
) {
    pause_state.set(PauseUIState::None);
}


// Below are behaviors that will be taken during the state.
/**
Initialize the pause UI on index page.
 */
pub(crate) fn init_pause_index(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Initialize the text.
    commands.spawn((
        PauseIndexUI,
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Pause",
            TextStyle {
                font: asset_server.load("fonts/指尖隶书体.ttf"),
                font_size: 200.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Percent(30.),
                top: Val::Percent(30.),
                ..default()
            },
            size: Size::new(Val::Percent(40.), Val::Percent(20.)),
            ..default()
        }),
    ));
    // Initialize the buttons with children texts.
    commands
        .spawn((
            PauseIndexUI,
            PauseIndexUIReturnButton,
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Percent(30.),
                        right: Val::Percent(50.),
                        ..default()
                    },
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Return",
                TextStyle {
                    font: asset_server.load("fonts/指尖隶书体.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ));
        });
    commands
        .spawn((
            PauseIndexUI,
            PauseIndexUIMainmenuButton,
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Percent(20.),
                        right: Val::Percent(50.),
                        ..default()
                    },
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                " Menu ",
                TextStyle {
                    font: asset_server.load("fonts/指尖隶书体.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ));
        });
    commands
        .spawn((
            PauseIndexUI,
            PauseIndexUIExitButton,
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Percent(10.),
                        right: Val::Percent(50.),
                        ..default()
                    },
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                " Exit ",
                TextStyle {
                    font: asset_server.load("fonts/指尖隶书体.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ));
        });
}

/**
Clears the pause index page UI.
 */
pub(crate) fn clear_pause_index(mut commands: Commands, query_ui: Query<Entity, With<PauseIndexUI>>) {
    for ui in &query_ui {
        commands.entity(ui).despawn_recursive();
    }
}


// Below is how to react to clicks.
/**
   Reaction for Return button.
*/
pub(crate) fn pause_index_return_button_reaction(
    mut interaction_query: Query<&Interaction, With<PauseIndexUIReturnButton>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::InGame);
            }
            _ => {}
        }
    }
}
/**
   Reaction for Mainmenu button.
*/
pub(crate) fn pause_index_main_menu_button_reaction(
    mut interaction_query: Query<&Interaction, With<PauseIndexUIMainmenuButton>>,
    mut game_state: ResMut<NextState<GameState>>,
    gamemap: Res<gamemap::GameMap>,
    mut commands: Commands,
    world_name: Res<gamemap::WorldName>,
    query_game_entities: Query<Entity, With<entities::Entity>>,
    query_game_blocks: Query<Entity, With<blocks::Block>>,
    query_game_camera: Query<Entity, With<init_game::GameCamera>>,
    query_game_lights: Query<Entity, With<DirectionalLight>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match gamemap::save_gamemap(&gamemap, &world_name) {
                    Ok(_) => {
                        println!("Saved world to {}", &world_name.name.clone().unwrap());
                        // Despawn everything in the game.
                        for entity in &query_game_entities {
                            commands.entity(entity).despawn_recursive();
                        }
                        for entity in &query_game_blocks {
                            commands.entity(entity).despawn_recursive();
                        }
                        // for entity in &query_game_camera {
                        //     commands.entity(entity).despawn_recursive();
                        // } // No need. Seems the camera is already despawned when exiting InGame state?
                        for entity in &query_game_lights {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                    e => panic!("Save gamemap failed: {:?}", e),
                }
                game_state.set(GameState::MainMenu);
            }
            _ => {}
        }
    }
}
/**
   Reaction for Exit button.
*/
pub(crate) fn pause_index_exit_button_reaction(
    mut interaction_query: Query<&Interaction, With<PauseIndexUIExitButton>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
}
