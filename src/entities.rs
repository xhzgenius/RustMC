use crate::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

const TIME_STEP: f32 = 1.0 / 60.0;

/// Plugin responsible for the update of entities.
/// Currently an entity only move itself duing update stage.
pub struct EntityUpdatePlugin;
impl Plugin for EntityUpdatePlugin {
    fn build(&self, app: &mut App) {
        // Update entities at fixed intervals.
        app.insert_resource(FixedTime::new_from_secs(TIME_STEP));
        app.add_systems((entity_move, gravity, die).in_schedule(CoreSchedule::FixedUpdate));
    }
}

/// A "tag" component for all entities.
/// Entities all have an `EntityStatusPointer` component.
#[derive(Component)]
pub struct Entity;

/// A "tag" component for the entity type "Creeper".
#[derive(Component)]
pub struct Creeper;

/// A "tag" component for the entity type "Torch".
#[derive(Component)]
pub struct Torch;

/**
A ref-counted pointer with lock, pointing to the entity's EntityStatus.
Use this pointer like this:
```
let mut status: std::sync::MutexGuard<EntityStatus> = status_ptr.pointer.lock().unwrap();
status.health += 1;
```
 */
#[derive(Component, Debug)]
pub struct EntityStatusPointer {
    pub pointer: Arc<Mutex<EntityStatus>>,
}

#[derive(Serialize, Deserialize, Debug)]
/**
This is the status for any entity. It is stored on heap, and shared by `Arc<Mutex<EntityStatus>>`.
*/
pub struct EntityStatus {
    /// The entity type, e.g. `"Creeper".to_string()`.
    pub entity_type: String,
    /// The health (i32). Current health limit for player is `20`.
    pub health: i32,
    /// The absolute position (Vec3). Synchronized with this entity's `Transform.translation`.
    pub position: Vec3,
    /// The absolute rotation in radians.
    /// Synchronized with this entity's `Transform.rotation.to_euler(EulerRot::YZX).0`.
    pub rotation: f32,
    /// The scaling factor (Vec3). Synchronized with this entity's `Transform.scale`.
    pub scaling: Vec3,
    /// The absolute velocity (Vec3).
    pub velocity: Vec3,
    /// The attack CD, In seconds.
    pub attack_cd: f32,
}

fn check_whether_in_game(game_state: Res<State<GameState>>) -> bool {
    return game_state.0 == GameState::InGame;
}

/**
Make the entity move according to its velocity, and write its new position, rotation and scaling into its `EntityStatus`.
 */
fn entity_move(
    mut query_entity_status: Query<(&EntityStatusPointer, &mut Transform), With<Entity>>,
    game_state: Res<State<GameState>>,
) {
    if check_whether_in_game(game_state) == false {
        return;
    }
    for (status_ptr, mut transform) in query_entity_status.iter_mut() {
        let mut status: std::sync::MutexGuard<EntityStatus> = status_ptr.pointer.lock().unwrap();
        let movement = status.velocity * TIME_STEP;
        transform.translation += movement;
        status.position = transform.translation;
        status.rotation = transform.rotation.to_euler(EulerRot::YZX).0;
        status.scaling = transform.scale;
        status.attack_cd -= TIME_STEP;
    }
}

fn gravity(
    mut query_entity_status: Query<&EntityStatusPointer, With<Entity>>,
    gamemap: Res<gamemap::GameMap>,
    game_state: Res<State<GameState>>,
) {
    if check_whether_in_game(game_state) == false {
        return;
    }
    for status_ptr in query_entity_status.iter_mut() {
        let mut status: std::sync::MutexGuard<EntityStatus> = status_ptr.pointer.lock().unwrap();
        let block_id = gamemap.query_block_by_xyz(status.position);
        if block_id.unwrap_or(-1) < 0 {
            status.velocity += Vec3::new(0., -9.8 * TIME_STEP, 0.);
        } else {
            status.velocity[1] = f32::max(status.velocity[1], 0.);
        }
    }
}

fn die(
    mut query_entity_status: Query<
        (bevy::ecs::prelude::Entity, &EntityStatusPointer),
        With<Entity>,
    >,
    mut commands: Commands,
) {
    for (entity, status_ptr) in query_entity_status.iter_mut() {
        let status: std::sync::MutexGuard<EntityStatus> = status_ptr.pointer.lock().unwrap();
        if status.health <= 0 {
            commands.entity(entity).despawn_recursive();
            println!("{:?} died!!!", entity);
        }
    }
}
