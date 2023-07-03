use crate::*;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::Arc;

/// Plugin resposible for initializing scene (and camera) for the game.
pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_blocks_and_entities);
        // app.add_system(update_camera);
    }
}

/**
 Initialize the whole scene in the game, in other words, load all blocks and entities and the camera.
*/
fn init_blocks_and_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    game_map: Res<gamemap::GameMap>,
) {
    // Prepare model for a block.
    let block_mesh = meshes.add(shape::Cube { size: 1.0 }.into());
    // Prepare material for every kind of blocks.
    let block_materials: Vec<Handle<StandardMaterial>> =
        load_block_textures(&asset_server, materials);
    let entity_models: HashMap<String, Handle<Scene>> = load_entity_models(&asset_server);

    // Spawn all blocks in the gamemap.
    for &(chunks_x, chunks_z) in game_map.map.keys() {
        let chunk = &game_map.map[&(chunks_x, chunks_z)];
        let chunk_blocks = chunk.blocks.lock().unwrap();
        for x in 0..gamemap::CHUNK_SIZE {
            for y in 0..gamemap::CHUNK_HEIGHT {
                for z in 0..gamemap::CHUNK_SIZE {
                    let block_id = chunk_blocks[x][y][z];
                    if let Some(block_material) = block_materials.get(block_id as usize) {
                        commands.spawn((
                            blocks::Block,
                            PbrBundle {
                                mesh: block_mesh.clone(),
                                material: block_material.clone(),
                                transform: Transform::from_xyz(
                                    (chunks_x * gamemap::CHUNK_SIZE as i32 + x as i32) as f32,
                                    y as f32,
                                    (chunks_z * gamemap::CHUNK_SIZE as i32 + z as i32) as f32,
                                ),
                                ..default()
                            },
                        ));
                    } // If block_id is negative or out of bound, treat as air.
                }
            }
        }
    }

    // Spawn all entities in the scene.
    for &(chunks_x, chunks_z) in game_map.map.keys() {
        let chunk = &game_map.map[&(chunks_x, chunks_z)];
        for entity_status_locked in &chunk.entities {
            let entity_status = entity_status_locked.lock().unwrap();
            let mut entity_transform: Transform =
                Transform::from_translation(entity_status.position)
                    .with_scale(entity_status.scaling);
            entity_transform.rotate_y(entity_status.rotation);
            let entity_model_name = find_model_name_by_type(&entity_status.entity_type);
            // First spawn the entity's status pointer and bounding box.
            let mut entity_commands = commands.spawn((
                entities::EntityStatusPointer {
                    pointer: Arc::clone(entity_status_locked),
                },
                PbrBundle {
                    transform: entity_transform,
                    ..default()
                },
            ));
            // Then insert entity tags into the entity.
            insert_entity_tags(&mut entity_commands, &entity_status.entity_type);
            // Then spawn the entity's shown model.
            entity_commands.with_children(|parent| {
                parent.spawn((SceneBundle {
                    scene: entity_models
                        .get(entity_model_name)
                        .expect(&format!("Model not loaded: {}", entity_model_name))
                        .clone(),
                    transform: get_proper_model_transform_by_type(&entity_status.entity_type),
                    visibility: match &entity_status.entity_type as &str {
                        "MainPlayer" => Visibility::Hidden,
                        _ => Visibility::Visible,
                    },
                    ..default()
                },));
            });
        }
    }

    // Spawn the sunlight.
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 30000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-PI * 0.5)),
        ..default()
    });
}
/// A "tag" component for the game camera.
#[derive(Component)]
pub struct GameCamera;

/**
 Load textures of every kind of blocks.
*/
fn load_block_textures(
    asset_server: &Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Vec<Handle<StandardMaterial>> {
    let block_textures = asset_server.load_folder("blocks")
        .expect("Failed to load block texture. Please eusure that block texture is in ./assets/blocks/ folder. ");
    // I think panic here is necessary. --XHZ
    let mut block_materials: Vec<Handle<StandardMaterial>> = Vec::new();
    for block_texture in block_textures {
        block_materials.push(materials.add(StandardMaterial {
            base_color_texture: Some(block_texture.typed()),
            ..default()
        }));
    }
    return block_materials;
}

/**
 Load models (including meshes and textures) of every kind of entities.
*/
fn load_entity_models(asset_server: &Res<AssetServer>) -> HashMap<String, Handle<Scene>> {
    let mut entity_models: HashMap<String, Handle<Scene>> = HashMap::new();
    for model_name in walkdir::WalkDir::new("./assets/models/") {
        let model_name = format!("{}", model_name.unwrap().file_name().to_str().unwrap());
        let model_path = format!("models/{}#Scene0", model_name.clone());
        let model: Handle<Scene> = asset_server.load(model_path);
        entity_models.insert(model_name, model);
    }
    return entity_models;
}

/// Util function.
fn find_model_name_by_type(entity_type: &str) -> &str {
    match entity_type {
        "Creeper" => "minecraft_creeper.glb",
        "Player" => "minecraft_steve.glb",
        "MainPlayer" => "minecraft_steve.glb",
        "Torch" => "minecraft_torch.glb",
        _ => panic!("Unknown entity type: {}", entity_type),
    }
}

/// Util function.
fn get_proper_model_transform_by_type(entity_type: &str) -> Transform {
    match entity_type {
        "Creeper" => {
            Transform::from_scale(Vec3::new(0.1, 0.1, 0.1)).with_rotation(Quat::from_rotation_y(PI))
        }
        "Player" => {
            Transform::from_scale(Vec3::new(0.1, 0.1, 0.1)).with_rotation(Quat::from_rotation_y(PI))
        }
        "MainPlayer" => {
            Transform::from_scale(Vec3::new(0.1, 0.1, 0.1)).with_rotation(Quat::from_rotation_y(PI))
        }
        "Torch" => Transform::from_scale(Vec3::new(0.5, 0.5, 0.5)),
        _ => panic!("Unknown entity type: {}", entity_type),
    }
}

/// Util function.
fn insert_entity_tags(entity_commands: &mut EntityCommands, entity_type: &str) {
    match entity_type {
        "MainPlayer" => entity_commands
            .insert((entities::Entity, player::Player, player::MainPlayer))
            .with_children(|parent| {
                parent.spawn((
                    GameCamera,
                    Camera3dBundle {
                        // transform: Transform::from_xyz(0., 3.0, 8.0), // Third person camera
                        transform: Transform::from_xyz(0., 1.7, -0.4), // First person camera
                        ..default()
                    },
                ));
            }),
        "Player" => entity_commands.insert((entities::Entity, player::Player)),
        "Creeper" => entity_commands.insert((entities::Entity, entities::Creeper)),
        "Torch" => entity_commands.insert((entities::Entity, entities::Torch)),
        _ => panic!("Unknown entity type: {}", entity_type),
    };
}
