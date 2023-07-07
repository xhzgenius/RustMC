use crate::*;
use bevy::prelude::*;
use noise::*;
use rand::*;
use rand_distr::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::array;
use std::cmp::min;
use std::f32::consts::PI;
use std::u64::MAX;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// The static variable representing the world's name.
/// At the start of the main menu, it is None.
/// It is initialized when choosing the world.
#[derive(Resource, Default)]
pub struct WorldName {
    pub name: Option<String>,
}

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 16;
/// Number of CHUNKS in a row.
pub const CHUNK_LEN: usize = 6;

/// The whole game map. Contains all blocks and entities.
/// Can be serialized and stored into a file, and deserialized from a file.
///
/// Usage:
/// ```
/// let mut game_map: GameMap = new_gamemap();
/// let chunks_x = 1;
/// let chunks_z = 2;
/// game_map.map.get((chunks_x, chunks_z)) // is a Chunk
/// ```
#[serde_as]
#[derive(Resource, Serialize, Deserialize, Default)]
pub struct GameMap {
    #[serde_as(as = "Vec<(_,_)>")]
    pub map: HashMap<(i32, i32), Chunk>,
}

impl GameMap {
    /// Query a position's chunk that it belongs.
    pub fn query_chunk_by_xyz(&self, xyz: Vec3) -> (i32, i32) {
        let x = xyz[0].floor() as i32;
        let z = xyz[2].floor() as i32;
        let chunk_x = x.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        return (chunk_x, chunk_z);
    }
    /// Query a block according to the coordinates.
    pub fn query_block_by_xyz(&self, xyz: Vec3) -> Option<i32> {
        let x = xyz[0].floor() as i32;
        let y = xyz[1].floor() as usize;
        let z = xyz[2].floor() as i32;
        let chunk_x = x.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        let newx: usize = (x - 16 * chunk_x).try_into().unwrap();
        let newz: usize = (z - 16 * chunk_z).try_into().unwrap();
        if y >= CHUNK_HEIGHT {
            return None;
        }
        return match self.map.get(&(chunk_x, chunk_z)) {
            Some(chunk) => Some(chunk.blocks.lock().unwrap()[newx][y][newz]),
            None => None,
        };
    }
}

type ChunkBlocks = [[[i32; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE];

type ChunkBlocksXZ = [[usize; CHUNK_SIZE]; CHUNK_SIZE];

/// A Chunk is blocks within a 16*height*16 region, with all entities in this region.
/// Both blocks and entities are stored as `Arc<Mutex<...>>`.
#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub blocks: Arc<Mutex<ChunkBlocks>>,
    pub entities: Vec<Arc<Mutex<entities::EntityStatus>>>,
}

/**
Returns a chunk with random height at each position, no entities inside the chunk.
Use Berlin Noise with different freqencies and amplitude to show different terrains.
 */
fn random_chunk(xx: usize, zz: usize, seed1: u32, seed2: u32, seed3: u32) -> Chunk {
    let mut blocks: ChunkBlocks = Default::default();
    let mut height: ChunkBlocksXZ = Default::default();
    //let normal = Normal::new(4.0, 1.0).unwrap();
    let noise1 = Perlin::new(seed1);
    let noise2 = Perlin::new(seed2);
    let noise3 = Perlin::new(seed3);
    let world_size = (CHUNK_LEN * CHUNK_SIZE) as f64;
    // let xx = 0;
    // let zz = 0;

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let noise_1 = CHUNK_HEIGHT as f64 / 2.0
                * noise1.get([
                    (x + xx * CHUNK_SIZE) as f64 / world_size,
                    (z + zz * CHUNK_SIZE) as f64 / world_size,
                ]);
            let noise_2 = CHUNK_HEIGHT as f64 / 4.0
                * noise2.get([
                    ((x + xx * CHUNK_SIZE) as f64 / world_size * 3.0).fract(),
                    ((z + zz * CHUNK_SIZE) as f64 / world_size * 3.0).fract(),
                ]);
            let noise_3 = CHUNK_HEIGHT as f64 / 8.0
                * noise3.get([
                    ((x + xx * CHUNK_SIZE) as f64 / world_size * 7.0).fract(),
                    ((z + zz * CHUNK_SIZE) as f64 / world_size * 7.0).fract(),
                ]);
            height[x][z] = (noise_1 + noise_2 + noise_3) as usize + CHUNK_HEIGHT / 2;
            height[x][z] = usize::max(1, min(CHUNK_HEIGHT, height[x][z]));

            for y in 0..height[x][z] {
                blocks[x][y][z] = 210;
            }
            for y in height[x][z]..CHUNK_HEIGHT {
                blocks[x][y][z] = -1;
            }
        }
    }
    return Chunk {
        blocks: Arc::new(Mutex::new(blocks)),
        entities: vec![],
    };
}

/**
Returns a flat chunk (half stone, half air) with no entities.
*/
fn flat_chunk() -> Chunk {
    let mut blocks: ChunkBlocks = Default::default();
    for y in 0..CHUNK_HEIGHT / 2 {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                blocks[x][y][z] = 210;
            }
        }
    }
    for y in CHUNK_HEIGHT / 2..CHUNK_HEIGHT {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                blocks[x][y][z] = -1;
            }
        }
    }
    return Chunk {
        blocks: Arc::new(Mutex::new(blocks)),
        entities: vec![],
    };
}

/**
A test game map, with 6*6 flat chunks, with some entities in the middle chunk.
*/
pub fn new_gamemap() -> GameMap {
    let mut new_map = HashMap::new();

    let mut range = rand::thread_rng();
    let mut seed1 = range.gen_range(0..u32::MAX);
    let mut seed2 = range.gen_range(0..u32::MAX);
    let mut seed3 = range.gen_range(0..u32::MAX);
    for x in -3..3 {
        for z in -3..3 {
            let (xx, zz) = ((x + 3) as usize, (z + 3) as usize);
            let mut chunk = random_chunk(xx, zz, seed1, seed2, seed3);
            if x == 0 && z == 0 {
                let proper_y: f32 = CHUNK_HEIGHT as f32;
                chunk
                    .entities
                    .push(Arc::new(Mutex::new(entities::EntityStatus {
                        entity_type: "MainPlayer".to_string(),
                        health: 20,
                        position: Vec3::new(0., proper_y, 0.),
                        rotation: PI * 0.,
                        scaling: Vec3::new(1., 1., 1.),
                        velocity: Vec3::new(0., 0., 0.),
                        attack_cd: 0.,
                    })));
                chunk
                    .entities
                    .push(Arc::new(Mutex::new(entities::EntityStatus {
                        entity_type: "Creeper".to_string(),
                        health: 20,
                        position: Vec3::new(5., proper_y, -10.),
                        rotation: PI * 0.,
                        scaling: Vec3::new(1., 1., 1.),
                        velocity: Vec3::new(0., 0., 0.),
                        attack_cd: 0.,
                    })));
                chunk
                    .entities
                    .push(Arc::new(Mutex::new(entities::EntityStatus {
                        entity_type: "Player".to_string(),
                        health: 20,
                        position: Vec3::new(10., proper_y, -10.),
                        rotation: PI * 0.,
                        scaling: Vec3::new(1., 1., 1.),
                        velocity: Vec3::new(0., 0., 0.),
                        attack_cd: 0.,
                    })));
                chunk
                    .entities
                    .push(Arc::new(Mutex::new(entities::EntityStatus {
                        entity_type: "Creeper".to_string(),
                        health: 20,
                        position: Vec3::new(10., proper_y, -8.),
                        rotation: PI * 0.,
                        scaling: Vec3::new(1., 1., 1.),
                        velocity: Vec3::new(0., 0., 0.),
                        attack_cd: 0.,
                    })));
            }
            new_map.insert((x, z), chunk);
        }
    }
    return GameMap { map: new_map };
}

/**
Load a game map from a file.
Returns GameMap if the file is successfully loaded. Otherwise panics.
(The Bevy framework does not support returning a Result here.)
*/
pub fn load_gamemap(world_name: &str) -> GameMap {
    let filename = format!("./saves/{}.json", world_name);
    println!("Loading world from {}", filename);
    match std::fs::read_to_string(filename) {
        Ok(serialized_gamemap) => match serde_json::from_str(&serialized_gamemap) {
            Ok(gamemap) => gamemap,
            Err(err) => panic!("Failed to deserialize map: {}", err),
        },
        Err(err) => panic!("Falied to load map from file: {}", err),
    }
}

/**
Save a game map to a file.
Returns Ok(()) if the map is successfully saved. Otherwise returns the error.
*/
pub fn save_gamemap(
    gamemap: &GameMap,
    world_name: &Res<WorldName>,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("./saves/{}.json", world_name.name.clone().unwrap());
    match serde_json::to_string(&gamemap) {
        Ok(serialized_gamemap) => match std::fs::write(&filename, &serialized_gamemap) {
            Ok(()) => Ok(()),
            Err(err) => Err(Box::new(err)),
        },
        Err(err) => Err(Box::new(err)),
    }
}
