use crate::{chunk::*, Block, BlockRegistry, Perlin, GEN_SEED};
use bevy::utils::hashbrown::HashMap;
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_meshem::prelude::*;
use std::sync::Arc;

#[derive(Component)]
pub struct ComputeChunk(pub Task<Option<((Mesh, MeshMD<Block>), [Block; CHUNK_LEN], [i32; 2])>>);

enum QdChunk {
    Spawn,
    Despawn,
}

#[derive(Resource, Default)]
pub struct ChunkQueue {
    // true = spawn, false= despawn
    queue: Vec<([i32; 2], QdChunk)>,
    pub panic_when_cant_find_chunk: bool,
}

#[derive(Resource, Default)]
pub struct ChunkMap {
    pos_to_ent: HashMap<[i32; 2], Entity>,
}

impl ChunkMap {
    pub fn get_ent(&self, cords: [i32; 2]) -> Option<Entity> {
        Some(*self.pos_to_ent.get(&cords)?)
    }

    pub fn insert_ent(&mut self, cords: [i32; 2], ent: Entity) {
        self.pos_to_ent.insert(cords, ent);
    }

    pub fn remove_ent(&mut self, cords: [i32; 2], ent: Entity) {
        assert_eq!(
            self.pos_to_ent.remove(&cords).unwrap(),
            ent,
            "Tried removing entity associated with chunk, but the entity given was wrong"
        );
    }

    pub fn exists(&self, cords: [i32; 2]) -> bool {
        self.pos_to_ent.contains_key(&cords)
    }

    pub fn iter_keys(&self) -> bevy::utils::hashbrown::hash_map::Keys<'_, [i32; 2], Entity> {
        self.pos_to_ent.keys()
    }

    pub fn iter(&self) -> bevy::utils::hashbrown::hash_map::Iter<'_, [i32; 2], Entity> {
        self.pos_to_ent.iter()
    }

    pub fn change_ent(&mut self, cords: [i32; 2], ent: Entity) {
        *(self
            .pos_to_ent
            .get_mut(&cords)
            .expect("Couldn't find chunk entity")) = ent;
    }
}

impl ChunkQueue {
    pub fn queue_spawn(&mut self, pos: [i32; 2]) {
        self.queue.push((pos, QdChunk::Spawn));
    }

    pub fn queue_despawn(&mut self, pos: [i32; 2]) {
        self.queue.push((pos, QdChunk::Despawn));
    }

    // Dequeue all the pending chunks to spawn / despawn.
    pub fn dequeue_all(
        &mut self,
        mut commands: Commands,
        breg: Arc<BlockRegistry>,
        chunk_map: &mut ChunkMap,
    ) {
        if self.queue.is_empty() {
            return;
        }

        let noise = Perlin::new(GEN_SEED);
        let thread_pool = AsyncComputeTaskPool::get();
        for chunk in self.queue.as_slice() {
            match chunk.1 {
                // Despawn chunk by just despawning it's entity, and removing it
                // from the ChunkMap data structure.
                QdChunk::Despawn => {
                    let ent;
                    if let Some(e) = chunk_map.get_ent(chunk.0) {
                        if e != Entity::PLACEHOLDER {
                            ent = commands.entity(e).id();
                        } else {
                            continue;
                        }
                    } else {
                        assert!(!self.panic_when_cant_find_chunk, "Couldn't find chunk");
                        continue;
                    }
                    chunk_map.remove_ent(chunk.0, ent);
                    commands.entity(ent).despawn();
                }

                // Spawn chunk by first inserting a placeholder entity to the ChunkMap so it won't
                // be added again while the thread is still computing the mesh.
                // Spawn a thread that computes the mesh etc. everything we need to spawn the
                // chunk.
                // `handle_tasks` picks up the resault of the thread when it finishes, and spawns
                // it.
                QdChunk::Spawn => {
                    let task;

                    if chunk_map.exists(chunk.0) {
                        assert!(
                            !self.panic_when_cant_find_chunk,
                            "Can't spawn chunk that is already spawned."
                        );
                        continue;
                    }

                    chunk_map.insert_ent(chunk.0, Entity::PLACEHOLDER);
                    let breg = Arc::clone(&breg);
                    let cords = chunk.0;
                    task = thread_pool.spawn(async move {
                        let grid = generate_chunk(cords, &noise);
                        let t = mesh_grid(
                            CHUNK_DIMS,
                            &[Bottom],
                            &grid,
                            &*breg,
                            MeshingAlgorithm::Culling,
                            Some(PbsParameters {
                                pbs_value: 0.12,
                                pbs_smoothing: 0.5,
                            }),
                        )?;
                        Some((t, grid, cords))
                    });
                    commands.spawn(ComputeChunk(task));
                }
            }
        }
        self.queue.clear();
    }
}
