use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

use crate::*;
use crate::entities::Creeper;
use crate::player::Player;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

/// Plugin responsible for controlling the player.
pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (walk, rotate, head_up, lock_mouse_cursor, operate, random_move).in_set(OnUpdate(GameState::InGame)),
        );
        app.add_systems((
            hide_cursor.in_schedule(OnEnter(GameState::InGame)),
            show_cursor.in_schedule(OnExit(GameState::InGame)),
        ));
    }
}

const MAX_VELOCITY: f32 = 4.;
/// This system is used to make the main player walk.
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
    // status.velocity.y = 0.;
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
        status.velocity.y = MAX_VELOCITY;
    }
    if keys.pressed(KeyCode::LShift) {
        status.velocity.y = -MAX_VELOCITY;
    }
    let abs_velocity: f32 =
        f32::sqrt(status.velocity.x * status.velocity.x + status.velocity.z * status.velocity.z);
    status.velocity.x *= MAX_VELOCITY / f32::max(1., abs_velocity);
    status.velocity.z *= MAX_VELOCITY / f32::max(1., abs_velocity);
    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        // Either the left or right shift are being held down
    }
}

// Random walk for players and creepers.
fn random_move(
    keys: Res<Input<KeyCode>>,
    mut query_player_status: Query<
        (&mut entities::EntityStatusPointer, &mut Transform),
        (With<entities::Entity>, Without<player::MainPlayer>)
    >,
){
    for (status_pointer, mut transform) in &mut query_player_status {
        if rand::random::<f32>() < 0.05 {
            let mut status = status_pointer.pointer.lock().unwrap();
            if rand::random::<f32>() < 0.5 {
                status.velocity.x = 0.;
                status.velocity.z = 0.;
            }
            else{
                let angle = 2. * PI * rand::random::<f32>();
                transform.rotate_y(angle);
                let direction = transform.forward();
                let abs_velocity = 5. * rand::random::<f32>();
                status.velocity.x = direction[0] * abs_velocity;
                status.velocity.z = direction[2] * abs_velocity;
            }
        }  
    }
}

/// This system is used to lock mouse cursor position when mouse is in the window.
fn lock_mouse_cursor(mut windows: Query<&mut Window>, _key: Res<Input<KeyCode>>) {
    let mut window = windows
        .get_single_mut()
        .expect("There is not exactly one window. ");
    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    let lock_position = Vec2::new(window.width() / 2., window.height() / 2.);
    window.set_cursor_position(Some(lock_position));
}

/// This sstem is used to hide cursor when entering InGame state.
fn hide_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows
        .get_single_mut()
        .expect("There is not exactly one window. ");
    window.cursor.visible = false;
}

/// This sstem is used to show cursor when exiting InGame state.
fn show_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows
        .get_single_mut()
        .expect("There is not exactly one window. ");
    window.cursor.visible = true;
}

const MOUSE_SENSITIVITY_HORIZONTAL: f32 = 0.2;
/// This system is used to rotate main player horizontally (around Y-axis).
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
/// This system is used to control the vertical angle of the camera.
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

const PLAYER_ATTACK_CD: f32 = 0.5;
fn operate(
    clicks_input: Res<Input<MouseButton>>,
    entity_event_writer: EventWriter<interaction::GameEntityEvent>,
    block_event_writer: EventWriter<interaction::GameBlockEvent>,
    target: Res<interaction::PlayerTarget>,
    query_main_player_status_ptr: Query<&entities::EntityStatusPointer, With<player::MainPlayer>>,
) {
    let mut status = query_main_player_status_ptr
        .get_single()
        .expect("Not exactly one main player!")
        .pointer
        .lock()
        .unwrap();
    match &target.entity_status_ptr {
        Some(entity_status_ptr) => {
            if clicks_input.just_pressed(MouseButton::Left) {
                if status.attack_cd <= 0. {
                    interaction::send_event_to_entity(
                        entities::EntityStatusPointer {
                            pointer: Arc::clone(&entity_status_ptr.pointer),
                        },
                        interaction::GameEventOpration::HIT(5), // TODO: Change damage value.
                        entity_event_writer,
                    );
                    status.attack_cd = PLAYER_ATTACK_CD;
                }
            } else if clicks_input.pressed(MouseButton::Right) {
                interaction::send_event_to_entity(
                    entities::EntityStatusPointer {
                        pointer: Arc::clone(&entity_status_ptr.pointer),
                    },
                    interaction::GameEventOpration::USE,
                    entity_event_writer,
                );
            }
            return;
        }
        None => {}
    };
    match target.block {
        Some(block) => {
            if clicks_input.just_pressed(MouseButton::Left) {
                interaction::send_event_to_block(
                    block,
                    interaction::GameEventOpration::HIT(0),
                    block_event_writer,
                )
            } else if clicks_input.pressed(MouseButton::Right) {
                interaction::send_event_to_block(
                    block,
                    interaction::GameEventOpration::USE,
                    block_event_writer,
                )
            }
        }
        None => {}
    }
}
