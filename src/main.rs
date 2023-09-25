#![allow(dead_code, unused_variables, unused_imports)]
mod add_break_blocks;
mod block_reg;
mod chunk;
mod chunk_queue;
mod player;
mod utils;

use add_break_blocks::*;
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, ComputeTaskPool, Task},
};
use bevy_meshem::prelude::*;
use block_reg::*;
use chunk::*;
use chunk_queue::*;
use futures_lite::future;
use noise::Perlin;
use player::*;
use std::{default, sync::Arc};
pub use utils::*;

// const FACTOR: usize = CHUNK_DIMS.0;
// Render distance should be above 1.
pub const RENDER_DISTANCE: i32 = 16;
pub const GEN_SEED: u32 = 5;

#[derive(Resource, Clone)]
pub struct BlockMaterial(Handle<StandardMaterial>);

#[derive(Component)]
struct ToUpdate;

#[derive(States, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub enum InitialChunkLoadState {
    #[default]
    Loading,
    MeshesLoaded,
    Complete,
}

#[derive(Component)]
struct LoadedChunks(usize);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_plugins(PlayerPlugin);

    app.init_resource::<BlockRegistry>();
    app.init_resource::<ChunkMap>();
    app.init_resource::<ChunkQueue>();
    app.insert_resource(AmbientLight {
        brightness: 0.5,
        color: Color::WHITE,
    });

    app.add_event::<BlockChange>();

    app.add_state::<InitialChunkLoadState>();

    app.add_systems(PreStartup, setup);
    app.add_systems(
        PostUpdate,
        update_closby_chunks.run_if(in_state(InitialChunkLoadState::Complete)),
    );
    app.add_systems(
        Update,
        check_if_loaded.run_if(in_state(InitialChunkLoadState::MeshesLoaded)),
    );
    app.add_systems(Update, update_mesh_frame);
    app.add_systems(Update, frame_chunk_update);
    app.add_systems(Update, handle_tasks);
    app.add_systems(Update, add_break_detector);
    app.add_systems(PostUpdate, spawn_and_despawn_chunks);
    app.add_systems(PostUpdate, handle_block_break_place);

    app.run();
}

fn setup(
    breg: Res<BlockRegistry>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let noise = Perlin::new(GEN_SEED);
    let texture_handle: Handle<Image> = asset_server.load("UV_map_example.png");
    let mat = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..default()
    });
    commands.insert_resource(BlockMaterial(mat));
    commands.spawn(LoadedChunks(0));
}

fn frame_chunk_update(
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

fn spawn_and_despawn_chunks(
    q2: Query<(&CurrentChunk, &Transform), Changed<CurrentChunk>>,
    mut chunk_queue: ResMut<ChunkQueue>,
    chunk_map: Res<ChunkMap>,
) {
    for (j, t) in q2.iter() {
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

fn update_closby_chunks(
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

fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut ComputeChunk)>,
    mat: Res<BlockMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_map: ResMut<ChunkMap>,
    current_chunk: Query<&CurrentChunk>,
    current_state: Res<State<InitialChunkLoadState>>,
    mut loaded_chunks: Query<(Entity, &mut LoadedChunks)>,
    mut next_state: ResMut<NextState<InitialChunkLoadState>>,
) {
    let cc = current_chunk
        .get_single()
        .expect("Couldn't find CurrentChunk component, which the player should always have.")
        .0;
    let mat = mat.into_inner().to_owned();
    for (entity, mut task) in transform_tasks.iter_mut() {
        if let Some(Some(((culled_mesh, metadata), grid, cords))) =
            future::block_on(future::poll_once(&mut task.0))
        {
            commands.entity(entity).remove::<ComputeChunk>();
            if (cc[0] - cords[0]).abs() > RENDER_DISTANCE {
                continue;
            }
            if (cc[1] - cords[1]).abs() > RENDER_DISTANCE {
                continue;
            }

            let culled_mesh_handle = meshes.add(culled_mesh);
            let ent = commands
                .spawn((
                    PbrBundle {
                        mesh: culled_mesh_handle,
                        material: mat.clone().0,
                        transform: Transform::from_xyz(
                            cords[0] as f32 * CHUNK_DIMS.0 as f32,
                            0.0,
                            cords[1] as f32 * CHUNK_DIMS.2 as f32,
                        ),
                        ..default()
                    },
                    Chunk {
                        // compressed_chunk: vec![(0, 0)],
                        grid,
                        cords,
                        meta_data: metadata,
                    },
                    BlockChangeQueue {
                        block_queue: vec![],
                    },
                ))
                .id();
            chunk_map.insert_ent(cords, ent);
            if let Ok((counter_ent, mut loaded_chunks)) = loaded_chunks.get_single_mut() {
                match current_state.get() {
                    &InitialChunkLoadState::Loading => {
                        loaded_chunks.0 += 1;
                        if loaded_chunks.0 == (RENDER_DISTANCE * RENDER_DISTANCE) as usize {
                            next_state.set(InitialChunkLoadState::MeshesLoaded);
                            commands.entity(counter_ent).despawn();
                            println!("\nInternal Log:\nMeshes have been loaded");
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn check_if_loaded(
    mut next_state: ResMut<NextState<InitialChunkLoadState>>,
    chunk_map: Res<ChunkMap>,
    mut commands: Commands,
) {
    for (chunk, ent) in chunk_map.iter() {
        match commands.get_entity(*ent) {
            None => return,
            _ => {}
        }
    }
    next_state.set(InitialChunkLoadState::Complete);
    println!("\nInternal Log:\nChunk entities have been successfully spawned");
}

fn handle_block_break_place(
    mut block_change: EventReader<BlockChange>,
    chunk_map: Res<ChunkMap>,
    mut chunk_query: Query<(Entity, &mut Chunk), With<ChunkCloseToPlayer>>,
    mut commands: Commands,
) {
    for event in block_change.iter() {
        'outer: for &(chunk, block) in event.blocks.iter() {
            let ent = chunk_map.get_ent(chunk).expect(
                "Chunk should be loaded into internal data structure `ChunkMap` but it isn't.",
            );
            for (e, mut c) in chunk_query.iter_mut() {
                if e != ent {
                    continue;
                }
                println!("hey1");
                assert_eq!(c.cords, chunk);
                let tmp_neighbors: Vec<Option<Block>> = vec![None; 6];
                let mut neighboring_voxels: [Option<Block>; 6] = [None; 6];

                for i in 0..6 {
                    neighboring_voxels[i] =
                        if let Some(a) = get_neighbor(block, Face::from(i), CHUNK_DIMS) {
                            Some(c.grid[a])
                        } else {
                            None
                        }
                }
                let vox = c.grid[block];

                if vox == AIR && matches!(event.change, VoxelChange::Broken) {
                    println!("hey2");
                    break;
                }
                if vox != AIR && matches!(event.change, VoxelChange::Added) {
                    break;
                }

                c.meta_data.log(
                    event.change,
                    block,
                    {
                        match event.change {
                            VoxelChange::Added => STONE,
                            VoxelChange::Broken => vox,
                        }
                    },
                    neighboring_voxels,
                );

                match event.change {
                    VoxelChange::Added => c.grid[block] = STONE,
                    VoxelChange::Broken => c.grid[block] = AIR,
                }

                println!("hey3");
                commands.entity(e).insert(ToUpdate);
                break 'outer;
            }
        }
    }
}

fn update_mesh_frame(
    mut query: Query<(Entity, &Handle<Mesh>, &mut Chunk), With<ToUpdate>>,
    mut meshes: ResMut<Assets<Mesh>>,
    breg: Res<BlockRegistry>,
) {
    let breg = Arc::new(breg.into_inner().clone());
    for (ent, mesh_handle, mut chunk) in query.iter_mut() {
        let mesh_ref_mut = meshes
            .get_mut(mesh_handle)
            .expect("Can't find chunk mesh in internal assets");
        update_mesh(mesh_ref_mut, &mut chunk.meta_data, &*breg.clone());
    }
}
