use std::f32::consts::PI;

use crate::{entities::EntityStatusPointer, *};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(walk);
        app.add_system(rotate);
        app.add_system(lock_mouse_cursor);
    }
}

const MAX_VELOCITY: f32 = 3.;
fn walk(
    keys: Res<Input<KeyCode>>,
    mut query_main_player_status: Query<
        (&mut entities::EntityStatusPointer, &Transform),
        With<player::MainPlayer>,
    >,
) {
    // Unwrap the Arc into mutable reference.
    let (status_pointer, transform) = &mut query_main_player_status
        .get_single_mut()
        .expect("Not exactly one main player!");
    let mut status = status_pointer.pointer.lock().unwrap();
    // Operate the status.
    status.velocity.x = 0.;
    status.velocity.y = 0.;
    status.velocity.z = 0.;
    // x means right, z means back, y means top.
    if keys.pressed(KeyCode::W) {
        status.velocity += transform.forward();
    }
    if keys.pressed(KeyCode::A) {
        status.velocity += transform.left();
    }
    if keys.pressed(KeyCode::S) {
        status.velocity += transform.back();
    }
    if keys.pressed(KeyCode::D) {
        status.velocity += transform.right();
    }
    if keys.pressed(KeyCode::Space) {
        status.velocity += transform.up();
    }
    if keys.pressed(KeyCode::LShift) {
        status.velocity += transform.down();
    }
    let abs_velocity = status.velocity.length();
    status.velocity *= MAX_VELOCITY / f32::max(1., abs_velocity);
    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        // Either the left or right shift are being held down
    }
}

fn lock_mouse_cursor(mut windows: Query<&mut Window>, key: Res<Input<KeyCode>>) {
    let mut window = windows.get_single_mut().unwrap();

    if key.pressed(KeyCode::Escape) {
        return;
    }
    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    let lock_position = Vec2::new(window.width()/2., window.height()/2.);
    window.set_cursor_position(Some(lock_position));
}

const MOUSE_SENSITIVITY_HORIZONTAL: f32 = 0.2;
fn rotate(
    mut motion_evr: EventReader<MouseMotion>,
    mut query_main_player_transform: Query<&mut Transform, With<player::MainPlayer>>,
) {
    let mut transform = query_main_player_transform
        .get_single_mut()
        .expect("Not exactly one main player!");
    for mouse_motion in motion_evr.iter() {
        transform.rotate_y(-mouse_motion.delta[0] * MOUSE_SENSITIVITY_HORIZONTAL * PI / 180.0);
    }
}
