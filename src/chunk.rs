use crate::block_reg::{AIR, DIRT, GRASS, STONE};
use bevy_meshem::Dimensions;
use noise::{NoiseFn, Perlin, Seedable};
pub const CHUNK_DIMS: Dimensions = (32, 32, 32);
pub const CHUNK_LEN: usize = CHUNK_DIMS.0 * CHUNK_DIMS.1 * CHUNK_DIMS.2;
const NOISE_FACTOR: f64 = 0.01;

pub fn generate_chunk(cords: [usize; 2], noise: impl NoiseFn<f64, 2>) -> [u16; CHUNK_LEN] {
    let mut height_map: [usize; CHUNK_DIMS.0 * CHUNK_DIMS.1] = [0; CHUNK_DIMS.0 * CHUNK_DIMS.1];
    let mut chunk = [0; CHUNK_LEN];
    for j in 0..CHUNK_DIMS.1 {
        for i in 0..CHUNK_DIMS.0 {
            height_map[i + j * CHUNK_DIMS.0] = CHUNK_DIMS.2 / 2
                + (noise.get([
                    ((i + cords[0]) as f64 + 0.5) * NOISE_FACTOR,
                    ((j + cords[1]) as f64 + 0.5) * NOISE_FACTOR,
                ]) * CHUNK_DIMS.2 as f64
                    * 0.5) as usize;
        }
    }
    for y in 0..CHUNK_DIMS.2 {
        for z in 0..CHUNK_DIMS.1 {
            for x in 0..CHUNK_DIMS.0 {
                if height_map[x + z * CHUNK_DIMS.0] < y {
                    chunk[x + z * CHUNK_DIMS.0 + y * CHUNK_DIMS.0 * CHUNK_DIMS.1] = AIR;
                } else if height_map[x + z * CHUNK_DIMS.0] == y {
                    chunk[x + z * CHUNK_DIMS.0 + y * CHUNK_DIMS.0 * CHUNK_DIMS.1] = GRASS;
                } else {
                    chunk[x + z * CHUNK_DIMS.0 + y * CHUNK_DIMS.0 * CHUNK_DIMS.1] = DIRT;
                }
            }
        }
    }
    chunk
}
