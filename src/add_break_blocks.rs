use crate::*;
use bevy::prelude::*;
use bevy_meshem::prelude::*;

const RAY_FORWARD_STEP: f32 = 0.01;
const NANO_STEP_FACTOR: f32 = 15.0;
pub const REACH_DISTANCE: u8 = 5;

#[derive(Event)]
pub struct BlockChange {
    // Only if we are placing a block, we need to know what block is against the block we are
    // placing, because we can't place blocks in the air. If change is `Broken` then this is None.
    pub blocks: Vec<([i32; 2], usize, Option<([i32; 2], usize)>)>,
    pub change: VoxelChange,
}

pub fn add_break_detector(
    mut block_change_event_writer: EventWriter<BlockChange>,
    player_query: Query<(&CurrentChunk, &Transform), With<FlyCam>>,
    buttons: Res<Input<MouseButton>>,
) {
    if let Ok((_, tran)) = player_query.get_single() {
        let forward = tran.forward();
        let pos = tran.translation;

        if buttons.just_pressed(MouseButton::Left) {
            block_change_event_writer.send(BlockChange {
                blocks: blocks_in_the_way(pos, forward, REACH_DISTANCE)
                    .iter()
                    .filter_map(|(x, y, _z)| {
                        if let Some(tmp) = one_d_cords_safe(*y, CHUNK_DIMS) {
                            Some((*x, tmp, None))
                        } else {
                            None
                        }
                    })
                    .collect(),
                change: VoxelChange::Broken,
            });
        }

        if buttons.just_pressed(MouseButton::Right) {
            block_change_event_writer.send(BlockChange {
                change: VoxelChange::Added,
                blocks: blocks_in_the_way(pos, forward, REACH_DISTANCE)
                    .iter()
                    .skip(1)
                    .filter_map(|&(x, y, z)| {
                        let tmp = one_d_cords_safe(y, CHUNK_DIMS)?;
                        if let Some(block) = get_neighbor(tmp, z, CHUNK_DIMS) {
                            Some((x, block, Some((x, tmp))))
                        } else {
                            match z {
                                Top => {
                                    warn!(
                                        "\nIn-Game Error: \nMaximum build limit has been reached"
                                    );
                                    None
                                }
                                Bottom => {
                                    warn!(
                                        "\nIn-Game Error: \nMinimum build limit has been reached"
                                    );
                                    None
                                }
                                Right => Some(([x[0] + 1, x[1]], tmp - WIDTH + 1, Some((x, tmp)))),
                                Left => Some(([x[0] - 1, x[1]], tmp + WIDTH - 1, Some((x, tmp)))),
                                Back => Some((
                                    [x[0], x[1] + 1],
                                    tmp - WIDTH * (LENGTH - 1),
                                    Some((x, tmp)),
                                )),
                                Forward => Some((
                                    [x[0], x[1] - 1],
                                    tmp + WIDTH * (LENGTH - 1),
                                    Some((x, tmp)),
                                )),
                            }
                            // (
                            //     x,
                            //     one_d_cords(y, CHUNK_DIMS),
                            //     Some(one_d_cords(y, CHUNK_DIMS)),
                            // )
                        }
                    })
                    .collect(),
            });
        }
    }
}

fn blocks_in_the_way(pos: Vec3, forward: Vec3, distance: u8) -> Vec<([i32; 2], [usize; 3], Face)> {
    let step = forward * RAY_FORWARD_STEP;
    let possible_faces: [Face; 3] = [
        if forward.x > 0.0 { Left } else { Right },
        if forward.y > 0.0 { Bottom } else { Top },
        if forward.z > 0.0 { Forward } else { Back },
    ];
    // let mut point = pos + Vec3::new(0.5, 0.5, 0.5);
    let mut point = pos;
    let mut current_block = [point.x.round(), point.y.round(), point.z.round()];
    let mut to_return: Vec<([i32; 2], [usize; 3], Face)> = vec![];

    while point.distance(pos) < distance as f32 {
        point += step;
        let tmp = [point.x.round(), point.y.round(), point.z.round()];
        if tmp != current_block {
            current_block = tmp;
            let face = {
                let r: Face;
                let mut p = point - step;
                let nano_step = step / NANO_STEP_FACTOR;
                loop {
                    p += nano_step;
                    let tmp = [p.x.round(), p.y.round(), p.z.round()];
                    if tmp == current_block {
                        r = closest_face(p, possible_faces);
                        break;
                    }
                }
                r
            };
            let block_pos = position_to_chunk_position(point, CHUNK_DIMS);
            to_return.push((block_pos.0, block_pos.1, face));
        }
    }
    to_return
}

#[allow(non_snake_case)]
fn closest_face(p: Vec3, possible_faces: [Face; 3]) -> Face {
    let mut min = f32::MAX;
    let mut face = Bottom;

    for f in possible_faces {
        let d = distance_from_face(p, f);
        if d < min {
            face = f;
            min = d;
        }
    }

    return face;
}

#[allow(non_snake_case)]
fn distance_from_face(p: Vec3, face: Face) -> f32 {
    let x = p.x;
    let z = p.z;
    let y = p.y;
    let X = p.x.round();
    let Z = p.z.round();
    let Y = p.y.round();
    match face {
        Top => (Y + 0.5 - y).abs(),
        Bottom => (Y - 0.5 - y).abs(),
        Right => (X + 0.5 - x).abs(),
        Left => (X - 0.5 - x).abs(),
        Back => (Z + 0.5 - z).abs(),
        Forward => (Z - 0.5 - z).abs(),
    }
}
