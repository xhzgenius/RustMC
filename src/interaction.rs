use std::sync::Arc;

use crate::*;
use bevy::prelude::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerTarget>();
        app.init_resource::<Events<GameEntityEvent>>();
        app.init_resource::<Events<GameBlockEvent>>();
        app.add_systems(
            (
                player_find_target,
                handle_entity_events,
                handle_block_events,
            )
                .in_set(OnUpdate(GameState::InGame)),
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
pub struct GameBlockEvent {
    pub target_position: Vec3,
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
    // Clear targets.
    target.entity_status_ptr = None;
    target.block = None;
    // Calculate possible collition points.
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
    // Iterate through possible collision points, and take the nearest.
    for point_id in 0..points.len() {
        // Check collision with entities.
        for (entity_status_ptr, collision_box) in query_entities.iter() {
            let status = entity_status_ptr.pointer.lock().unwrap();
            let box_ = meshes.get(collision_box).unwrap().compute_aabb().unwrap();
            let point = points[point_id];
            if entities::collide_with(
                status.position + Vec3::new(box_.min().x, box_.min().y, box_.min().z),
                status.position + Vec3::new(box_.max().x, box_.max().y, box_.max().z),
                point,
                point,
            ) && point_id < nearest_point_id
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
        // Check collision with blocks.
        let block_id = gamemap.query_block_by_xyz(points[point_id]).unwrap_or(-1);
        if block_id > 0 {
            target.block = Some(points[point_id]);
            break;
        }
    }
    //  println!("Player's target: {:#?}", target);
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

/// Tell the game engine that an operation will be performed on an entity.
pub fn send_event_to_block(
    target_position: Vec3,
    operation: GameEventOpration,
    mut event_writer: EventWriter<GameBlockEvent>,
) {
    event_writer.send(GameBlockEvent {
        target_position: target_position,
        operation: operation,
    });
}

/// Deal with block events.
fn handle_block_events(
    mut event_reader: EventReader<GameBlockEvent>,
    gamemap: ResMut<gamemap::GameMap>,
    mut commands: Commands,
    block_entity_id_map: ResMut<init_game::BlockEntityIDMap>,
) {
    for event in event_reader.iter() {
        let target_potision = event.target_position;
        let chunk_key = gamemap.query_chunk_by_xyz(target_potision);
        match gamemap.to_integer(target_potision) {
            Some((x, y, z)) => {
                // Target block is found.
                match event.operation {
                    GameEventOpration::HIT(_damage) => {
                        gamemap.map.get(&chunk_key).unwrap().blocks.lock().unwrap()[x][y][z] = -1;
                        commands
                            .entity(
                                *block_entity_id_map
                                    .map
                                    .get(&(
                                        target_potision.x.floor() as i32,
                                        target_potision.y.floor() as i32,
                                        target_potision.z.floor() as i32,
                                    ))
                                    .unwrap(),
                            )
                            .despawn_recursive();
                    }
                    GameEventOpration::USE => {}
                }
            }
            None => {}
        }
    }
}
