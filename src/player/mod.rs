pub mod movement;
#[allow(unused_imports)]
use crate::*;
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::{
    core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    pbr::{
        ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionQualityLevel,
        ScreenSpaceAmbientOcclusionSettings,
    },
    prelude::*,
    render::camera::TemporalJitter,
};
use movement::*;
// ALWAYS ODD!
pub(crate) const CAGE_SIZE: usize = 7;
pub(crate) const HALF_CAGE_I: i32 = (CAGE_SIZE / 2) as i32;
pub const CAGE_LEN: usize = CAGE_SIZE * CAGE_SIZE * CAGE_SIZE;
pub(crate) const CAGE_DIMS: (usize, usize, usize) = (CAGE_SIZE, CAGE_SIZE, CAGE_SIZE);
// pub(crate) const GRAVITY: f32 = 0.005;

pub mod prelude {
    pub use crate::*;
}

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

/// Mouse sensitivity and movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

#[derive(Component)]
#[allow(non_snake_case)]
pub struct VelocityVectors {
    pub yV: Vec3,
    pub xV: Vec3,
    pub zV: Vec3,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00008,
            speed: 10.,
        }
    }
}

/// Key configuration
#[derive(Resource)]
pub struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_ascend: KeyCode,
    pub move_descend: KeyCode,
    pub toggle_grab_cursor: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::W,
            move_backward: KeyCode::S,
            move_left: KeyCode::A,
            move_right: KeyCode::D,
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ShiftLeft,
            toggle_grab_cursor: KeyCode::Escape,
        }
    }
}

/// Used in queries when you want flycams and not other cameras
/// A marker component used in queries when you want flycams and not other cameras
#[derive(Component)]
pub struct FlyCam;

#[derive(Component)]
// Keeps track of the blocks surrounding the player for physics
pub struct Cage {
    pub blocks: [Block; CAGE_LEN],
}

#[derive(Component)]
pub struct CurrentChunk(pub [i32; 2]);

fn update_cage(
    chunk_query: Query<&Chunk, With<ChunkCloseToPlayer>>,
    mut player_query: Query<(&mut Cage, &Transform), Changed<Transform>>,
) {
    if let Ok((mut cage, tran)) = player_query.get_single_mut() {
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
                    let (chunk_pos, block_pos, flag) =
                        position_to_chunk_position(block_pos, CHUNK_DIMS);

                    let block = if !flag {
                        AIR
                    } else {
                        let mut r = AIR;
                        for chunk in chunk_query.iter() {
                            if chunk.cords == chunk_pos {
                                r = chunk.grid[one_d_cords(block_pos, CHUNK_DIMS)];
                            }
                        }
                        r
                    };
                    cage.blocks[cage_index_1d] = block;
                }
            }
        }
    }
}

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

fn cursor_grab(
    keys: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_grab_cursor) {
            toggle_grab_cursor(&mut window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        assert!(CAGE_SIZE % 2 == 1, "Cage size should always be odd!");
        app.add_plugins((TemporalAntiAliasPlugin,))
            .init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .add_systems(Startup, setup_player)
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(
                Update,
                (player_move, player_look, cursor_grab, update_cage)
                    .run_if(in_state(InitialChunkLoadState::Complete)),
            );
    }
}
