use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

const TIME_STEP: f32 = 1.0 / 10.0;

pub struct EntityUpdatePlugin;

impl Plugin for EntityUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(TIME_STEP));
        app.add_system(update_entity);
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
pub struct EntityStatus {
    pub entity_type: String,
    pub health: i32,
    pub position: Vec3,
    pub rotation: f32,
    pub scaling: Vec3,
    pub velocity: Vec3,
}

fn update_entity(
    mut query_entity_status: Query<(&EntityStatusPointer, &mut Transform), With<Entity>>,
) {
    for (status_ptr, mut transform) in query_entity_status.iter_mut() {
        let mut status: std::sync::MutexGuard<EntityStatus> = status_ptr.pointer.lock().unwrap();
        let movement = status.velocity * TIME_STEP;
        transform.translation += movement;
        status.position = transform.translation;
        // status.rotation = transform.rotation;
        status.scaling = transform.scale;
        // println!("Transform of {}: {:?}", status.entity_type, transform)
    }
}
