//! UI logic
//! ---
//! The game itself has states: `MainMenu`, `InGame`, `Pause`, ...
//!
//! At MainMenu state, there is main menu UI.
//! The main menu UI has states too: `Index`, `ChooseWorld`, `Settings`,
//! and a `None` state representing that the game state is not at `MainMenu`.
//!
//! At InGame state, there is in-game UI.
//! It does not have states. Instead, all components are controlled by its own bool value: show or not.
//!
//! At Pause state, there is pause UI.
//! It has states. To be determined...
//! TODO: Implement the pause UI.

use crate::*;
use bevy::prelude::*;
use std::f32::consts::PI;

/// Plugin responsible for in-game UI.
/// Currently it only shows some debug information.
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuUIState>();
        // Enter or exit the whole main menu.
        app.add_system(enter_main_menu.in_schedule(OnEnter(GameState::MainMenu)));
        app.add_system(exit_main_menu.in_schedule(OnExit(GameState::MainMenu)));
        // Show or clear each state of main menu.
        app.add_system(init_main_menu_index.in_schedule(OnEnter(MainMenuUIState::Index)));
        app.add_system(clear_main_menu_index.in_schedule(OnExit(MainMenuUIState::Index)));

        app.add_state::<InGameUIState>();
        app.add_system(init_in_game_ui_text.in_schedule(OnEnter(GameState::InGame)));
        app.add_system(update_in_game_ui_text.in_set(OnUpdate(GameState::InGame)));
    }
}

/**
The enum that represents the state of the main menu UI. This is a global resource.
 */
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
enum MainMenuUIState {
    None,
    #[default]
    Index,
    Settings,
    ChooseWorld,
}

/**
The enum that represents the state of the in-game UI. This is a global resource.
 */
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
enum InGameUIState {
    #[default]
    None,
    Pause,
}

// These are the group identifiers of the buttons, texts, etc.
/// A "tag" component for the UI camera.
#[derive(Component)]
struct UICamera;
/// A "tag" component for a section of main menu UI on index page.
#[derive(Component)]
struct MainMenuIndexUI;
/// A "tag" component for a section of main menu UI on settings page.
#[derive(Component)]
struct MainMenuSettingsUI;
/// A "tag" component for a section of main menu UI on choose world page.
#[derive(Component)]
struct MainMenuChooseWorldUI;

// Below are the names of individual buttons, texts, etc.
/// A "name" for the start button on the main menu index.
#[derive(Component)]
struct MainMenuIndexUIStartButton;
/// A "name" for the bottom-left text area in-game UI.
#[derive(Component)]
struct InGameUIBottomLeftText;

/**
Initialize the UI camera for main menu, and set the UI state to Index.
 */
fn enter_main_menu(
    mut commands: Commands,
    mut main_menu_state: ResMut<NextState<MainMenuUIState>>,
) {
    main_menu_state.set(MainMenuUIState::Index);
    commands.spawn((UICamera, Camera2dBundle { ..default() }));
}
/**
Clears the UI camera for main menu, and set the UI state to None.
 */
fn exit_main_menu(
    mut commands: Commands,
    mut main_menu_state: ResMut<NextState<MainMenuUIState>>,
    query_camera: Query<Entity, With<UICamera>>,
) {
    main_menu_state.set(MainMenuUIState::None);
    for camera in &query_camera {
        commands.entity(camera).despawn_recursive();
    }
}

/**
Initialize the main menu UI on index page.
 */
fn init_main_menu_index(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}

/**
Clears the main menu index page UI.
 */
fn clear_main_menu_index(mut commands: Commands, query_ui: Query<Entity, With<MainMenuIndexUI>>) {
    for ui in &query_ui {
        commands.entity(ui).despawn_recursive();
    }
}

/**
Initialize the in-game UI text at the bottom-left corner of the screen.
 */
fn init_in_game_ui_text(mut commands: Commands, asset_server: Res<AssetServer>) {
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
fn update_in_game_ui_text(
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
