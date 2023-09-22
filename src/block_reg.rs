use bevy::prelude::*;
use bevy::render::mesh::{Mesh, MeshVertexAttribute};
use bevy_meshem::prelude::*;

pub const AIR: u16 = 0;
pub const DIRT: u16 = 1;
pub const GRASS: u16 = 2;
pub const STONE: u16 = 3;

#[derive(Resource)]
pub struct BlockRegistry {
    grass_block: Mesh,
    dirt_block: Mesh,
    stone_block: Mesh,
}

impl Default for BlockRegistry {
    fn default() -> Self {
        BlockRegistry {
            grass_block: generate_voxel_mesh(
                [1.0, 1.0, 1.0],
                [4, 4],
                [
                    (Top, [0, 0]),
                    (Bottom, [2, 0]),
                    (Right, [1, 0]),
                    (Left, [1, 0]),
                    (Forward, [1, 0]),
                    (Back, [1, 0]),
                ],
            ),
            dirt_block: generate_voxel_mesh(
                [1.0, 1.0, 1.0],
                [4, 4],
                [
                    (Top, [2, 0]),
                    (Bottom, [2, 0]),
                    (Right, [2, 0]),
                    (Left, [2, 0]),
                    (Forward, [2, 0]),
                    (Back, [2, 0]),
                ],
            ),
            stone_block: generate_voxel_mesh(
                [1.0, 1.0, 1.0],
                [4, 4],
                [
                    (Top, [3, 0]),
                    (Bottom, [3, 0]),
                    (Right, [3, 0]),
                    (Left, [3, 0]),
                    (Forward, [3, 0]),
                    (Back, [3, 0]),
                ],
            ),
        }
    }
}

impl VoxelRegistry for BlockRegistry {
    type Voxel = u16;

    fn all_attributes(&self) -> Vec<MeshVertexAttribute> {
        vec![
            Mesh::ATTRIBUTE_POSITION,
            Mesh::ATTRIBUTE_UV_0,
            Mesh::ATTRIBUTE_NORMAL,
        ]
    }

    fn get_voxel_dimensions(&self) -> [f32; 3] {
        [1.0, 1.0, 1.0]
    }

    fn get_center(&self) -> [f32; 3] {
        [0.0, 0.0, 0.0]
    }

    fn is_voxel(&self, voxel: &Self::Voxel) -> bool {
        *voxel != AIR
    }

    fn get_mesh(&self, voxel: &Self::Voxel) -> Option<&Mesh> {
        match *voxel {
            AIR => None,
            DIRT => Some(&self.dirt_block),
            GRASS => Some(&self.grass_block),
            STONE => Some(&self.stone_block),
            _ => None,
        }
    }
}
