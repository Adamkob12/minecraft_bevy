use crate::{chunk::*, Block, BlockMaterial, BlockRegistry, Perlin, GEN_SEED};
use bevy::utils::hashbrown::HashMap;
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_meshem::prelude::*;
use noise::NoiseFn;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Component)]
pub struct ComputeChunk(pub Task<Option<((Mesh, MeshMD<Block>), [Block; CHUNK_LEN], [i32; 2])>>);

#[derive(Resource, Default)]
pub struct ChunkQueue {
    // true = spawn, false= despawn
    queue: Vec<([i32; 2], bool)>,
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
        assert_eq!(self.pos_to_ent.remove(&cords).unwrap(), ent);
    }
}

impl ChunkQueue {
    pub fn queue_spawn(&mut self, pos: [i32; 2]) {
        self.queue.push((pos, true));
    }

    pub fn queue_despawn(&mut self, pos: [i32; 2]) {
        self.queue.push((pos, false));
    }

    pub fn dequeue_all(
        &mut self,
        mut commands: Commands,
        breg: BlockRegistry,
        chunk_map: &mut ChunkMap,
    ) {
        let breg = Arc::new(breg);
        let noise = Perlin::new(GEN_SEED);
        let thread_pool = AsyncComputeTaskPool::get();
        for chunk in self.queue.as_slice() {
            if !chunk.1 {
                let ent = commands
                    .entity(
                        chunk_map
                            .get_ent(chunk.0)
                            .expect("Couldn't find chunk entity in Chunk Map"),
                    )
                    .id();
                chunk_map.remove_ent(chunk.0, ent);
                commands.entity(ent).despawn();
            }

            let breg = Arc::clone(&breg);
            let cords = chunk.0.clone();
            let task = thread_pool.spawn(async move {
                println!("doing stuff");
                let grid = generate_chunk(cords, &noise);
                let t = mesh_grid(CHUNK_DIMS, grid.to_vec(), &*breg, MeshingAlgorithm::Culling)?;
                Some((t, grid, cords))
            });
            println!("did stuff?");

            commands.spawn(ComputeChunk(task));
        }
        self.queue.clear();
    }
}
