#![allow(dead_code, unused_variables, unused_imports)]
mod add_break_blocks;
mod block_reg;
mod chunk;
mod chunk_queue;
mod compute_aabb;
mod debug_3d;
mod player;
mod sky;
mod utils;

use add_break_blocks::*;
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, ComputeTaskPool, Task},
    window::PrimaryWindow,
};
use bevy_atmosphere::prelude::*;
use bevy_meshem::prelude::*;
use block_reg::*;
use chunk::*;
use chunk_queue::*;
use debug_3d::*;
use futures_lite::future;
use noise::Perlin;
use player::*;
use sky::*;
use std::{default, sync::Arc};
pub use utils::*;

use crate::utils::three_d_cords;

// const FACTOR: usize = CHUNK_DIMS.0;
// Render distance should be above 1.
pub const RENDER_DISTANCE: i32 = 8;
pub const GEN_SEED: u32 = 5;
const CROSSHAIR_SIZE: f32 = 36.0;

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
    app.add_plugins(AtmospherePlugin);
    app.add_plugins(PlayerPlugin);

    app.init_resource::<BlockRegistry>();
    app.init_resource::<ChunkMap>();
    app.init_resource::<ChunkQueue>();
    app.insert_resource(AmbientLight {
        brightness: 0.25,
        color: Color::WHITE,
    });
    app.insert_resource(CycleTimer(Timer::new(
        bevy::utils::Duration::from_millis(50),
        TimerMode::Repeating,
    )));
    app.insert_resource(AtmosphereModel::default());

    app.add_event::<BlockChange>();

    app.add_state::<InitialChunkLoadState>();

    app.add_systems(PostStartup, setup);
    app.add_systems(
        PostUpdate,
        (update_closby_chunks, daylight_cycle).run_if(in_state(InitialChunkLoadState::Complete)),
    );
    app.add_systems(OnEnter(InitialChunkLoadState::Complete), setup_light);
    app.add_systems(
        Update,
        check_if_loaded.run_if(in_state(InitialChunkLoadState::MeshesLoaded)),
    );
    app.add_systems(
        Update,
        (
            // draw_ray_forward,
            handle_tasks,
            frame_chunk_update,
            update_mesh_frame,
            add_break_detector,
        ),
    );
    app.add_systems(Update, spawn_and_despawn_chunks);
    app.add_systems(Update, handle_block_break_place);

    app.run();
}

fn setup(
    breg: Res<BlockRegistry>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    let noise = Perlin::new(GEN_SEED);
    let texture_handle: Handle<Image> = asset_server.load("UV_map_example.png");
    let mat = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        reflectance: 0.0,
        ..default()
    });
    commands.insert_resource(BlockMaterial(mat));
    commands.spawn(LoadedChunks(0));
    let mut window_width = CROSSHAIR_SIZE;
    let mut window_height = CROSSHAIR_SIZE;
    if let Ok(window) = primary_window.get_single() {
        (window_width, window_height) = (window.resolution.width(), window.resolution.height());
    } else {
        warn!("Primary window not found ");
    }

    commands.spawn(
        TextBundle::from_section(
            format!("+"),
            TextStyle {
                font_size: CROSSHAIR_SIZE,
                color: Color::LIME_GREEN,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            top: Val::Px(window_height / 2.0 - CROSSHAIR_SIZE / 2.0),
            left: Val::Px(window_width / 2.0 - CROSSHAIR_SIZE / 2.0),
            ..default()
        }),
    );
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
        'A: for &(chunk, block, onto) in event.blocks.iter() {
            let ent = chunk_map.get_ent(chunk).expect(
                "Chunk should be loaded into internal data structure `ChunkMap` but it isn't.",
            );
            'B: for (e, mut c) in chunk_query.iter_mut() {
                if e != ent {
                    continue 'B;
                }
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
                let onto = onto.unwrap_or(usize::max_value());
                let onto = if onto == usize::max_value() {
                    u16::max_value()
                } else {
                    c.grid[onto]
                };

                dbg!(block, onto, vox, three_d_cords(block, CHUNK_DIMS));

                if vox == AIR && matches!(event.change, VoxelChange::Broken) {
                    println!("hey2");
                    break;
                }
                if (onto == AIR || vox != AIR) && matches!(event.change, VoxelChange::Added) {
                    println!("hey2");
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
                break 'A;
            }
        }
    }
}

fn update_mesh_frame(
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
            commands.entity(ent).insert(aabb).remove::<ToUpdate>();
        } else {
            warn!("Couldn't compute Aabb for mesh after updating");
        }
    }
}
