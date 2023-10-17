use bevy::prelude::*;
use bevy::render::mesh::{Mesh, MeshVertexAttribute};
use bevy_meshem::prelude::*;

pub type Block = u16;

pub const AIR: Block = 0;
pub const DIRT: Block = 1;
pub const GRASS: Block = 2;
pub const STONE: Block = 3;
pub const BRICKS: Block = 4;
pub const LOG: Block = 5;
pub const WOOD: Block = 6;
pub const LEAVES: Block = 7;
pub const GLASS: Block = 8;
pub const GLOWSTONE: Block = 9;

pub const VOXEL_DIMS: [f32; 3] = [1.0, 1.0, 1.0];

const ATLAS_CORDS: [u32; 2] = [24, 24];
const PADDING: f32 = 0.0625;

#[derive(Resource, Clone)]
pub struct BlockRegistry {
    grass_block: Mesh,
    dirt_block: Mesh,
    stone_block: Mesh,
    bricks_block: Mesh,
    log_block: Mesh,
    wood_block: Mesh,
    leaves_block: Mesh,
    glass_block: Mesh,
    glowstone_block: Mesh,
}

impl Default for BlockRegistry {
    fn default() -> Self {
        BlockRegistry {
            grass_block: generate_voxel_mesh(
                VOXEL_DIMS,
                ATLAS_CORDS,
                [
                    (Top, [0, 0]),
                    (Bottom, [2, 0]),
                    (Right, [1, 0]),
                    (Left, [1, 0]),
                    (Forward, [1, 0]),
                    (Back, [1, 0]),
                ],
                PADDING,
                Some(0.75),
            ),
            dirt_block: generate_voxel_mesh(
                VOXEL_DIMS,
                ATLAS_CORDS,
                [
                    (Top, [2, 0]),
                    (Bottom, [2, 0]),
                    (Right, [2, 0]),
                    (Left, [2, 0]),
                    (Forward, [2, 0]),
                    (Back, [2, 0]),
                ],
                PADDING,
                Some(0.75),
            ),
            stone_block: generate_voxel_mesh(
                VOXEL_DIMS,
                ATLAS_CORDS,
                [
                    (Top, [3, 0]),
                    (Bottom, [3, 0]),
                    (Right, [3, 0]),
                    (Left, [3, 0]),
                    (Forward, [3, 0]),
                    (Back, [3, 0]),
                ],
                PADDING,
                Some(0.75),
            ),
            bricks_block: generate_voxel_mesh(
                VOXEL_DIMS,
                ATLAS_CORDS,
                [
                    (Top, [4, 0]),
                    (Bottom, [4, 0]),
                    (Right, [4, 0]),
                    (Left, [4, 0]),
                    (Forward, [4, 0]),
                    (Back, [4, 0]),
                ],
                PADDING,
                Some(0.75),
            ),
            log_block: generate_voxel_mesh(
                VOXEL_DIMS,
                ATLAS_CORDS,
                [
                    (Top, [5, 0]),
                    (Bottom, [5, 0]),
                    (Right, [6, 0]),
                    (Left, [6, 0]),
                    (Forward, [6, 0]),
                    (Back, [6, 0]),
                ],
                PADDING,
                Some(0.90),
            ),
            wood_block: generate_voxel_mesh(
                VOXEL_DIMS,
                ATLAS_CORDS,
                [
                    (Top, [7, 0]),
                    (Bottom, [7, 0]),
                    (Right, [7, 0]),
                    (Left, [7, 0]),
                    (Forward, [7, 0]),
                    (Back, [7, 0]),
                ],
                PADDING,
                Some(0.75),
            ),
            leaves_block: generate_voxel_mesh(
                VOXEL_DIMS,
                ATLAS_CORDS,
                [
                    (Top, [8, 0]),
                    (Bottom, [8, 0]),
                    (Right, [8, 0]),
                    (Left, [8, 0]),
                    (Forward, [8, 0]),
                    (Back, [8, 0]),
                ],
                PADDING,
                Some(0.75),
            ),
            glass_block: generate_voxel_mesh(
                VOXEL_DIMS,
                ATLAS_CORDS,
                [
                    (Top, [9, 0]),
                    (Bottom, [9, 0]),
                    (Right, [9, 0]),
                    (Left, [9, 0]),
                    (Forward, [9, 0]),
                    (Back, [9, 0]),
                ],
                PADDING,
                Some(0.75),
            ),
            glowstone_block: generate_voxel_mesh(
                VOXEL_DIMS,
                ATLAS_CORDS,
                [
                    (Top, [10, 0]),
                    (Bottom, [10, 0]),
                    (Right, [10, 0]),
                    (Left, [10, 0]),
                    (Forward, [10, 0]),
                    (Back, [10, 0]),
                ],
                PADDING,
                Some(0.75),
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
            Mesh::ATTRIBUTE_COLOR,
        ]
    }

    fn get_voxel_dimensions(&self) -> [f32; 3] {
        VOXEL_DIMS
    }

    fn get_center(&self) -> [f32; 3] {
        [0.0, 0.0, 0.0]
    }

    fn is_covering(&self, voxel: &Self::Voxel, _side: prelude::Face) -> bool {
        *voxel != AIR && *voxel != LEAVES && *voxel != GLASS
    }

    fn get_mesh(&self, voxel: &Self::Voxel) -> VoxelMesh<&Mesh> {
        match *voxel {
            AIR => VoxelMesh::Null,
            DIRT => VoxelMesh::NormalCube(&self.dirt_block),
            GRASS => VoxelMesh::NormalCube(&self.grass_block),
            STONE => VoxelMesh::NormalCube(&self.stone_block),
            BRICKS => VoxelMesh::NormalCube(&self.bricks_block),
            LOG => VoxelMesh::NormalCube(&self.log_block),
            WOOD => VoxelMesh::NormalCube(&self.wood_block),
            LEAVES => VoxelMesh::NormalCube(&self.leaves_block),
            GLASS => VoxelMesh::NormalCube(&self.glass_block),
            GLOWSTONE => VoxelMesh::NormalCube(&self.glowstone_block),
            _ => VoxelMesh::Null,
        }
    }
}
