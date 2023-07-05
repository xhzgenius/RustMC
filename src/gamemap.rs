use crate::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::f32::consts::PI;
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
pub const CHUNK_HEIGHT: usize = 8;

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
    /// Query a block according to the coordinates.
    pub fn query_block_by_xyz(&self, xyz: Vec3) -> Option<i32> {
        let x = xyz[0] as i32;
        let y = xyz[1] as usize;
        let z = xyz[2] as i32;
        let chunk_x = x / 16;
        let chunk_z = z / 16;
        let newx: usize = ((x % 16 + 16) % 16).try_into().unwrap();
        let newz: usize = ((z % 16 + 16) % 16).try_into().unwrap();
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

/// A Chunk is blocks within a 16*height*16 region, with all entities in this region.
/// Both blocks and entities are stored as `Arc<Mutex<...>>`.
#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub blocks: Arc<Mutex<ChunkBlocks>>,
    pub entities: Vec<Arc<Mutex<entities::EntityStatus>>>,
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
    for x in -3..3 {
        for z in -3..3 {
            let mut chunk = flat_chunk();
            if x == 0 && z == 0 {
                let proper_y: f32 = CHUNK_HEIGHT as f32 / 2.;
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
