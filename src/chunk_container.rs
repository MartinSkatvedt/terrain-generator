use std::{collections::HashMap, rc::Rc, thread::JoinHandle};

use crate::{
    chunk::Chunk, material::Material, mesh::mesh_settings::MeshSettings,
    noise_map::noise_map_settings::NoiseMapSettings, scenenode::SceneNode,
};

pub struct ChunkContainer {
    chunk_size: i32,
    chunks_visible_in_view_dst: i32,
    chunk_map: HashMap<(i32, i32), Rc<Chunk>>,
    current_visible_chunks: Vec<Rc<Chunk>>,

    chunks_in_queue: Vec<JoinHandle<Chunk>>,

    default_chunk: Rc<Chunk>,

    noise_map_settings: NoiseMapSettings,
    mesh_settings: MeshSettings,
    materials: Vec<Material>,
}

impl ChunkContainer {
    pub fn new(
        chunk_size: i32,
        view_distance: f32,
        materials: &Vec<Material>,
        noise_map_settings: &NoiseMapSettings,
        mesh_settings: &MeshSettings,
    ) -> Self {
        let chunks_visible_in_view_dst = (view_distance / chunk_size as f32).round() as i32;

        Self {
            chunk_size,
            chunks_visible_in_view_dst,
            chunk_map: HashMap::new(),
            current_visible_chunks: Vec::new(),
            chunks_in_queue: Vec::new(),
            default_chunk: Rc::new(Chunk::new((0, 0), &materials)),
            materials: materials.clone(),
            noise_map_settings: noise_map_settings.clone(),
            mesh_settings: mesh_settings.clone(),
        }
    }

    pub fn update_settings(
        &mut self,
        materials: &Vec<Material>,
        noise_map_settings: &NoiseMapSettings,
        mesh_settings: &MeshSettings,
    ) {
        self.materials = materials.clone();
        self.noise_map_settings = noise_map_settings.clone();
        self.mesh_settings = mesh_settings.clone();
    }

    pub fn generate_visible_chunks(&mut self, camera_position: glm::Vec3) {
        self.current_visible_chunks.clear();

        let current_chunk_coordinates = (
            (camera_position.x / self.chunk_size as f32).round() as i32,
            (camera_position.z / self.chunk_size as f32).round() as i32,
        );

        for y_offset in -self.chunks_visible_in_view_dst..=self.chunks_visible_in_view_dst {
            for x_offset in -self.chunks_visible_in_view_dst..=self.chunks_visible_in_view_dst {
                let chunk_coordinates = (
                    current_chunk_coordinates.0 + x_offset,
                    current_chunk_coordinates.1 + y_offset,
                );

                if self.chunk_map.contains_key(&chunk_coordinates) {
                    self.current_visible_chunks
                        .push(Rc::clone(self.chunk_map.get(&chunk_coordinates).unwrap()));
                } else {
                    let handle = Chunk::request_chunk_generation(
                        chunk_coordinates,
                        &self.materials,
                        &self.noise_map_settings,
                        &self.mesh_settings,
                    );
                    self.chunks_in_queue.push(handle);

                    self.chunk_map
                        .insert(chunk_coordinates, Rc::clone(&self.default_chunk));

                    self.current_visible_chunks
                        .push(Rc::clone(&self.default_chunk));
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

                self.chunk_map.insert(chunk.position, Rc::new(chunk));
            } else {
                unfinished_threads.push(handle);
            }
        }

        self.chunks_in_queue = unfinished_threads;
    }

    pub fn clear_chunk_container_for_update(&mut self, camera_position: glm::Vec3) {
        let current_chunk_coordinates = (
            (camera_position.x / self.chunk_size as f32).round() as i32,
            (camera_position.z / self.chunk_size as f32).round() as i32,
        );

        for handle in self.chunks_in_queue.drain(..) {
            println!("Chunk generation thread finished");
            handle.join().unwrap();
        }

        self.chunks_in_queue.clear();
        self.current_visible_chunks.clear();
        self.chunk_map.clear();

        let mut new_default_chunk = Chunk::create_chunk(
            current_chunk_coordinates,
            &self.materials,
            &self.noise_map_settings,
            &self.mesh_settings,
        );
        new_default_chunk.rebind_vao();

        self.default_chunk = Rc::new(new_default_chunk);

        self.chunk_map
            .insert(current_chunk_coordinates, Rc::clone(&self.default_chunk));
        self.current_visible_chunks
            .push(Rc::clone(&self.default_chunk));
    }

    pub fn generate_scene(&mut self, shader_id: u32) -> Vec<SceneNode> {
        let mut scene: Vec<SceneNode> = Vec::new();
        for chunk in self.current_visible_chunks.iter() {
            scene.push(chunk.get_scene_node(shader_id));
        }
        scene
    }
}
