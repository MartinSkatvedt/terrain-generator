use std::{collections::HashMap, os::windows::thread};

use crate::{chunk::Chunk, material::Material, scenenode::SceneNode};

pub struct ChunkContainer {
    chunk_size: i32,
    chunks_visible_in_view_dst: i32,
    chunk_map: HashMap<(i32, i32), Chunk>,
    current_visible_chunks: Vec<Chunk>,
}

impl ChunkContainer {
    pub fn new(chunk_size: i32, view_distance: f32) -> Self {
        let chunks_visible_in_view_dst = (view_distance / chunk_size as f32).ceil() as i32;

        Self {
            chunk_size: chunk_size,
            chunks_visible_in_view_dst,
            chunk_map: HashMap::new(),
            current_visible_chunks: Vec::new(),
        }
    }

    pub fn generate_visible_chunks(
        &mut self,
        camera_position: glm::Vec3,
        materials: &Vec<Material>,
    ) {
        // self.current_visible_chunks.clear();

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
                    //
                } else {
                    let chunk = Chunk::new(chunk_coordinates, materials);
                    self.chunk_map.insert(chunk_coordinates, chunk);

                    println!(
                        "Chunk ({}, {}) generated",
                        chunk_coordinates.0, chunk_coordinates.1
                    );

                    self.current_visible_chunks
                        .push(self.chunk_map.get(&chunk_coordinates).unwrap().clone());
                }
            }
        }
    }

    pub fn update_current_visible_chunks(
        &mut self,
        materials: &Vec<Material>,
        noise_map_settings: &crate::noise_map::noise_map_settings::NoiseMapSettings,
        mesh_settings: &crate::mesh::mesh_settings::MeshSettings,
    ) {
        for chunk in &mut self.current_visible_chunks {
            chunk.update(materials, noise_map_settings, mesh_settings);
        }
    }

    pub fn generate_scene(&self, shader_id: u32) -> Vec<SceneNode> {
        self.current_visible_chunks
            .iter()
            .map(|c| c.get_scene_node(shader_id))
            .collect()
    }
}
