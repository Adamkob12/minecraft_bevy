use bevy::prelude::Vec3;
pub fn position_to_chunk(pos: Vec3, chunk_dims: (usize, usize, usize)) -> [i32; 2] {
    let chunk_width = chunk_dims.0;
    let chunk_length = chunk_dims.1;
    [
        (pos.x / chunk_width as f32 + (pos.x.signum() - 1.0) / 2.0) as i32,
        (pos.z / chunk_length as f32 + (pos.z.signum() - 1.0) / 2.0) as i32,
    ]
}

// the bool is for whether or not the pos is within the height bounds
pub fn position_to_chunk_position(
    pos: Vec3,
    chunk_dims: (usize, usize, usize),
) -> ([i32; 2], [usize; 3], bool) {
    let chunk_width = chunk_dims.0;
    let chunk_length = chunk_dims.1;
    let chunk_height = chunk_dims.2;
    let chunk = position_to_chunk(pos, chunk_dims);

    let chunk_pos = [
        (pos.x - chunk[0] as f32 * chunk_width as f32) as usize,
        pos.y as usize,
        (pos.z - chunk[1] as f32 * chunk_length as f32) as usize,
    ];

    let flag = pos.y >= 0.0 && pos.y <= chunk_height as f32;
    (chunk, chunk_pos, flag)
}

pub fn three_d_cords(oned: usize, dims: (usize, usize, usize)) -> (usize, usize, usize) {
    let height = dims.2;
    let length = dims.1;
    let width = dims.0;

    let h = (oned / (length * width)) as usize;
    let l = ((oned - h * (length * width)) / width) as usize;
    let w = (oned - h * (length * width) - l * width) as usize;

    assert!(w < width, "Out of bounds to convert into 3d coordinate.");
    assert!(h < height, "Out of bounds to convert into 3d coordinate.");
    assert!(l < length, "Out of bounds to convert into 3d coordinate.");

    (w, l, h)
}

pub fn one_d_cords(threed: [usize; 3], dims: (usize, usize, usize)) -> usize {
    assert!(threed[0] < dims.0, "3d coordinate out of dimension bounds.");
    assert!(threed[1] < dims.1, "3d coordinate out of dimension bounds.");
    assert!(threed[2] < dims.2, "3d coordinate out of dimension bounds.");
    threed[2] * (dims.0 * dims.1) + threed[1] * dims.0 + threed[0]
}
