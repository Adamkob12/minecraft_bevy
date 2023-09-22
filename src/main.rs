#![allow(dead_code, unused_imports, unused_variables)]
mod block_reg;
mod chunk;

use bevy::pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy_meshem::prelude::*;
use block_reg::*;
use chunk::*;
use noise::{NoiseFn, Perlin, Seedable};

const FACTOR: usize = CHUNK_DIMS.0;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).add_plugins(WireframePlugin);
    app.init_resource::<BlockRegistry>();

    app.insert_resource(AmbientLight {
        brightness: 0.5,
        color: Color::WHITE,
    });

    app.add_systems(Startup, setup);

    app.run();
}

fn setup(
    breg: Res<BlockRegistry>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let noise = Perlin::new(1);
    let texture_handle: Handle<Image> = asset_server.load("UV_map_example.png");
    let grid = generate_chunk([0, 0], noise);
    let dims = CHUNK_DIMS;
    let (culled_mesh, metadata) = mesh_grid(
        dims,
        grid.to_vec(),
        breg.into_inner(),
        MeshingAlgorithm::Culling,
    )
    .unwrap();
    let culled_mesh_handle: Handle<Mesh> = meshes.add(culled_mesh.clone());
    commands.spawn(PbrBundle {
        mesh: culled_mesh_handle,
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle),
            ..default()
        }),
        ..default()
    });

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
