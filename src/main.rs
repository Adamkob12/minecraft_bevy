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
use std::rc::Rc;

const FACTOR: usize = CHUNK_DIMS.0;
pub const RENDER_DISTANCE: i32 = 5;
pub const GEN_SEED: u32 = 5;

#[derive(Resource, Clone)]
pub struct BlockMaterial(Handle<StandardMaterial>);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.init_resource::<BlockRegistry>();
    app.init_resource::<ChunkMap>();
    app.init_resource::<ChunkQueue>();

    app.insert_resource(AmbientLight {
        brightness: 0.5,
        color: Color::WHITE,
    });

    app.add_systems(PreStartup, setup);
    app.add_systems(PreStartup, frame_chunk_update);
    app.add_systems(Update, handle_tasks);

    app.run();
}

fn setup(
    breg: Res<BlockRegistry>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut chunk_queue: ResMut<ChunkQueue>,
) {
    let noise = Perlin::new(GEN_SEED);
    let texture_handle: Handle<Image> = asset_server.load("UV_map_example.png");
    let mat = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..default()
    });
    commands.insert_resource(BlockMaterial(mat));

    for x in (-RENDER_DISTANCE)..RENDER_DISTANCE + 1 {
        for z in (-RENDER_DISTANCE)..RENDER_DISTANCE + 1 {
            chunk_queue.queue_spawn([x, z]);
        }
    }
    // chunk_queue.queue_spawn([0, 0]);

    let camera_and_light_transform = Transform::from_xyz(
        FACTOR as f32 * 1.7,
        FACTOR as f32 * 1.7,
        FACTOR as f32 * 1.7,
    )
    .looking_at(
        Vec3::new(
            FACTOR as f32 * 0.5,
            FACTOR as f32 * 0.5,
            FACTOR as f32 * 0.5,
        ),
        Vec3::Y,
    );

    // Camera in 3D space.
    commands.spawn(Camera3dBundle {
        transform: camera_and_light_transform,
        ..default()
    });

    // Light up the scene.
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 1000.0,
            ..default()
        },
        transform: camera_and_light_transform,
        ..default()
    });
}

fn frame_chunk_update(
    mut cq: ResMut<ChunkQueue>,
    cm: ResMut<ChunkMap>,
    breg: Res<BlockRegistry>,
    commands: Commands,
) {
    cq.dequeue_all(commands, breg.into_inner().clone(), cm.into_inner());
}

fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut ComputeChunk)>,
    mat: Res<BlockMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_map: ResMut<ChunkMap>,
) {
    let mat = mat.into_inner().to_owned();
    for (entity, mut task) in transform_tasks.iter_mut() {
        println!("S");
        if let Some(Some(((culled_mesh, metadata), grid, cords))) =
            future::block_on(future::poll_once(&mut task.0))
        {
            print!("SS\n");
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
                        compressed_chunk: rle_compress(&grid),
                        cords,
                        meta_data: metadata,
                    },
                ))
                .id();
            chunk_map.insert_ent(cords, ent);
            commands.entity(entity).remove::<ComputeChunk>();
        }
    }
}
