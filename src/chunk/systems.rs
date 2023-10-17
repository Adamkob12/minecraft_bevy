use super::ToCull;
use crate::{
    block_reg::BlockRegistry, chunk_queue::*, iter_faces_of_chunk, update_mesh, Arc, Chunk,
    ChunkCloseToPlayer, CurrentChunk, Face, Face::*, ToUpdate, VoxelRegistry, LENGTH,
    RENDER_DISTANCE, WIDTH,
};
use bevy::prelude::*;
use bevy_meshem::prelude::VoxelChange;

// Each frame, dequeue all the pending chunks to despawn / spawn onto
// the thread pool, after they are done, they will be picked up by `handle_tasks`.
pub(crate) fn frame_chunk_update(
    mut cq: ResMut<ChunkQueue>,
    cm: ResMut<ChunkMap>,
    breg: Res<BlockRegistry>,
    commands: Commands,
) {
    cq.dequeue_all(
        commands,
        Arc::new(breg.into_inner().clone()),
        cm.into_inner(),
    );
}

pub(crate) fn spawn_and_despawn_chunks(
    q2: Query<(&CurrentChunk, &Transform), Changed<CurrentChunk>>,
    mut chunk_queue: ResMut<ChunkQueue>,
    chunk_map: Res<ChunkMap>,
) {
    for (j, _) in q2.iter() {
        let cords = j.0;
        for chunk in chunk_map.iter_keys() {
            if (chunk[0] - cords[0]).abs() > RENDER_DISTANCE {
                chunk_queue.queue_despawn(*chunk);
            }
            if (chunk[1] - cords[1]).abs() > RENDER_DISTANCE {
                chunk_queue.queue_despawn(*chunk);
            }
        }
        for u in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for v in -RENDER_DISTANCE..=RENDER_DISTANCE {
                chunk_queue.queue_spawn([cords[0] + u, cords[1] + v]);
            }
        }
    }
}

// We need to keep track of the chunks that are close to the player,
// so we dont need to iterate over all the chunks when breaking / placing blocks.
pub(crate) fn update_closby_chunks(
    current_chunk: Query<&CurrentChunk, Changed<CurrentChunk>>,
    chunk_map: Res<ChunkMap>,
    mut commands: Commands,
    old_close_chunks: Query<Entity, With<ChunkCloseToPlayer>>,
) {
    if let Ok(cords) = current_chunk.get_single() {
        let cords = cords.0;
        for chunk in old_close_chunks.iter() {
            commands.entity(chunk).remove::<ChunkCloseToPlayer>();
        }

        for i in -1..=1 {
            for j in -1..=1 {
                commands
                    .entity(
                        chunk_map
                            .get_ent([cords[0] + i, cords[1] + j])
                            .expect("Chunk that was supposed to be loaded was not."),
                    )
                    .insert(ChunkCloseToPlayer);
            }
        }
    }
}

// Update the mesh of the chunks every frame.
pub(crate) fn update_mesh_frame(
    mut query: Query<(Entity, &Handle<Mesh>, &mut Chunk), With<ToUpdate>>,
    mut meshes: ResMut<Assets<Mesh>>,
    breg: Res<BlockRegistry>,
    mut commands: Commands,
) {
    let breg = Arc::new(breg.into_inner().clone());
    for (ent, mesh_handle, mut chunk) in query.iter_mut() {
        let mesh_ref_mut = meshes
            .get_mut(mesh_handle)
            .expect("Can't find chunk mesh in internal assets");
        update_mesh(mesh_ref_mut, &mut chunk.meta_data, &*breg.clone());
        if let Some(aabb) = mesh_ref_mut.compute_aabb() {
            if let Some(mut comm) = commands.get_entity(ent) {
                comm.insert(aabb).remove::<ToUpdate>();
            }
        } else {
            warn!("Couldn't compute Aabb for mesh after updating");
        }
    }
}

//
pub(crate) fn cull_sides_of_mesh(
    chunks_to_cull: Query<(Entity, &ToCull), With<Chunk>>,
    mut chunks_query: Query<&mut Chunk>,
    breg: Res<BlockRegistry>,
    mut commands: Commands,
    chunk_map: Res<ChunkMap>,
) {
    let breg = breg.into_inner();
    for (ent, to_cull) in &chunks_to_cull {
        if ent == Entity::PLACEHOLDER {
            continue;
        }
        let chunk = chunks_query.get(ent).unwrap();
        let cords = chunk.cords;
        let grid = chunk.grid;
        let dims = chunk.meta_data.dims;
        let mut culled = [true; 6];
        for i in 0..6 {
            if to_cull.culled[i] {
                continue;
            }
            let face = Face::from(i);
            let adj_chunk_cords = match face {
                Right => [cords[0] + 1, cords[1]],
                Left => [cords[0] - 1, cords[1]],
                Back => [cords[0], cords[1] + 1],
                Forward => [cords[0], cords[1] - 1],
                Top | Bottom => panic!("Shouldn't happen"),
            };
            if let Some(adj_chunk) = chunk_map.get_ent(adj_chunk_cords) {
                if adj_chunk == Entity::PLACEHOLDER {
                    continue;
                }
                let adj_chunk_grid = { chunks_query.get(adj_chunk).unwrap().grid };
                for svox in iter_faces_of_chunk(dims, face) {
                    let adj_voxel_ind = match face {
                        Right => svox - WIDTH + 1,
                        Left => svox + WIDTH - 1,
                        Back => svox - WIDTH * (LENGTH - 1),
                        Forward => svox + WIDTH * (LENGTH - 1),
                        _ => panic!("Shouldn't happen"),
                    };

                    let adj_voxel = &adj_chunk_grid[adj_voxel_ind];
                    if breg.is_covering(adj_voxel, face.opposite()) {
                        let mut chunk = chunks_query.get_mut(ent).unwrap();
                        chunk
                            .meta_data
                            .log(VoxelChange::CullFaces, svox, grid[svox], {
                                let mut r = [None; 6];
                                r[face as usize] = Some(*adj_voxel);
                                r
                            });
                    }
                }
            } else {
                culled[face as usize] = false;
                continue;
            }
        }
        if culled == [true; 6] {
            commands.entity(ent).insert(ToUpdate).remove::<ToCull>();
        } else {
            commands
                .entity(ent)
                .insert(ToUpdate)
                .insert(ToCull { culled });
        }
    }
}
