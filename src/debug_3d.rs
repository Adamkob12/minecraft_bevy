#![allow(dead_code)]
use crate::player::prelude::AIR;
use crate::CHUNK_DIMS;
use crate::{one_d_cords, Cage, CAGE_DIMS, HALF_CAGE_I};
use bevy::prelude::*;

use crate::player::FlyCam;

pub fn debug_cords(mut gizmos: Gizmos) {
    for i in 0..CHUNK_DIMS.0 {
        for j in 0..CHUNK_DIMS.2 {
            for k in 0..CHUNK_DIMS.1 {
                gizmos.sphere(
                    Vec3::new(i as f32, k as f32, j as f32),
                    Quat::IDENTITY,
                    0.1,
                    Color::RED,
                );
            }
        }
    }
}

pub fn debug_cage(mut gizmos: Gizmos, cage: Query<(&Cage, &Transform)>) {
    if let Ok((cage, tran)) = cage.get_single() {
        let pos = tran.translation;
        let current_block: Vec3 = [pos.x.round(), pos.y.round(), pos.z.round()].into();
        for x in -HALF_CAGE_I..=HALF_CAGE_I {
            for z in -HALF_CAGE_I..=HALF_CAGE_I {
                for y in -HALF_CAGE_I..=HALF_CAGE_I {
                    let cage_index_3d = [
                        (x + HALF_CAGE_I) as usize,
                        (y + HALF_CAGE_I) as usize,
                        (z + HALF_CAGE_I) as usize,
                    ];
                    let cage_index_1d = one_d_cords(cage_index_3d, CAGE_DIMS);
                    let block_pos = current_block + Vec3::new(x as f32, y as f32, z as f32);
                    if cage.blocks[cage_index_1d] != AIR {
                        gizmos.cuboid(
                            Transform::from_xyz(block_pos.x, block_pos.y, block_pos.z)
                                .with_scale([1.0, 1.0, 1.0].into()),
                            Color::CYAN,
                        );
                    }
                }
            }
        }
    }
}

pub fn draw_ray_forward(mut gizmos: Gizmos, player: Query<&Transform, With<FlyCam>>) {
    if let Ok(tran) = player.get_single() {
        // gizmos.ray(
        //     tran.translation,
        //     tran.forward().normalize() * 5.0,
        //     Color::RED,
        // );
        // gizmos.sphere(
        //     tran.translation + tran.forward() * 1.0,
        //     Quat::IDENTITY,
        //     0.25,
        //     Color::GREEN,
        // );
        // gizmos.sphere(
        //     tran.translation + tran.forward() * 2.0,
        //     Quat::IDENTITY,
        //     0.5,
        //     Color::YELLOW,
        // );
        // gizmos.sphere(
        //     tran.translation + tran.forward() * 3.0,
        //     Quat::IDENTITY,
        //     0.8,
        //     Color::ORANGE,
        // );
        gizmos
            .sphere(
                tran.translation + tran.forward() * 4.0,
                Quat::IDENTITY,
                0.25,
                Color::RED,
            )
            .circle_segments(64);
    }
}
