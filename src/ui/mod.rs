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

use crate::{init_game::GameCamera, *};
use bevy::{prelude::*, ecs::world};
use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
};

pub mod chooseworld;
pub mod ingame;
pub mod mainmenu;
pub mod pause;
pub mod setting;
use mainmenu::*;
use chooseworld::*;
use ingame::*;
use setting::*;
use pause::*;


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
        app.add_system(update_in_game_ui_cursor.in_schedule(OnEnter(GameState::InGame)));
        app.add_system(update_in_game_ui_text.in_set(OnUpdate(GameState::InGame)));
        app.add_system(clear_in_game_ui.in_schedule(OnExit(GameState::InGame)));

        // React to esc in Game state.
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
            //pause_index_exit_button_reaction.in_set(OnUpdate(PauseUIState::Pause)),
        ));
    }
}

// Below are the group identifiers of the buttons, texts, etc.
/// A "tag" component for the UI camera.
#[derive(Component)]
pub struct UICamera;