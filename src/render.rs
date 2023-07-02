use crate::*;
use bevy::gltf::Gltf;
use bevy::prelude::*;

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
 The player's starting status. Can be configured.
 TODO: Save the player's status in the save.
*/
fn player_starting_status() -> entities::EntityStatus {
    return entities::EntityStatus {
        health: 20,
        velocity: Vec3::new(0., 0., 0.),
    };
}

/**
Spawn all gltf(*.glb) entities.
*/
fn load_gltf_object(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let path_creeper = "models/minecraft_creeper.glb#Scene0";
    let path_steve = "models/minecraft_steve.glb#Scene0";
    let path_torch = "models/minecraft_torch.glb#Scene0";
    let transform = Transform::from_xyz(10., gamemap::CHUNK_HEIGHT as f32 / 2. + 1., 5.)
        .with_scale(Vec3::new(0.1, 0.1, 0.1));
    let transform2 = Transform::from_xyz(5., gamemap::CHUNK_HEIGHT as f32 / 2. + 1., 10.)
        .with_scale(Vec3::new(0.1, 0.1, 0.1));
    let transform3 = Transform::from_xyz(5., gamemap::CHUNK_HEIGHT as f32 / 2., 5.)
        .with_scale(Vec3::new(0.5, 0.5, 0.5));
    let gltf_creeper: Handle<Scene> = asset_server.load(path_creeper);
    let gltf_steve: Handle<Scene> = asset_server.load(path_steve);
    let gltf_torch: Handle<Scene> = asset_server.load(path_torch);
    // spawn the first scene in the file
    commands.spawn(SceneBundle {
        scene: gltf_creeper,
        transform: transform.clone(),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: gltf_steve,
        transform: transform2.clone(),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: gltf_torch,
        transform: transform3.clone(),
        ..default()
    });
}

/**
 Initialize the whole scene in the game, in other words, load all blocks and entities.
*/
fn init_blocks_and_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_map: Res<gamemap::GameMap>,
) {
    // Prepare model for a block.
    let block_mesh = meshes.add(shape::Cube { size: 1.0 }.into());
    // Prepare material for every kind of blocks.
    let block_materials = load_block_textures(&asset_server, materials);
    // Spawn all blocks in the gamemap.
    for &(chunks_x, chunks_z) in game_map.map.keys() {
        for x in 0..gamemap::CHUNK_SIZE {
            for y in 0..gamemap::CHUNK_HEIGHT {
                for z in 0..gamemap::CHUNK_SIZE {
                    let block_id = game_map.map[&(chunks_x, chunks_z)].get((x, y, z)).unwrap();
                    if let Some(block_material) = block_materials.get(*block_id as usize) {
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
    // Spawn the player.
    commands.spawn((
        player::GamePlayer,
        player::GameMainPlayer,
        entities::Entity,
        player_starting_status(),
        PbrBundle {
            transform: player_starting_transform(),
            ..default()
        },
    ));
    // Try to spawn something in gltf
    load_gltf_object(&mut commands, &asset_server);

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
    query_main_player: Query<&GlobalTransform, (With<player::GameMainPlayer>, Without<GameCamera>)>,
) {
    let mut camera_transform = query_camera
        .get_single_mut()
        .expect("Not exactly one camera!");
    let player_transform = query_main_player
        .get_single()
        .expect("Not exactly one player!");
    camera_transform.clone_from(player_transform); // Set the camera transform equal to the player transform
}

/**
 Load textures of every kind of block.
*/
fn load_block_textures(
    asset_server: &Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Vec<Handle<StandardMaterial>> {
    let block_textures = asset_server.load_folder("blocks").unwrap();
    let mut block_materials: Vec<Handle<StandardMaterial>> = Vec::new();
    for block_texture in block_textures {
        block_materials.push(materials.add(StandardMaterial {
            base_color_texture: Some(block_texture.typed()),
            ..default()
        }));
    }
    return block_materials;
}
