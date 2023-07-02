use bevy::prelude::*;
use ndarray::Array3;
use std::collections::HashMap;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 32;

#[derive(Resource)]
pub struct GameMap {
    pub map: HashMap<(i32, i32), Array3<i32>>,
}

/**
 Returns a zero chunk (full of zeros). 
 */
fn zero_chunk() -> Array3<i32> {
    return Array3::zeros((CHUNK_SIZE, CHUNK_HEIGHT, CHUNK_SIZE));
}

/**
 Returns a flat chunk (half grass, half air). 
 */
fn flat_chunk() -> Array3<i32> {
    let mut chunk: Array3<i32> = Array3::zeros((CHUNK_SIZE, CHUNK_HEIGHT, CHUNK_SIZE));
    chunk.fill(210);
    for y in CHUNK_HEIGHT/2..CHUNK_HEIGHT {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                *chunk.get_mut((x, y, z)).unwrap() = -1;
            }
        }
    }
    return chunk;
}

fn test_gamemap() -> GameMap {
    let mut new_map = HashMap::new();
    for x in -3..3 {
        for z in -3..3 {
            new_map.insert((x, z), flat_chunk());
        }
    }
    return GameMap {
        map: new_map
    };
}

/**
 Load a game map from a file. 
 The game map is a 5-D array with axis (chunk_x, chunk_z, x, y, z). 
 */
pub fn load_gamemap(filename: &str) -> GameMap {
    return test_gamemap(); //TODO: implement load_gamemap
}
