//! UI logic
//! ---
//! The game itself has states: `MainMenu`, `InGame`, `Pause`, ...
//!
//! At `MainMenu` state, there is main menu UI.
//! The main menu UI has states too: `Index`, `ChooseWorld`, `Settings`,
//! and a `None` state representing that the game state is not at `MainMenu`.
//!
//! At `InGame` state, there is in-game UI.
//! It does not have states. Instead, all components are controlled by its own bool value: show or not.
//!
//! At `Pause` state, there is pause UI.
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
        // Main menu UI.
        app.add_state::<MainMenuUIState>();
        // Enter or exit the whole main menu.
        app.add_systems((
            enter_main_menu.in_schedule(OnEnter(GameState::MainMenu)),
            exit_main_menu.in_schedule(OnExit(GameState::MainMenu)),
        ));
        // Show or clear UI of each state of main menu.
        app.add_systems((
            init_main_menu_index.in_schedule(OnEnter(MainMenuUIState::Index)),
            clear_main_menu_index.in_schedule(OnExit(MainMenuUIState::Index)),
            init_main_menu_choose_world.in_schedule(OnEnter(MainMenuUIState::ChooseWorld)),
            clear_main_menu_choose_world.in_schedule(OnExit(MainMenuUIState::ChooseWorld)),
            init_main_menu_settings.in_schedule(OnEnter(MainMenuUIState::Settings)),
            clear_main_menu_settings.in_schedule(OnExit(MainMenuUIState::Settings)),
        ));
        // React to clicks.
        app.add_systems((
            main_menu_index_start_button_reaction.in_set(OnUpdate(MainMenuUIState::Index)),
        ));

        // In-game UI.
        app.add_state::<InGameUIState>();
        app.add_system(init_in_game_ui_text.in_schedule(OnEnter(GameState::InGame)));
        app.add_system(update_in_game_ui_text.in_set(OnUpdate(GameState::InGame)));
        app.add_system(in_game_pause_reaction.in_set(OnUpdate(GameState::InGame)));

        // Pause UI
        // Enter the Pause State.

        app.add_state::<PauseUIState>();
        app.add_systems((
            enter_pause.in_schedule(OnEnter(GameState::Pause)),
            exit_pause.in_schedule(OnExit(GameState::Pause)),
        ));

        // Show or clear UI of each state of Pause State.
        app.add_system(init_pause_index.in_schedule(OnEnter(PauseUIState::Pause)));
        app.add_system(clear_pause_index.in_schedule(OnExit(PauseUIState::Pause)));

        // React to clicks in Pause state.
        app.add_systems((
            pause_index_return_button_reaction.in_set(OnUpdate(PauseUIState::Pause)),
            pause_index_main_menu_button_reaction.in_set(OnUpdate(PauseUIState::Pause)),
            pause_index_exit_button_reaction.in_set(OnUpdate(PauseUIState::Pause)),
        ));
    }
}

/**
The enum that represents the state of the main menu UI. This is a global resource.
 */
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
enum MainMenuUIState {
    #[default]
    None,
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

/**
The enum that represents the state of the pause UI. This is a global resource.
 */
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
enum PauseUIState {
    #[default]
    None,
    Pause,
}

// Below are the group identifiers of the buttons, texts, etc.
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

/// A "tag" component for a section of pause UI on index page.
#[derive(Component)]
struct PauseIndexUI;
/// A "tag" component for a section of pause UI on return game page.
#[derive(Component)]
struct PauseReturnGameUI;

/// A "tag" component for a section of pause UI on return memu page.
#[derive(Component)]
struct PauseReturnMEnuUI;

/// A "tag" component for a section of pause UI on return memu page.
#[derive(Component)]
struct PauseReturnMenuUI;

/// A "tag" component for a section of pause UI on exit page.
#[derive(Component)]
struct PauseExituUI;

// Below are the names of individual buttons, texts, etc.
/// A "name" for the start button on the main menu index.
#[derive(Component)]
struct MainMenuIndexUIStartButton;
/// A "name" for the bottom-left text area in-game UI.
#[derive(Component)]
struct InGameUIBottomLeftText;

/// A "name" for the return game button on the pause index.
#[derive(Component)]
struct PauseIndexUIReturnButton;

/// A "name" for the main menu button on the pause index.
#[derive(Component)]
struct PauseIndexUIMainmenuButton;

/// A "name" for the exit button on the pause index.
#[derive(Component)]
struct PauseIndexUIExitButton;

// Below are the behaviors when state changes.
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
    for camera in &query_camera {
        commands.entity(camera).despawn_recursive();
    }
    main_menu_state.set(MainMenuUIState::None);
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
Initialize the main menu UI on choose world page.
 */
fn init_main_menu_choose_world(mut commands: Commands, asset_server: Res<AssetServer>) {}

/**
Clears the main menu choose world page UI.
 */
fn clear_main_menu_choose_world(
    mut commands: Commands,
    query_ui: Query<Entity, With<MainMenuChooseWorldUI>>,
) {
    for ui in &query_ui {
        commands.entity(ui).despawn_recursive();
    }
}

/**
Initialize the main menu UI on settings page.
 */
fn init_main_menu_settings(mut commands: Commands, asset_server: Res<AssetServer>) {}

/**
Clears the main menu settings page UI.
 */
fn clear_main_menu_settings(
    mut commands: Commands,
    query_ui: Query<Entity, With<MainMenuSettingsUI>>,
) {
    for ui in &query_ui {
        commands.entity(ui).despawn_recursive();
    }
}

// Below is how to react to clicks.

/// Enter the game.
fn main_menu_index_start_button_reaction(
    mut interaction_query: Query<&Interaction, With<MainMenuIndexUIStartButton>>,
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

/// From in_game state to pause state
fn in_game_pause_reaction(key: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if key.pressed(KeyCode::Escape) {
        game_state.set(GameState::Pause);
    }
}

/**
    Initialize the UI camera for pause state
*/
fn enter_pause(mut commands: Commands, mut pause_state: ResMut<NextState<PauseUIState>>) {
    pause_state.set(PauseUIState::Pause);
    commands.spawn((UICamera, Camera2dBundle { ..default() }));
}

/**
   Clears the UI camera for pause state, and set the UI state to None.
*/
fn exit_pause(
    mut commands: Commands,
    mut pause_state: ResMut<NextState<PauseUIState>>,
    query_camera: Query<Entity, With<UICamera>>,
) {
    pause_state.set(PauseUIState::None);
    for camera in &query_camera {
        commands.entity(camera).despawn_recursive();
    }
}

/**
Initialize the pause UI on index page.
 */
fn init_pause_index(mut commands: Commands, asset_server: Res<AssetServer>) {
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
Clears the main menu index page UI.
 */
fn clear_pause_index(mut commands: Commands, query_ui: Query<Entity, With<MainMenuIndexUI>>) {
    for ui in &query_ui {
        commands.entity(ui).despawn_recursive();
    }
}

/// Pause state mouse response.
fn pause_index_return_button_reaction(
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

fn pause_index_main_menu_button_reaction(
    mut interaction_query: Query<&Interaction, With<PauseIndexUIMainmenuButton>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::MainMenu);
            }
            _ => {}
        }
    }
}

fn pause_index_exit_button_reaction(
    mut interaction_query: Query<&Interaction, With<PauseIndexUIExitButton>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
}
