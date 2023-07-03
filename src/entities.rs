use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

const TIME_STEP: f32 = 1.0 / 60.0;

pub struct EntityUpdatePlugin;

impl Plugin for EntityUpdatePlugin {
    fn build(&self, app: &mut App) {
        // Update entities at fixed intervals.
        app.insert_resource(FixedTime::new_from_secs(TIME_STEP));
        app.add_system(entity_move.in_schedule(CoreSchedule::FixedUpdate));
    }
}

#[derive(Component)]
pub struct Entity;

#[derive(Component)]
pub struct Creeper;

#[derive(Component)]
pub struct Torch;

#[derive(Component)]
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
}

/**
Make the entity move according to its velocity, and write its new position, rotation and scaling into its `EntityStatus`.
 */
fn entity_move(
    mut query_entity_status: Query<(&EntityStatusPointer, &mut Transform), With<Entity>>,
) {
    for (status_ptr, mut transform) in query_entity_status.iter_mut() {
        let mut status: std::sync::MutexGuard<EntityStatus> = status_ptr.pointer.lock().unwrap();
        let movement = status.velocity * TIME_STEP;
        transform.translation += movement;
        status.position = transform.translation;
        status.rotation = transform.rotation.to_euler(EulerRot::YZX).0;
        status.scaling = transform.scale;
        // println!("Transform of {}: {:?}", status.entity_type, transform)
    }
}
