use crate::{init_game::GameCamera, *, gamemap::find_gamemap};
use bevy::{prelude::*, ecs::world};
use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
};

use super::mainmenu::*;
// use super::chooseworld::*;
use super::ingame::*;
use super::setting::*;
use super::pause::*;
use super::*;

// Below are the group identifiers of the buttons, texts, etc.
/// A "name" for the wolrd button on the main menu choose world state.
#[derive(Component)]
pub struct MainMenuChooseWorldUIWorldButton1;
#[derive(Component)]
pub struct MainMenuChooseWorldUIWorldButton2;
#[derive(Component)]
pub struct MainMenuChooseWorldUIWorldButton3;

// Below are the behaviors when state changes.

// Below are behaviors that will be taken during the state.
/**
Initialize the main menu UI on choose world page.
 */
pub(crate) fn init_main_menu_choose_world(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    // Initialize the text.
    commands.spawn((
        MainMenuChooseWorldUI,
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Choose World",
            TextStyle {
                font: asset_server.load("fonts/指尖隶书体.ttf"),
                font_size: 100.0,
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
    let mut name = match find_gamemap("World1") {
        true => "World1",
        false => "New world"
    };

    commands
        .spawn((
            MainMenuChooseWorldUI,
            MainMenuChooseWorldUIWorldButton1,
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
                name,
                TextStyle {
                    font: asset_server.load("fonts/指尖隶书体.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ));
        });
        let mut name = match find_gamemap("World2") {
            true => "World2",
            false => "New world"
        };
        commands
        .spawn((
            MainMenuChooseWorldUI,
            MainMenuChooseWorldUIWorldButton2,
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
                name,
                TextStyle {
                    font: asset_server.load("fonts/指尖隶书体.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ));
        });
        let mut name = match find_gamemap("World3") {
            true => "World3",
            false => "New world"
        };
        commands
        .spawn((
            MainMenuChooseWorldUI,
            MainMenuChooseWorldUIWorldButton3,
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
                name,
                TextStyle {
                    font: asset_server.load("fonts/指尖隶书体.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ));
        });    
    
        
}

/**
Clears the main menu choose world page UI.
 */
pub(crate) fn clear_main_menu_choose_world(
    mut commands: Commands,
    query_ui: Query<Entity, With<MainMenuChooseWorldUI>>,
) {
    for ui in &query_ui {
        commands.entity(ui).despawn_recursive();
    }
}


// Below is how to react to clicks.
/// Enter the game.
pub fn main_menu_index_choose_world_button1_reaction(
    mut interaction_query: Query<&Interaction, With<MainMenuChooseWorldUIWorldButton1>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut world_name: ResMut<gamemap::WorldName>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *world_name = gamemap::WorldName {
                    name: Some("World1".to_string()),
                };
                game_state.set(GameState::Loading);
            }
            _ => {}
        }
    }
}
/// Enter the game.
pub fn main_menu_index_choose_world_button2_reaction(
    mut interaction_query: Query<&Interaction, With<MainMenuChooseWorldUIWorldButton2>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut world_name: ResMut<gamemap::WorldName>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *world_name = gamemap::WorldName {
                    name: Some("World2".to_string()),
                };
                game_state.set(GameState::Loading);
            }
            _ => {}
        }
    }
}
/// Enter the game.
pub fn main_menu_index_choose_world_button3_reaction(
    mut interaction_query: Query<&Interaction, With<MainMenuChooseWorldUIWorldButton3>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut world_name: ResMut<gamemap::WorldName>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *world_name = gamemap::WorldName {
                    name: Some("World3".to_string()),
                };
                game_state.set(GameState::Loading);
            }
            _ => {}
        }
    }
}