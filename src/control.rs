use std::sync::Arc;

use crate::{entities::EntityStatusPointer, *};
use bevy::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(control);
    }
}

fn control(
    keys: Res<Input<KeyCode>>,
    mut query_main_player_status: Query<&mut entities::EntityStatusPointer, With<player::MainPlayer>>,
) {
    // Unwrap the Arc into mutable reference. 
    let status_pointer: &mut EntityStatusPointer = &mut query_main_player_status
        .get_single_mut()
        .expect("Not exactly one player!");
    let mut status = status_pointer.pointer.lock().unwrap();
    // Operate the status. 
    status.velocity.x = 0.;
    status.velocity.z = 0.;
    if keys.pressed(KeyCode::W) {
        status.velocity.x = 1.0;
    }
    if keys.pressed(KeyCode::A) {
        status.velocity.z = -1.0;
    }
    if keys.pressed(KeyCode::S) {
        status.velocity.x = -1.0;
    }
    if keys.pressed(KeyCode::D) {
        status.velocity.z = 1.0;
    }
    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        // Either the left or right shift are being held down
    }
}
