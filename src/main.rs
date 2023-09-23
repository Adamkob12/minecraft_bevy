#![allow(dead_code, unused_imports, unused_variables)]
mod block_reg;
mod chunk;
mod chunk_queue;
mod player;

use bevy::prelude::*;
use bevy_meshem::prelude::*;
use block_reg::*;
use chunk::*;
use chunk_queue::*;
use futures_lite::future;
use noise::{NoiseFn, Perlin, Seedable};
use player::*;
use std::sync::Arc;

const FACTOR: usize = CHUNK_DIMS.0;
pub const RENDER_DISTANCE: i32 = 16;
pub const GEN_SEED: u32 = 5;

#[derive(Resource, Clone)]
pub struct BlockMaterial(Handle<StandardMaterial>);

#[derive(Component)]
pub struct VertexCount(usize);

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

    app.add_systems(PreStartup, setup);
    app.add_systems(Update, frame_chunk_update);
    app.add_systems(Update, handle_tasks);
    app.add_systems(PostUpdate, spawn_and_despawn_chunks);

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
    commands.spawn(VertexCount(0));
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

        // for u in -RENDER_DISTANCE + 1..=RENDER_DISTANCE {
        //     chunk_queue.queue_spawn([cords[0] + RENDER_DISTANCE, cords[1] + u]);
        //     chunk_queue.queue_spawn([cords[0] - RENDER_DISTANCE, cords[1] + u]);
        //     chunk_queue.queue_spawn([cords[0] + u, cords[1] + RENDER_DISTANCE]);
        //     chunk_queue.queue_spawn([cords[0] + u, cords[1] - RENDER_DISTANCE]);
        // }
        for u in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for v in -RENDER_DISTANCE..=RENDER_DISTANCE {
                chunk_queue.queue_spawn([cords[0] + u, cords[1] + v]);
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
                        compressed_chunk: vec![(0, 0)],
                        cords,
                        meta_data: metadata,
                    },
                ))
                .id();
            chunk_map.insert_ent(cords, ent);
        }
    }
}
