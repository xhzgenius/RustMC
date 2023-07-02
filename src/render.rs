use crate::*;
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
    let block_materials = load_textures(asset_server, materials);
    for &(chunks_x, chunks_z) in game_map.map.keys() {
        for x in 0..gamemap::CHUNK_SIZE {
            for y in 0..gamemap::CHUNK_HEIGHT {
                for z in 0..gamemap::CHUNK_SIZE {
                    let block_id = game_map.map[&(chunks_x, chunks_z)].get((x, y, z)).unwrap();
                    commands.spawn((
                        blocks::Block,
                        PbrBundle {
                            mesh: block_mesh.clone(),
                            material: block_materials.get(*block_id as usize).unwrap().clone(),
                            transform: Transform::from_xyz(
                                (chunks_x * gamemap::CHUNK_SIZE as i32 + x as i32) as f32,
                                y as f32,
                                (chunks_z * gamemap::CHUNK_SIZE as i32 + z as i32) as f32,
                            ),
                            ..default()
                        },
                    ));
                }
            }
        }
    }

    // Prepare model for the entities in the scene.
    // Prepare model for the player.
    commands.spawn((
        player::GamePlayer,
        player::GameMainPlayer,
        entities::Entity,
        PbrBundle {
            transform: Transform::from_xyz(0., gamemap::CHUNK_HEIGHT as f32 + 5., 0.)
                .looking_at(
                    Vec3 {
                        x: 10.,
                        y: gamemap::CHUNK_HEIGHT as f32,
                        z: 10.,
                    },
                    Vec3 {
                        x: 0.,
                        y: 10.,
                        z: 0.,
                    },
                ),
            ..default()
        },
    ));

    // Prepare the sunlight.
    commands.spawn(
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 30000.,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            ..default()
        },
    );
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
    query_player: Query<&GlobalTransform, (With<player::GameMainPlayer>, Without<GameCamera>)>,
) {
    let mut camera_transform = match query_camera.get_single_mut() {
        Ok(result) => result,
        Err(_) => panic!("Not exactly one camera!"),
    };
    let player_transform = match query_player.get_single() {
        Ok(result) => result,
        Err(_) => panic!("Not exactly one player!"),
    };
    camera_transform.clone_from(player_transform); // Set the camera transform equal to the player transform
}

fn load_textures(
    asset_server: Res<AssetServer>,
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
