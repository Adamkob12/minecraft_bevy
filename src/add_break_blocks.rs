use crate::{one_d_cords, three_d_cords, *};
use bevy::prelude::*;
use bevy_meshem::prelude::*;

const RAY_FORWARD_STEP: f32 = 0.01;
const NANO_STEP_FACTOR: f32 = 15.0;
const REACH_DISTANCE: u8 = 7;

#[derive(Event)]
pub struct BlockChange {
    // Only if we are placing a block, we need to know what block is against the block we are
    // placing, because we can't place blocks in the air. If change is `Broken` then this is None.
    pub blocks: Vec<([i32; 2], usize, Option<usize>)>,
    pub change: VoxelChange,
}

pub fn add_break_detector(
    mut block_change_event_writer: EventWriter<BlockChange>,
    player_query: Query<(&CurrentChunk, &Transform), With<FlyCam>>,
    buttons: Res<Input<MouseButton>>,
) {
    if let Ok((chunk, tran)) = player_query.get_single() {
        if tran.translation.y > HEIGHT as f32 || tran.translation.y < 0.0 {
            return;
        }
        let chunk = (*chunk).0;
        let forward = tran.forward();
        let pos = tran.translation;

        if buttons.just_pressed(MouseButton::Left) {
            block_change_event_writer.send(BlockChange {
                blocks: blocks_in_the_way(pos, forward, REACH_DISTANCE)
                    .iter()
                    .map(|(x, y, z)| (*x, one_d_cords(*y, CHUNK_DIMS), None))
                    .collect(),
                change: VoxelChange::Broken,
            });
        }
        if buttons.just_pressed(MouseButton::Right) {
            block_change_event_writer.send(BlockChange {
                change: VoxelChange::Added,
                blocks: blocks_in_the_way(pos, forward, REACH_DISTANCE)
                    .iter()
                    .rev()
                    .map(|&(x, y, z)| {
                        let tmp = one_d_cords(y, CHUNK_DIMS);
                        if let Some(block) = get_neighbor(tmp, z, CHUNK_DIMS) {
                            // dbg!((x, block));
                            (x, block, Some(tmp))
                        } else {
                            match z {
                                Top => panic!(
                                    "\nIn-Game Error: \nMaximum build limit has been reached"
                                ),
                                Bottom => {
                                    panic!("\nIn-Game Error: \nCan't build lower than y = 0.")
                                }
                                Right => ([x[0] + 1, x[1]], tmp - WIDTH + 1, Some(tmp)),
                                Left => ([x[0] - 1, x[1]], tmp + WIDTH - 1, Some(tmp)),
                                Back => ([x[0], x[1] + 1], tmp - WIDTH * (LENGTH - 1), Some(tmp)),
                                Forward => {
                                    ([x[0], x[1] - 1], tmp + WIDTH * (LENGTH - 1), Some(tmp))
                                }
                            }
                        }
                    })
                    .collect(),
            });
        }
    }
}

fn blocks_in_the_way(pos: Vec3, forward: Vec3, distance: u8) -> Vec<([i32; 2], [usize; 3], Face)> {
    println!("\n----\nposition: {}", pos);
    println!("forward vector: {}", forward);
    let step = forward * RAY_FORWARD_STEP;
    let mut point = pos;
    let mut current_block = [
        point.x.floor() + 0.5,
        point.y.floor() + 0.5,
        point.z.floor() + 0.5,
    ];
    let mut to_return: Vec<([i32; 2], [usize; 3], Face)> = vec![];

    while point.distance(pos) < distance as f32 {
        point += step;
        let tmp = [
            point.x.floor() + 0.5,
            point.y.floor() + 0.5,
            point.z.floor() + 0.5,
        ];
        if tmp != current_block {
            println!("Encountered block {:?}", tmp);
            current_block = tmp;
            let face = {
                let mut r: Face = Top;
                let mut p = point - step;
                let nano_step = step / NANO_STEP_FACTOR;
                for _ in 1..NANO_STEP_FACTOR as usize {
                    p += nano_step;
                    let tmp = [p.x.floor() + 0.5, p.y.floor() + 0.5, p.z.floor() + 0.5];
                    if tmp == current_block {
                        r = closest_face(p);
                        break;
                    }
                }
                r
            };
            let block_pos = position_to_chunk_position(point, CHUNK_DIMS);
            dbg!(block_pos, face as usize);
            to_return.push((block_pos.0, block_pos.1, face));
        }
    }
    to_return
}

//
// fn closest_face(p: Vec3) -> Face {
//     let mut min = f32::MAX;
//     let mut face = Top;
//
//     if (p.x.floor() - p.x).abs() < min {
//         min = (p.x.floor() - p.x).abs();
//         face = Left;
//     }
//     if (p.x.ceil() - p.x).abs() < min {
//         min = (p.x.ceil() - p.x).abs();
//         face = Right;
//     }
//     if (p.z.floor() - p.z).abs() < min {
//         min = (p.z.floor() - p.z).abs();
//         face = Forward;
//     }
//     if (p.z.ceil() - p.z).abs() < min {
//         min = (p.z.ceil() - p.z).abs();
//         face = Back;
//     }
//     if (p.y.floor() - p.y).abs() < min {
//         min = (p.y.floor() - p.y).abs();
//         face = Bottom;
//     }
//     if (p.y.ceil() - p.y).abs() < min {
//         face = Top;
//     }
//     return face;
// }
fn closest_face(p: Vec3) -> Face {
    let faces = [
        ((p.x.floor() - p.x).abs(), Face::Left),
        ((p.x.ceil() - p.x).abs(), Face::Right),
        ((p.y.floor() - p.y).abs(), Face::Bottom),
        ((p.y.ceil() - p.y).abs(), Face::Top),
        ((p.z.floor() - p.z).abs(), Face::Forward),
        ((p.z.ceil() - p.z).abs(), Face::Back),
    ];

    faces
        .iter()
        .cloned()
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(_, face)| face)
        .unwrap_or(Face::Top) // Default face if somehow comparison fails.
}
