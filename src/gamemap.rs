use crate::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 8;

#[derive(Resource, Serialize, Deserialize)]
pub struct GameMap {
    pub map: HashMap<(i32, i32), Chunk>,
}

type ChunkBlocks = [[[i32; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE];

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
                let proper_y: f32 = CHUNK_HEIGHT as f32 / 2. + 2.;
                chunk
                    .entities
                    .push(Arc::new(Mutex::new(entities::EntityStatus {
                        entity_type: "MainPlayer".to_string(),
                        health: 20,
                        position: Vec3::new(0., proper_y, 0.),
                        rotation: 0.,
                        scaling: Vec3::new(0.1, 0.1, 0.1),
                        velocity: Vec3::new(0., 0., 0.),
                    })));
                chunk
                    .entities
                    .push(Arc::new(Mutex::new(entities::EntityStatus {
                        entity_type: "Creeper".to_string(),
                        health: 20,
                        position: Vec3::new(5., proper_y, 10.),
                        rotation: 0.,
                        scaling: Vec3::new(0.1, 0.1, 0.1),
                        velocity: Vec3::new(0., 0., 0.),
                    })));
                chunk
                    .entities
                    .push(Arc::new(Mutex::new(entities::EntityStatus {
                        entity_type: "Player".to_string(),
                        health: 20,
                        position: Vec3::new(10., proper_y, 10.),
                        rotation: 0.,
                        scaling: Vec3::new(0.1, 0.1, 0.1),
                        velocity: Vec3::new(0., 0., 0.),
                    })));
                chunk
                    .entities
                    .push(Arc::new(Mutex::new(entities::EntityStatus {
                        entity_type: "Creeper".to_string(),
                        health: 20,
                        position: Vec3::new(10., proper_y, 8.),
                        rotation: 0.,
                        scaling: Vec3::new(0.1, 0.1, 0.1),
                        velocity: Vec3::new(0., 0., 0.),
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
pub fn load_gamemap(filename: &str) -> GameMap {
    match std::fs::read_to_string(filename) {
        Ok(serialized_gamemap) => match serde_json::from_str(&serialized_gamemap) {
            Ok(gamemap) => gamemap,
            Err(err) => panic!("Failed to deserialize map: {}", err),
        },
        Err(err) => panic!("{}", err),
    }
}

/**
 Save a game map to a file.
 Returns Ok(()) if the map is successfully saved. Otherwise returns the error.
*/
pub fn save_gamemap(gamemap: &GameMap, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    match serde_json::to_string(&gamemap) {
        Ok(serialized_gamemap) => match std::fs::write(&filename, &serialized_gamemap) {
            Ok(()) => Ok(()),
            Err(err) => Err(Box::new(err)),
        },
        Err(err) => Err(Box::new(err)),
    }
}
