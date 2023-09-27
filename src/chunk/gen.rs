use super::{CHUNK_LEN, HEIGHT, LENGTH, NOISE_FACTOR_CONT, NOISE_FACTOR_SCALE, WIDTH};
use crate::block_reg::{AIR, DIRT, GRASS /* STONE */};
use noise::NoiseFn;

// Generate chunk from noise
pub fn generate_chunk(cords: [i32; 2], noise: &impl NoiseFn<f64, 2>) -> [u16; CHUNK_LEN] {
    let mut height_map: [usize; WIDTH * LENGTH] = [0; WIDTH * LENGTH];
    let mut chunk = [0; CHUNK_LEN];
    // First, generate a height map
    for j in 0..LENGTH {
        for i in 0..WIDTH {
            height_map[i + j * WIDTH] = (HEIGHT as f64 * 1.0 / NOISE_FACTOR_SCALE
                + (noise.get([
                    ((i as i32 + cords[0] * WIDTH as i32) as f64 + 0.5) * NOISE_FACTOR_CONT,
                    ((j as i32 + cords[1] * LENGTH as i32) as f64 + 0.5) * NOISE_FACTOR_CONT,
                ]) * HEIGHT as f64
                    * (1.0 - 1.0 / NOISE_FACTOR_SCALE)))
                as usize;
        }
    }
    // From the height map, assign a value to each block based on wether it is below or above the
    // height level at that position, if it is the exact position of the height, grass block.
    for y in 0..HEIGHT {
        for z in 0..LENGTH {
            for x in 0..WIDTH {
                if height_map[x + z * WIDTH] < y {
                    chunk[x + z * WIDTH + y * WIDTH * LENGTH] = AIR;
                } else if height_map[x + z * WIDTH] == y && y > HEIGHT / 4 {
                    chunk[x + z * WIDTH + y * WIDTH * LENGTH] = GRASS;
                } else {
                    chunk[x + z * WIDTH + y * WIDTH * LENGTH] = DIRT;
                }
            }
        }
    }
    chunk
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn rle_decompress<T: Copy>(compressed: &[(T, usize)]) -> Vec<T> {
    let mut decompressed = Vec::new();
    for &(val, count) in compressed {
        decompressed.extend(std::iter::repeat(val).take(count));
    }
    decompressed
}
