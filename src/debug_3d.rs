use crate::CHUNK_DIMS;
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
