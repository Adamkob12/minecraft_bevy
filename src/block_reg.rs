use bevy::prelude::*;
use bevy::render::mesh::{Mesh, MeshVertexAttribute};
use bevy_meshem::prelude::*;

pub type Block = u16;

pub const AIR: Block = 0;
pub const DIRT: Block = 1;
pub const GRASS: Block = 2;
pub const STONE: Block = 3;
pub const LIGHT_MAGIC: Block = 4;
pub const DARK_MAGIC: Block = 5;
pub const TRANSPERENT: Block = 6;
pub const WOOD_DARK_GREY: Block = 7;
pub const PINK_LEAVES: Block = 8;

pub const VOXEL_DIMS: [f32; 3] = [1.0, 1.0, 1.0];

#[derive(Resource, Clone)]
pub struct BlockRegistry {
    grass_block: Mesh,
    dirt_block: Mesh,
    stone_block: Mesh,
    light_magic_block: Mesh,
    dark_magic_block: Mesh,
    transperent_block: Mesh,
    wood_dark_grey_block: Mesh,
    pink_leaves: Mesh,
}

impl Default for BlockRegistry {
    fn default() -> Self {
        BlockRegistry {
            grass_block: generate_voxel_mesh(
                VOXEL_DIMS,
                [4, 4],
                [
                    (Top, [0, 0]),
                    (Bottom, [2, 0]),
                    (Right, [1, 0]),
                    (Left, [1, 0]),
                    (Forward, [1, 0]),
                    (Back, [1, 0]),
                ],
                0.02,
            ),
            dirt_block: generate_voxel_mesh(
                VOXEL_DIMS,
                [4, 4],
                [
                    (Top, [2, 0]),
                    (Bottom, [2, 0]),
                    (Right, [2, 0]),
                    (Left, [2, 0]),
                    (Forward, [2, 0]),
                    (Back, [2, 0]),
                ],
                0.02,
            ),
            stone_block: generate_voxel_mesh(
                VOXEL_DIMS,
                [4, 4],
                [
                    (Top, [3, 0]),
                    (Bottom, [3, 0]),
                    (Right, [3, 0]),
                    (Left, [3, 0]),
                    (Forward, [3, 0]),
                    (Back, [3, 0]),
                ],
                0.02,
            ),
            light_magic_block: generate_voxel_mesh(
                VOXEL_DIMS,
                [4, 4],
                [
                    (Top, [2, 1]),
                    (Bottom, [2, 1]),
                    (Right, [2, 1]),
                    (Left, [2, 1]),
                    (Forward, [2, 1]),
                    (Back, [2, 1]),
                ],
                0.02,
            ),
            dark_magic_block: generate_voxel_mesh(
                VOXEL_DIMS,
                [4, 4],
                [
                    (Top, [1, 1]),
                    (Bottom, [1, 1]),
                    (Right, [1, 1]),
                    (Left, [1, 1]),
                    (Forward, [1, 1]),
                    (Back, [1, 1]),
                ],
                0.02,
            ),
            transperent_block: generate_voxel_mesh(
                VOXEL_DIMS,
                [4, 4],
                [
                    (Top, [3, 1]),
                    (Bottom, [3, 1]),
                    (Right, [3, 1]),
                    (Left, [3, 1]),
                    (Forward, [3, 1]),
                    (Back, [3, 1]),
                ],
                0.02,
            ),
            wood_dark_grey_block: generate_voxel_mesh(
                VOXEL_DIMS,
                [4, 4],
                [
                    (Top, [0, 2]),
                    (Bottom, [0, 2]),
                    (Right, [0, 2]),
                    (Left, [0, 2]),
                    (Forward, [0, 2]),
                    (Back, [0, 2]),
                ],
                0.02,
            ),
            pink_leaves: generate_voxel_mesh(
                VOXEL_DIMS,
                [4, 4],
                [
                    (Top, [1, 2]),
                    (Bottom, [1, 2]),
                    (Right, [1, 2]),
                    (Left, [1, 2]),
                    (Forward, [1, 2]),
                    (Back, [1, 2]),
                ],
                0.02,
            ),
        }
    }
}

impl VoxelRegistry for BlockRegistry {
    type Voxel = Block;

    fn all_attributes(&self) -> Vec<MeshVertexAttribute> {
        vec![
            Mesh::ATTRIBUTE_POSITION,
            Mesh::ATTRIBUTE_UV_0,
            Mesh::ATTRIBUTE_NORMAL,
        ]
    }

    fn get_voxel_dimensions(&self) -> [f32; 3] {
        VOXEL_DIMS
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
            LIGHT_MAGIC => Some(&self.light_magic_block),
            DARK_MAGIC => Some(&self.dark_magic_block),
            TRANSPERENT => Some(&self.transperent_block),
            WOOD_DARK_GREY => Some(&self.wood_dark_grey_block),
            PINK_LEAVES => Some(&self.pink_leaves),
            _ => None,
        }
    }
}
