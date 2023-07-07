use crate::{init_game::GameCamera, *};
use bevy::{prelude::*, ecs::world};
use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
};

use super::mainmenu::*;
use super::chooseworld::*;
use super::ingame::*;
// use super::setting::*;
use super::pause::*; 

// Below are the group identifiers of the buttons, texts, etc.

// Below are the behaviors when state changes.

// Below are behaviors that will be taken during the state.
/**
Initialize the main menu UI on settings page.
 */
pub fn init_main_menu_settings(mut commands: Commands, asset_server: Res<AssetServer>) {}

/**
Clears the main menu settings page UI.
 */
pub fn clear_main_menu_settings(
    mut commands: Commands,
    query_ui: Query<Entity, With<MainMenuSettingsUI>>,
) {
    for ui in &query_ui {
        commands.entity(ui).despawn_recursive();
    }
}


// Below is how to react to clicks.