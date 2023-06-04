use std::{collections::HashMap, thread::JoinHandle};

use crate::{chunk::Chunk, material::Material, scenenode::SceneNode};

pub struct ChunkContainer {
    chunk_size: i32,
    chunks_visible_in_view_dst: i32,
    chunk_map: HashMap<(i32, i32), Chunk>,
    current_visible_chunks: Vec<Chunk>,

    chunks_in_queue: Vec<JoinHandle<Chunk>>,

    default_chunk: Chunk,
}

impl ChunkContainer {
    pub fn new(chunk_size: i32, view_distance: f32, materials: &Vec<Material>) -> Self {
        let chunks_visible_in_view_dst = (view_distance / chunk_size as f32).ceil() as i32;

        Self {
            chunk_size: chunk_size,
            chunks_visible_in_view_dst,
            chunk_map: HashMap::new(),
            current_visible_chunks: Vec::new(),
            chunks_in_queue: Vec::new(),
            default_chunk: Chunk::new((0, 0), &materials),
        }
    }

    pub fn on_chunk_generated(&mut self, chunk: Chunk) {
        println!(
            "Chunk ({}, {}) received",
            chunk.position.0, chunk.position.1
        );

        self.chunk_map.insert(chunk.position, chunk);
    }

    pub fn generate_visible_chunks(
        &mut self,
        camera_position: glm::Vec3,
        materials: &Vec<Material>,
    ) {
        self.current_visible_chunks.clear();

        let current_chunk_coordinates = (
            (camera_position.x / self.chunk_size as f32).floor() as i32,
            (camera_position.z / self.chunk_size as f32).floor() as i32,
        );

        for y_offset in -self.chunks_visible_in_view_dst..=self.chunks_visible_in_view_dst {
            for x_offset in -self.chunks_visible_in_view_dst..=self.chunks_visible_in_view_dst {
                let chunk_coordinates = (
                    current_chunk_coordinates.0 + x_offset,
                    current_chunk_coordinates.1 + y_offset,
                );

                if self.chunk_map.contains_key(&chunk_coordinates) {
                    self.current_visible_chunks
                        .push(self.chunk_map.get(&chunk_coordinates).unwrap().clone());
                } else {
                    let handle = Chunk::request_chunk_generation(chunk_coordinates, &materials);

                    self.chunks_in_queue.push(handle);

                    let mut temp_chunk = self.default_chunk.clone();
                    temp_chunk.position = chunk_coordinates;
                    let temp_chunk_2 = temp_chunk.clone();

                    self.chunk_map.insert(chunk_coordinates, temp_chunk);

                    self.current_visible_chunks.push(temp_chunk_2);
                }
            }
        }
    }

    pub fn update_chunk_map(&mut self) {
        let mut unfinished_threads: Vec<JoinHandle<Chunk>> = Vec::new();

        for handle in self.chunks_in_queue.drain(..) {
            if handle.is_finished() {
                let mut chunk = handle.join().unwrap();
                chunk.rebind_vao();

                self.chunk_map.insert(chunk.position, chunk);
            } else {
                unfinished_threads.push(handle);
            }
        }

        self.chunks_in_queue = unfinished_threads;
    }

    pub fn update_current_visible_chunks(
        &mut self,
        materials: &Vec<Material>,
        noise_map_settings: &crate::noise_map::noise_map_settings::NoiseMapSettings,
        mesh_settings: &crate::mesh::mesh_settings::MeshSettings,
    ) {
        //for chunk in &mut self.arc_current_visible_chunks {
        //    chunk.update(materials, noise_map_settings, mesh_settings);
        //}
    }

    pub fn generate_scene(&mut self, shader_id: u32) -> Vec<SceneNode> {
        let mut scene: Vec<SceneNode> = Vec::new();
        for chunk in self.current_visible_chunks.iter() {
            scene.push(chunk.get_scene_node(shader_id));
        }
        scene
    }
}
