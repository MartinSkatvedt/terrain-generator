use std::{collections::HashMap, rc::Rc, thread::JoinHandle};

use crate::{
    lod::LevelOfDetailInfo, material::Material, mesh::mesh_settings::MeshSettings,
    noise_map::noise_map_settings::NoiseMapSettings, scenenode::SceneNode,
};

use self::chunk::Chunk;
pub mod chunk;

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

    detail_levels: Vec<LevelOfDetailInfo>,
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

        let detail_levels = vec![
            LevelOfDetailInfo::new(0, 200.0),
            LevelOfDetailInfo::new(1, 300.0),
            LevelOfDetailInfo::new(2, 400.0),
            LevelOfDetailInfo::new(4, 600.0),
        ];

        Self {
            chunk_size,
            chunks_visible_in_view_dst,
            chunk_map: HashMap::new(),
            current_visible_chunks: Vec::new(),
            chunks_in_queue: Vec::new(),
            default_chunk: Rc::new(Chunk::create_chunk(
                (0, 0),
                &materials,
                &noise_map_settings,
                &mesh_settings,
                &detail_levels,
            )),
            materials: materials.clone(),
            noise_map_settings: noise_map_settings.clone(),
            mesh_settings: mesh_settings.clone(),
            detail_levels,
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
                        &self.detail_levels,
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

                chunk.rebind_vaos();

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
            &self.detail_levels,
        );
        new_default_chunk.rebind_vaos();

        self.default_chunk = Rc::new(new_default_chunk);

        self.chunk_map
            .insert(current_chunk_coordinates, Rc::clone(&self.default_chunk));
        self.current_visible_chunks
            .push(Rc::clone(&self.default_chunk));
    }

    pub fn generate_scene(&mut self, shader_id: u32, camera_position: glm::Vec3) -> Vec<SceneNode> {
        let mut scene: Vec<SceneNode> = Vec::new();
        for chunk in self.current_visible_chunks.iter() {
            let chunk_world_position = glm::vec3(
                chunk.position.0 as f32 * self.chunk_size as f32,
                0.0,
                chunk.position.1 as f32 * self.chunk_size as f32,
            );

            let distance_to_chunk = glm::distance(&camera_position, &chunk_world_position);

            let mut lod_index = 0;
            for (index, detail_level) in self.detail_levels.iter().enumerate() {
                if distance_to_chunk > detail_level.distance {
                    lod_index = index;
                }
            }

            scene.push(chunk.get_scene_node(shader_id, lod_index));
        }
        scene
    }
}
