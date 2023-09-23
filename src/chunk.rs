use crate::block_reg::{Block, AIR, DIRT, GRASS, STONE};
use bevy::prelude::Component;
use bevy_meshem::prelude::{Dimensions, MeshMD};
use noise::{NoiseFn, Perlin, Seedable};
pub const CHUNK_DIMS: Dimensions = (32, 32, 32);
pub const CHUNK_LEN: usize = CHUNK_DIMS.0 * CHUNK_DIMS.1 * CHUNK_DIMS.2;
const NOISE_FACTOR_CONT: f64 = 0.01;
// has to be greater than 1.0
const NOISE_FACTOR_SCALE: f64 = 2.0;

#[derive(Component)]
pub struct Chunk {
    pub meta_data: MeshMD<Block>,
    pub cords: [i32; 2],
    pub compressed_chunk: Vec<(Block, usize)>,
}

pub fn generate_chunk(cords: [i32; 2], noise: &impl NoiseFn<f64, 2>) -> [u16; CHUNK_LEN] {
    let mut height_map: [usize; CHUNK_DIMS.0 * CHUNK_DIMS.1] = [0; CHUNK_DIMS.0 * CHUNK_DIMS.1];
    let mut chunk = [0; CHUNK_LEN];
    for j in 0..CHUNK_DIMS.1 {
        for i in 0..CHUNK_DIMS.0 {
            height_map[i + j * CHUNK_DIMS.0] = (CHUNK_DIMS.2 as f64 * 1.0 / NOISE_FACTOR_SCALE
                + (noise.get([
                    ((i as i32 + cords[0] * CHUNK_DIMS.0 as i32) as f64 + 0.5) * NOISE_FACTOR_CONT,
                    ((j as i32 + cords[1] * CHUNK_DIMS.1 as i32) as f64 + 0.5) * NOISE_FACTOR_CONT,
                ]) * CHUNK_DIMS.2 as f64
                    * (1.0 - 1.0 / NOISE_FACTOR_SCALE)))
                as usize;
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

pub fn rle_compress<T: PartialEq + Copy>(data: &[T]) -> Vec<(T, usize)> {
    let mut compressed = Vec::new();
    let mut iter = data.iter();
    if let Some(mut run_val) = iter.next().copied() {
        let mut run_len = 1;
        for &val in iter {
            if val == run_val {
                run_len += 1;
            } else {
                compressed.push((run_val, run_len));
                run_val = val;
                run_len = 1;
            }
        }
        compressed.push((run_val, run_len));
    }
    compressed
}

pub fn rle_decompress<T: Copy>(compressed: &[(T, usize)]) -> Vec<T> {
    let mut decompressed = Vec::new();
    for &(val, count) in compressed {
        decompressed.extend(std::iter::repeat(val).take(count));
    }
    decompressed
}
