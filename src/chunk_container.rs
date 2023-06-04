use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use crate::{chunk::Chunk, material::Material, scenenode::SceneNode};

pub struct ChunkContainer {
    chunk_size: i32,
    chunks_visible_in_view_dst: i32,
    arc_chunk_map: Arc<Mutex<HashMap<(i32, i32), Chunk>>>,
    arc_current_visible_chunks: Arc<Mutex<Vec<Chunk>>>,
    running_chunk_generation_threads: Vec<Option<thread::JoinHandle<()>>>,
}

impl ChunkContainer {
    pub fn wait_for_all_threads_to_finish(&mut self) {
        for thread in self.running_chunk_generation_threads.iter_mut() {
            if thread.is_some() {
                thread.take().unwrap().join().unwrap();
            }
        }
    }

    pub fn new(chunk_size: i32, view_distance: f32) -> Self {
        let chunks_visible_in_view_dst = (view_distance / chunk_size as f32).ceil() as i32;

        Self {
            chunk_size: chunk_size,
            chunks_visible_in_view_dst,
            arc_chunk_map: Arc::new(Mutex::new(HashMap::new())),
            arc_current_visible_chunks: Arc::new(Mutex::new(Vec::new())),
            running_chunk_generation_threads: Vec::<Option<thread::JoinHandle<()>>>::new(),
        }
    }

    pub fn generate_visible_chunks(
        &mut self,
        camera_position: glm::Vec3,
        materials: &Vec<Material>,
    ) {
        let arc_current_visible_chunks_clone_1 = Arc::clone(&self.arc_current_visible_chunks);

        if let Ok(mut current_visible_chunks) = self.arc_current_visible_chunks.lock() {
            current_visible_chunks.clear();
        }

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

                let mut is_new_chunk = true;

                if let Ok(chunk_map) = self.arc_chunk_map.lock() {
                    if chunk_map.contains_key(&chunk_coordinates) {
                        is_new_chunk = false;

                        if let Ok(mut current_visible_chunks) =
                            arc_current_visible_chunks_clone_1.lock()
                        {
                            current_visible_chunks
                                .push(chunk_map.get(&chunk_coordinates).unwrap().clone());
                        }
                    }
                }

                if is_new_chunk {
                    let arc_chunk_map_clone = Arc::clone(&self.arc_chunk_map);
                    let arc_current_visible_chunks_clone =
                        Arc::clone(&self.arc_current_visible_chunks);

                    let materials_clone = materials.clone();
                    let handle = thread::spawn(move || {
                        let chunk = Chunk::new(chunk_coordinates, &materials_clone);

                        println!(
                            "Chunk ({}, {}) generated",
                            chunk_coordinates.0, chunk_coordinates.1
                        );
                        let chunk_clone = chunk.clone();
                        if let Ok(mut chunk_map) = arc_chunk_map_clone.lock() {
                            chunk_map.insert(chunk_coordinates, chunk);
                            println!(
                                "Chunk ({}, {}) inserted into chunk map",
                                chunk_coordinates.0, chunk_coordinates.1
                            );
                        }

                        if let Ok(mut current_visible_chunks) =
                            arc_current_visible_chunks_clone.lock()
                        {
                            current_visible_chunks.push(chunk_clone);
                        }
                    });
                    self.running_chunk_generation_threads.push(Some(handle));
                }
            }
            self.wait_for_all_threads_to_finish();
        }
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

    pub fn rebind_vaos(&mut self) {
        if let Ok(mut current_visible_chunks) = self.arc_current_visible_chunks.lock() {
            for chunk in current_visible_chunks.iter_mut() {
                chunk.create_vao();
            }
        }
    }

    pub fn generate_scene(&self, shader_id: u32) -> Vec<SceneNode> {
        let mut scene: Vec<SceneNode> = Vec::new();
        if let Ok(mut current_visible_chunks) = self.arc_current_visible_chunks.lock() {
            for chunk in current_visible_chunks.iter_mut() {
                if chunk.vao_id == 0 {
                    chunk.create_vao();
                }
                scene.push(chunk.get_scene_node(shader_id));
            }
        }
        scene
    }
}
