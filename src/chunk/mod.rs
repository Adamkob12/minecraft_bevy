pub mod chunk_queue;
pub mod gen;
pub mod systems;

pub use chunk_queue::*;
pub use gen::*;
use systems::*;

use crate::{Block, GlobalSecondsCounter};
use bevy::prelude::*;
use bevy_meshem::prelude::{Dimensions, MeshMD};

const CHUNK_SIZE: usize = 16;
pub const CHUNK_DIMS: Dimensions = (CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE);
pub const HEIGHT: usize = CHUNK_DIMS.1;
pub const WIDTH: usize = CHUNK_DIMS.0;
pub const LENGTH: usize = CHUNK_DIMS.2;
pub const CHUNK_LEN: usize = CHUNK_DIMS.0 * CHUNK_DIMS.1 * CHUNK_DIMS.2;
const NOISE_FACTOR_CONT: f64 = 0.020;
// has to be greater than 1.0
pub(crate) const NOISE_FACTOR_SCALE: f64 = 1.8;

#[derive(Component)]
pub struct ToUpdate;

#[derive(Component)]
pub struct Chunk {
    pub meta_data: MeshMD<Block>,
    pub cords: [i32; 2],
    // pub compressed_chunk: Vec<(Block, usize)>,
    pub grid: [Block; CHUNK_LEN],
}

#[derive(States, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub enum InitialChunkLoadState {
    #[default]
    Loading,
    MeshesLoaded,
    Complete,
}

#[derive(Component)]
pub struct ChunkCloseToPlayer;

#[derive(Component)]
pub struct ToCull {
    pub culled: [bool; 6],
}

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        // Systems
        app.add_systems(
            Update,
            (
                spawn_and_despawn_chunks,
                frame_chunk_update,
                (update_closby_chunks).run_if(in_state(InitialChunkLoadState::Complete)),
            ),
        );
        app.add_systems(
            PostUpdate,
            (
                cull_sides_of_mesh.run_if(
                    in_state(InitialChunkLoadState::Complete)
                        .and_then(resource_changed::<GlobalSecondsCounter>()),
                ),
                update_mesh_frame,
            ),
        );

        // Resources
        app.init_resource::<ChunkMap>()
            .init_resource::<ChunkQueue>();

        // States
        app.add_state::<InitialChunkLoadState>();
    }
}
