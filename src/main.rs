// #![allow(dead_code, unused_variables, unused_imports)]
mod add_break_blocks;
mod block_reg;
mod chunk;
mod debug_3d;
mod player;
mod sky;
mod utils;

use add_break_blocks::*;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_atmosphere::prelude::*;
use bevy_meshem::prelude::*;
use block_reg::*;
use chunk::*;
use core::f32::consts::PI;
#[allow(unused_imports)]
use debug_3d::*;
use futures_lite::future;
use noise::Perlin;
use player::*;
use sky::*;
use std::sync::Arc;
pub use utils::*;

// const FACTOR: usize = CHUNK_DIMS.0;
// Render distance should be above 1.
pub const RENDER_DISTANCE: i32 = 4;
pub const GEN_SEED: u32 = 5;
const CROSSHAIR_SIZE: f32 = 36.0;

#[derive(Resource, Clone)]
pub struct BlockMaterial(Handle<StandardMaterial>);

#[derive(Component)]
struct LoadedChunks(usize);

#[rustfmt::skip]
fn main() {
    let mut app = App::new();
    
    // Plugins
    app
        .add_plugins

        ((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: bevy::window::WindowMode::Windowed,
                    ..Default::default()}),..Default::default()}),

        AtmospherePlugin,
        PlayerPlugin,
        ChunkPlugin,
    ));

    // Resources
    app

        .init_resource::<BlockRegistry>()
        .insert_resource(AmbientLight {
                brightness: 0.8, color: Color::WHITE,})
        .insert_resource(CycleTimer(Timer::new(
                bevy::utils::Duration::from_millis(50),
                TimerMode::Repeating,)))
        .insert_resource(AtmosphereModel::default());

    // Events
    app.add_event::<BlockChange>();

    // Systems
    app.add_systems(PostStartup, setup)
        .add_systems(
            PostUpdate, (daylight_cycle).run_if(in_state(InitialChunkLoadState::Complete)),)
        .add_systems(OnEnter(InitialChunkLoadState::Complete), setup_light)
        .add_systems(Update,
            check_if_loaded.run_if(in_state(InitialChunkLoadState::MeshesLoaded)),)
        .add_systems(Update,(handle_tasks, add_break_detector, /* debug_cage */),)
        .add_systems(PostUpdate, handle_block_break_place);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<&mut Projection>,
) {
    let texture_handle: Handle<Image> = asset_server.load("UV_map_example.png");
    let mat = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        reflectance: 0.0,
        ..default()
    });
    commands.insert_resource(BlockMaterial(mat));
    commands.spawn(LoadedChunks(0));
    let mut projection = camera_query.get_single_mut().unwrap();
    if let Projection::Perspective(ref mut perspective) = *projection {
        perspective.fov = PI / 3.0;
    }
}

// System to handle chunk spawning / despawning every frame.
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
    // Get the current chunk.
    let current_chunk = current_chunk
        .get_single()
        .expect("Couldn't find CurrentChunk component, which the player should always have.")
        .0;
    let mat = mat.into_inner().to_owned();
    // Iterate over the tasks.
    for (entity, mut task) in transform_tasks.iter_mut() {
        if let Some(Some(((culled_mesh, metadata), grid, cords))) =
            future::block_on(future::poll_once(&mut task.0))
        {
            // Remove the task so we don't poll it again
            commands.entity(entity).remove::<ComputeChunk>();
            // If while the task was computing, the player left the area from which the chunk
            // should be in, we just don't spawn the chunk.
            if (current_chunk[0] - cords[0]).abs() > RENDER_DISTANCE
                || (current_chunk[1] - cords[1]).abs() > RENDER_DISTANCE
            {
                chunk_map.remove_ent(cords, Entity::PLACEHOLDER);
                continue;
            }

            // // Extract the vertex data for the physics engine.
            // let pos_vertices = extract_position_vertex_data(&culled_mesh);
            // // Extract the indices for the physics engine.
            // let indices = extract_indices_data(&culled_mesh);

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
                ))
                .id();
            // Remember that the ChunkMap already as an Entity Placeholder stored (explanation in
            // the function dequeue_all) so we swap it for the real chunk's entity.
            chunk_map.change_ent(cords, ent);
            if let Ok((counter_ent, mut loaded_chunks)) = loaded_chunks.get_single_mut() {
                // Update the number of chunks loaded, and wether all the chunks (on startup) have
                // been loaded initially.
                match current_state.get() {
                    &InitialChunkLoadState::Loading => {
                        loaded_chunks.0 += 1;
                        if loaded_chunks.0 == (RENDER_DISTANCE * RENDER_DISTANCE) as usize {
                            next_state.set(InitialChunkLoadState::MeshesLoaded);
                            commands.entity(counter_ent).despawn();
                            info!("\nInternal Log:\nMeshes have been loaded");
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

// Quick system to check if all the Chunks have been initially loaded.
fn check_if_loaded(
    mut next_state: ResMut<NextState<InitialChunkLoadState>>,
    chunk_map: Res<ChunkMap>,
    mut commands: Commands,
) {
    for (_, ent) in chunk_map.iter() {
        match commands.get_entity(*ent) {
            None => return,
            _ => {}
        }
    }
    next_state.set(InitialChunkLoadState::Complete);
    info!("\nInternal Log:\nChunk entities have been successfully spawned");
}

// Whoever reads this funtion sometime in the future, Please forgive me.
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
            let (onto_chunk, onto) = onto.unwrap_or((
                [i16::min_value() as i32, i32::min_value()],
                usize::max_value(),
            ));
            let onto_ent = if onto_chunk != [i16::min_value() as i32, i32::min_value()] {
                chunk_map.get_ent(onto_chunk).expect(
                    "Chunk should be loaded into internal data structure `ChunkMap` but it isn't",
                )
            } else {
                Entity::PLACEHOLDER
            };
            let mut onto_chunk = [u16::max_value(); CHUNK_LEN];
            for (e, c) in chunk_query.iter() {
                if e == onto_ent {
                    onto_chunk = c.grid;
                    break;
                }
            }
            if onto_ent != Entity::PLACEHOLDER {
                assert_ne!(onto_chunk[0], u16::max_value());
            }
            'B: for (e, mut c) in chunk_query.iter_mut() {
                if e != ent {
                    continue 'B;
                }
                assert_eq!(c.cords, chunk);
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
                let onto = if onto == usize::max_value() {
                    u16::max_value()
                } else {
                    onto_chunk[onto]
                };

                if vox == AIR && matches!(event.change, VoxelChange::Broken) {
                    continue 'A;
                }
                if (onto == AIR || vox != AIR) && matches!(event.change, VoxelChange::Added) {
                    if onto != AIR {
                        break 'A;
                    }
                    continue 'A;
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
                    VoxelChange::Added => {
                        c.grid[block] = STONE;
                    }
                    VoxelChange::Broken => c.grid[block] = AIR,
                }

                commands.entity(e).insert(ToUpdate);
                break 'A;
            }
        }
    }
}
