use bevy::prelude::Vec3;

pub fn position_to_chunk(pos: Vec3, chunk_dims: (usize, usize, usize)) -> [i32; 2] {
    let chunk_width = chunk_dims.0;
    let chunk_length = chunk_dims.1;
    let x = pos.x + 0.5;
    let z = pos.z + 0.5;
    [
        (x / chunk_width as f32 + (x.signum() - 1.0) / 2.0) as i32,
        (z / chunk_length as f32 + (z.signum() - 1.0) / 2.0) as i32,
    ]
}

// the bool is for whether or not the pos is within the height bounds
pub fn position_to_chunk_position(
    pos: Vec3,
    chunk_dims: (usize, usize, usize),
) -> ([i32; 2], [usize; 3], bool) {
    let chunk_width = chunk_dims.0;
    let chunk_length = chunk_dims.2;
    let chunk_height = chunk_dims.1;
    let chunk = position_to_chunk(pos, chunk_dims);

    let x = pos.x + 0.5;
    let z = pos.z + 0.5;
    let y = pos.y + 0.5;

    let chunk_pos = [
        (x - chunk[0] as f32 * chunk_width as f32) as usize,
        y as usize,
        (z - chunk[1] as f32 * chunk_length as f32) as usize,
    ];

    let flag = y >= 0.0 && y <= chunk_height as f32;
    (chunk, chunk_pos, flag)
}

pub const fn three_d_cords(oned: usize, dims: (usize, usize, usize)) -> (usize, usize, usize) {
    let height = dims.2;
    let length = dims.1;
    let width = dims.0;

    let h = (oned / (length * width)) as usize;
    let l = ((oned - h * (length * width)) / width) as usize;
    let w = (oned - h * (length * width) - l * width) as usize;

    assert!(w < width, "Out of bounds to convert into 3d coordinate.");
    assert!(h < height, "Out of bounds to convert into 3d coordinate.");
    assert!(l < length, "Out of bounds to convert into 3d coordinate.");

    (w, h, l)
}

pub const fn one_d_cords(threed: [usize; 3], dims: (usize, usize, usize)) -> usize {
    assert!(threed[0] < dims.0, "3d coordinate out of dimension bounds.");
    assert!(threed[1] < dims.1, "3d coordinate out of dimension bounds.");
    assert!(threed[2] < dims.2, "3d coordinate out of dimension bounds.");
    threed[1] * (dims.0 * dims.2) + threed[2] * dims.0 + threed[0]
}

// Extract the vertex data for the physics engine.
use bevy::render::mesh::{Mesh, VertexAttributeValues};
pub fn extract_position_vertex_data(mesh: &Mesh) -> Vec<Vec3> {
    let VertexAttributeValues::Float32x3(pos_vertices) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() else {
        panic!("Vertex position data should be in `VertexAttributeValues::Float32x3`")
    };
    pos_vertices
        .iter()
        .map(|[x, y, z]| Vec3::new(*x, *y, *z))
        .collect()
}

// Extract the indices for the physics engine.
use bevy::render::mesh::Indices;
pub fn extract_indices_data(mesh: &Mesh) -> Vec<[u32; 3]> {
    let Indices::U32(indices) = mesh.indices().unwrap() else {
        panic!("Indices data shoud be in `Indices::U32` format")
    };
    indices
        .chunks(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect()
}
