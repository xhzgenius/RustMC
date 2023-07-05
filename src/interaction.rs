use std::sync::Arc;

use crate::*;
use bevy::prelude::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerTarget>();
        app.init_resource::<Events<GameEntityEvent>>();
        app.add_systems(
            (player_find_target, handle_entity_events).in_set(OnUpdate(GameState::InGame)),
        );
    }
}

pub enum GameEventOpration {
    HIT(i32),
    USE,
}
pub struct GameEntityEvent {
    pub target: entities::EntityStatusPointer,
    pub operation: GameEventOpration,
}

/// The player's current target entity or block position.
#[derive(Resource, Default, Debug)]
pub struct PlayerTarget {
    pub entity_status_ptr: Option<entities::EntityStatusPointer>,
    pub block: Option<Vec3>,
}

/// Update the player's possible target.
/// If there is no entity in the player's operation range, returns None.
/// TODO: Implement the selection algorithm.
fn player_find_target(
    mut target: ResMut<PlayerTarget>,
    query_camera_transform: Query<&GlobalTransform, With<init_game::GameCamera>>,
    query_entities: Query<
        (&entities::EntityStatusPointer, &Handle<Mesh>),
        (With<entities::Entity>, Without<player::MainPlayer>),
    >,
    gamemap: Res<gamemap::GameMap>,
    meshes: Res<Assets<Mesh>>,
) {
    let transform = query_camera_transform
        .get_single()
        .expect("Not exactly one main player!");
    let center = transform.translation();
    let forward = transform.forward();
    let mut nearest_point_id: usize = 114514;
    let mut points = vec![];
    for i in 0..51 {
        points.push(center + forward * i as f32 * 0.1);
    }
    for (entity_status_ptr, collision_box) in query_entities.iter() {
        let status = entity_status_ptr.pointer.lock().unwrap();
        let box_ = meshes.get(collision_box).unwrap().compute_aabb().unwrap();
        let min_x = box_.min().x + status.position.x;
        let min_y = box_.min().y + status.position.y;
        let min_z = box_.min().z + status.position.z;
        let max_x = box_.max().x + status.position.x;
        let max_y = box_.max().y + status.position.y;
        let max_z = box_.max().z + status.position.z;
        for point_id in 0..points.len() {
            let point = points[point_id];
            if min_x < point.x
                && point.x < max_x
                && min_y < point.y
                && point.y < max_y
                && min_z < point.z
                && point.z < max_z
                && point_id < nearest_point_id
            {
                // In range.
                target.entity_status_ptr = Some(entities::EntityStatusPointer {
                    pointer: Arc::clone(&entity_status_ptr.pointer),
                });
                nearest_point_id = point_id;
                // println!("In range: {:?}", target.entity_status_ptr);
                break;
            }
        }
    }
    // target.entity_status_ptr = None;
    target.block = None; // Placeholder
                         // println!("Player's target: {:#?}", target);
}

/// Tell the game engine that an operation will be performed on an entity.
pub fn send_event_to_entity(
    entity_status_ptr: entities::EntityStatusPointer,
    operation: GameEventOpration,
    mut event_writer: EventWriter<GameEntityEvent>,
) {
    event_writer.send(GameEntityEvent {
        target: entity_status_ptr,
        operation: operation,
    });
}

/// Deal with eneity events.
fn handle_entity_events(mut event_reader: EventReader<GameEntityEvent>) {
    for event in event_reader.iter() {
        let entity_status_ptr = &event.target;
        let mut entity_status = entity_status_ptr.pointer.lock().unwrap();
        match event.operation {
            GameEventOpration::HIT(damage) => {
                entity_status.health -= damage;
                entity_status.velocity.y += 2.;
            }
            GameEventOpration::USE => {}
        }
    }
}
