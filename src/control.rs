use std::f32::consts::PI;
use std::sync::Mutex;

use crate::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

/// Plugin responsible for controlling the player.
pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (walk, rotate, head_up, lock_mouse_cursor).in_set(OnUpdate(GameState::InGame)),
        );
    }
}

const MAX_VELOCITY: f32 = 5.;
/**
This system is used to make the main player walk.
 */
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

/**
This system is used to lock mouse cursor position when mouse is in the window.
 */
fn lock_mouse_cursor(mut windows: Query<&mut Window>, key: Res<Input<KeyCode>>) {
    let mut window = windows
        .get_single_mut()
        .expect("There is not exactly one window. ");

    if key.pressed(KeyCode::Escape) {
        return;
    }
    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    let lock_position = Vec2::new(window.width() / 2., window.height() / 2.);
    window.set_cursor_position(Some(lock_position));
}

const MOUSE_SENSITIVITY_HORIZONTAL: f32 = 0.2;
/**
This system is used to rotate main player horizontally (around Y-axis).
*/
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

const MOUSE_SENSITIVITY_VERTICAL: f32 = 0.2;
/**
This system is used to control the vertical angle of the camera.
 */
fn head_up(
    mut motion_evr: EventReader<MouseMotion>,
    mut query_camera_transform: Query<&mut Transform, With<init_game::GameCamera>>,
) {
    static HEAD_UP_ANGLE: Mutex<f32> = Mutex::new(0.);
    let mut transform = query_camera_transform
        .get_single_mut()
        .expect("Not exactly one camera!");
    for mouse_motion in motion_evr.iter() {
        let delta_angle = -mouse_motion.delta[1] * MOUSE_SENSITIVITY_VERTICAL;
        let new_angle = *HEAD_UP_ANGLE.lock().unwrap() + delta_angle;
        *HEAD_UP_ANGLE.lock().unwrap() = f32::min(f32::max(new_angle, -90.), 90.);
        transform.rotation = Quat::from_rotation_x(*HEAD_UP_ANGLE.lock().unwrap() * PI / 180.);
    }
}
