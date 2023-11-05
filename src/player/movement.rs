use super::*;
use bevy::{
    core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    pbr::{
        ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionQualityLevel,
        ScreenSpaceAmbientOcclusionSettings,
    },
    prelude::*,
    render::camera::TemporalJitter,
};

/// Grabs the cursor when game first starts
pub(super) fn initial_grab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

/// Spawns the `Camera3dBundle` to be controlled
pub(super) fn setup_player(mut commands: Commands) {
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(
                    // -RENDER_DISTANCE as f32 * WIDTH as f32,
                    0.0,
                    HEIGHT as f32 * 2.0, // -RENDER_DISTANCE as f32 * LENGTH as f32,
                    0.0,
                )
                .looking_to(Vec3::new(5.0, -1.0, 5.0), Vec3::Y),
                ..Default::default()
            },
            Cage {
                blocks: [AIR; CAGE_LEN],
            },
            FlyCam,
            CurrentChunk([0, 0]),
            VelocityVectors {
                xV: Vec3::ZERO,
                yV: Vec3::ZERO,
                zV: Vec3::ZERO,
            },
        ))
        .insert(ScreenSpaceAmbientOcclusionBundle {
            settings: ScreenSpaceAmbientOcclusionSettings {
                quality_level: ScreenSpaceAmbientOcclusionQualityLevel::High,
            },
            ..Default::default()
        });
}

/// Handles keyboard input and movement
pub(super) fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut query: Query<(
        &FlyCam,
        &mut Transform,
        &mut CurrentChunk,
        &Cage,
        &mut VelocityVectors,
    )>,
    //    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for (_camera, mut transform, mut chunk, cage, mut _vv) in query.iter_mut() {
            let mut direction = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    if keys.pressed(key_bindings.move_forward) {
                        direction += forward;
                    }
                    if keys.pressed(key_bindings.move_backward) {
                        direction -= forward;
                    }
                    if keys.pressed(key_bindings.move_right) {
                        direction += right;
                    }
                    if keys.pressed(key_bindings.move_left) {
                        direction -= right;
                    }
                    if keys.pressed(key_bindings.move_ascend) {
                        direction += Vec3::Y;
                    }
                    if keys.pressed(key_bindings.move_descend) {
                        direction -= Vec3::Y;
                    }
                }
            }
            direction = direction.normalize_or_zero();
            // Calculate if we collided with a block
            let velocity = direction * time.delta_seconds() * settings.speed;
            let new_pos = transform.translation + velocity;
            let tmp = transform.translation + direction;
            let pos = transform.translation;
            let current_block: Vec3 = [pos.x.round(), pos.y.round(), pos.z.round()].into();
            let tmp_block: Vec3 = [tmp.x.round(), tmp.y.round(), tmp.z.round()].into();
            let Vec3 {
                x: dx,
                y: dy,
                z: dz,
            } = tmp_block - current_block;
            let xblock_cords = [
                (HALF_CAGE_I + dx as i32) as usize,
                (HALF_CAGE_I as i32) as usize,
                (HALF_CAGE_I as i32) as usize,
            ];
            let yblock_cords = [
                (HALF_CAGE_I as i32) as usize,
                (HALF_CAGE_I + dy as i32) as usize,
                (HALF_CAGE_I as i32) as usize,
            ];
            let zblock_cords = [
                (HALF_CAGE_I as i32) as usize,
                (HALF_CAGE_I as i32) as usize,
                (HALF_CAGE_I + dz as i32) as usize,
            ];
            let xzblock_cords = [
                (HALF_CAGE_I + dx as i32) as usize,
                (HALF_CAGE_I as i32) as usize,
                (HALF_CAGE_I + dz as i32) as usize,
            ];

            let xblock = one_d_cords(xblock_cords, CAGE_DIMS);
            let yblock = one_d_cords(yblock_cords, CAGE_DIMS);
            let zblock = one_d_cords(zblock_cords, CAGE_DIMS);
            let xzblock = one_d_cords(xzblock_cords, CAGE_DIMS);
            let xblock = cage.blocks[xblock];
            let yblock = cage.blocks[yblock];
            let zblock = cage.blocks[zblock];
            let xzblock = cage.blocks[xzblock];

            if xblock == AIR {
                transform.translation.x = new_pos.x;
            } else if (pos.x - pos.x.round() + 0.5 * dx.signum() * -1.0).abs() > 0.5 {
                transform.translation.x +=
                    velocity.x * (pos.x - pos.x.round() + 0.5 * dx.signum() * -1.0).powi(2);
            }
            if yblock == AIR {
                transform.translation.y = new_pos.y;
            } else if (pos.y - pos.y.round() + 0.5 * dy.signum() * -1.0).abs() > 0.5 {
                transform.translation.y +=
                    velocity.y * (pos.y - pos.y.round() + 0.5 * dy.signum() * -1.0).powi(2);
            }
            if zblock == AIR && xzblock == AIR {
                transform.translation.z = new_pos.z;
            } else if (pos.z - pos.z.round() + 0.5 * dz.signum() * -1.0).abs() > 0.5 {
                transform.translation.z +=
                    velocity.z * (pos.z - pos.z.round() + 0.5 * dz.signum() * -1.0).powi(2);
            }

            let t = transform.translation;
            // find the current chunk we are in
            let tmp = position_to_chunk(t, CHUNK_DIMS);
            if tmp != chunk.0 {
                chunk.0 = tmp;
            }
            // look_transform.eye = transform.translation;
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

// /// Handles looking around if cursor is locked
pub(super) fn player_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.read(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}
