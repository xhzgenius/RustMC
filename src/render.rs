use crate::*;
use bevy::ecs::system::EntityCommands;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_blocks_and_entities);
        app.add_startup_system(setup_camera);
        app.add_system(update_camera);
    }
}

/**
 The player's starting transform. Can be configured.
 TODO: Save the player's starting transform in the save.
*/
fn player_starting_transform() -> Transform {
    let mut transform = Transform::from_xyz(0., gamemap::CHUNK_HEIGHT as f32 / 2. + 5., 0.);
    transform.look_at(
        Vec3::new(20., gamemap::CHUNK_HEIGHT as f32 / 2. - 5., 10.),
        Vec3::new(0., 1., 0.),
    );
    return transform;
}

/**
 Initialize the whole scene in the game, in other words, load all blocks and entities.
*/
fn init_blocks_and_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_map: Res<gamemap::GameMap>,
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
            let mut entity_status = entity_status_locked.lock().unwrap();
            let mut entity_transform: Transform =
                Transform::from_translation(entity_status.position)
                    .with_scale(entity_status.scaling);
            entity_transform.rotate_y(entity_status.rotation);
            let entity_model_name = match find_model_name_by_type(&entity_status.entity_type) {
                Some(model_name) => model_name,
                None => panic!(
                    "{}",
                    format!(
                        "Model not found for entity type {}! ",
                        entity_status.entity_type
                    )
                ),
            };
            let mut entity_commands = commands.spawn((
                entities::Entity,
                entities::EntityStatusPointer {
                    pointer: Arc::clone(entity_status_locked),
                },
                SceneBundle {
                    scene: entity_models
                        .get(entity_model_name)
                        .expect(&format!("Model not loaded: {}", entity_model_name))
                        .clone(),
                    transform: entity_transform,
                    ..default()
                },
            ));
            insert_entity_tags(&mut entity_commands, &entity_status.entity_type);

        }
    }

    // Spawn the sunlight.
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 30000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
}

#[derive(Component)]
struct GameCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((GameCamera, Camera3dBundle::default()));
}

/**
 Update the camera per frame, making it consistent with the player.
*/
fn update_camera(
    mut query_camera: Query<&mut GlobalTransform, With<GameCamera>>,
    query_main_player: Query<&GlobalTransform, (With<player::MainPlayer>, Without<GameCamera>)>,
) {
    let mut camera_transform = query_camera
        .get_single_mut()
        .expect("Not exactly one camera!");
    // TODO: use two cameras: 1st point of view and third person camera.
    let player_transform = query_main_player
        .get_single()
        .expect("Not exactly one main player!");
    // I think panic here is necessary, because this should never happen. --XHZ
    camera_transform.clone_from(player_transform); // Set the camera transform equal to the player transform
}

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

fn find_model_name_by_type(model_type: &str) -> Option<&str> {
    match model_type {
        "Creeper" => Some("minecraft_creeper.glb"),
        "Player" => Some("minecraft_steve.glb"),
        "MainPlayer" => Some("minecraft_steve.glb"),
        "Torch" => Some("minecraft_torch.glb"),
        _ => None,
    }
}

fn insert_entity_tags(entity_commands: &mut EntityCommands, entity_type: &str) {
    match entity_type {
        "MainPlayer" => entity_commands.insert((entities::Entity, player::Player, player::MainPlayer)),
        "Player" => entity_commands.insert((entities::Entity, player::Player)),
        "Creeper" => entity_commands.insert((entities::Entity, entities::Creeper)), 
        "Torch" => entity_commands.insert((entities::Entity, entities::Torch)), 
        _ => entity_commands
    };
}
