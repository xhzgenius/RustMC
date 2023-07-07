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
use super::pause; 


/**
The enum that represents the state of the main menu UI. This is a global resource.
 */
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum MainMenuUIState {
    None,
    #[default]
    Index,
    Settings,
    ChooseWorld,
}

// use super::mainmenu::*;
use super::chooseworld::*;
use super::ingame::*;
use super::setting::*;
use super::pause::*;
use super::*;


// Below are the group identifiers of the buttons, texts, etc.
/// A "tag" component for a section of main menu UI on index page.
#[derive(Component)]
pub struct MainMenuIndexUI;
/// A "tag" component for a section of main menu UI on settings page.
#[derive(Component)]
pub struct MainMenuSettingsUI;
/// A "tag" component for a section of main menu UI on choose world page.
#[derive(Component)]
pub struct MainMenuChooseWorldUI;
/// A "tag" component for a section of main menu UI on exit page.
#[derive(Component)]
pub struct MainMenuExitUI;
/// A "name" for the start button on the main menu index.
#[derive(Component)]
pub struct MainMenuIndexUIStartButton;

/// A "name" for the start button on the main menu index.
#[derive(Component)]
pub struct MainMenuIndexUIChooseWorldButton;

/// A "name" for the exit button on the main menu index.
#[derive(Component)]
pub struct MainMenuIndexUIExitButton;


// Below are the behaviors when state changes.
/**
Initialize the UI camera for main menu, and set the UI state to Index.
 */
pub fn enter_main_menu(
    mut commands: Commands,
    mut main_menu_state: ResMut<NextState<MainMenuUIState>>,
    // query_camera: Query<Entity, With<GameCamera>>,
) {
    main_menu_state.set(MainMenuUIState::Index);
    // for camera in &query_camera {
    //     commands.entity(camera).despawn_recursive();
    // }
    commands.spawn((UICamera, Camera2dBundle { ..default() }));
}
/**
Clears the UI camera for main menu, and set the UI state to None.
 */
pub fn exit_main_menu(
    mut commands: Commands,
    mut main_menu_state: ResMut<NextState<MainMenuUIState>>,
    query_camera: Query<Entity, With<UICamera>>,
) {
    main_menu_state.set(MainMenuUIState::None);
    // clear main menu camrea.
    for camera in &query_camera {
        commands.entity(camera).despawn_recursive();
    }
}


// Below are behaviors that will be taken during the state.
/**
Initialize the main menu UI on index page.
 */
pub fn init_main_menu_index(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Initialize the text.
    commands.spawn((
        MainMenuIndexUI,
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "RustMC!",
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
            MainMenuIndexUI,
            MainMenuIndexUIStartButton,
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
                "Start!",
                TextStyle {
                    font: asset_server.load("fonts/指尖隶书体.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ));
        });
    commands
        .spawn((
            MainMenuIndexUI,
            MainMenuIndexUIChooseWorldButton,
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
                "Choose World",
                TextStyle {
                    font: asset_server.load("fonts/指尖隶书体.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ));
        });
    commands
        .spawn((
            MainMenuIndexUI,
            MainMenuIndexUIExitButton,
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
                "Exit",
                TextStyle {
                    font: asset_server.load("fonts/指尖隶书体.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ));
        });
}

/**
Clears the main menu index page UI.
 */
pub fn clear_main_menu_index(mut commands: Commands, query_ui: Query<Entity, With<MainMenuIndexUI>>) {
    for ui in &query_ui {
        commands.entity(ui).despawn_recursive();
    }
}


// Below is how to react to clicks.
/// Enter the game.
pub fn main_menu_index_start_button_reaction(
    mut interaction_query: Query<&Interaction, With<MainMenuIndexUIStartButton>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::Loading);
            }
            _ => {}
        }
    }
}
// to do.