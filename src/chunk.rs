use std::thread::{self, JoinHandle};

use crate::{
    lod::LevelOfDetailInfo,
    material::Material,
    mesh::{self, mesh_settings::MeshSettings, Mesh},
    noise_map::noise_map_settings::NoiseMapSettings,
    scenenode::SceneNode,
};

#[derive(Clone)]
pub struct Chunk {
    pub position: (i32, i32),

    meshes: Vec<Mesh>,
    pub lod_index: usize,
}

impl Chunk {
    pub fn create_chunk(
        position: (i32, i32),
        materials: &Vec<Material>,
        noise_map_settings: &NoiseMapSettings,
        mesh_settings: &MeshSettings,
        level_of_details: &Vec<LevelOfDetailInfo>,
    ) -> Self {
        let mut meshes = Vec::new();

        let mut adjusted_noise_map_settings = noise_map_settings.clone();
        adjusted_noise_map_settings.offset_x = position.0 as f64 * 240.0;
        adjusted_noise_map_settings.offset_y = position.1 as f64 * 240.0;

        for lod in level_of_details {
            let mut adjusted_mesh_settings = mesh_settings.clone();
            adjusted_mesh_settings.level_of_detail = lod.lod as i32;

            meshes.push(Mesh::create_terrain_mesh(
                materials,
                &adjusted_noise_map_settings,
                &adjusted_mesh_settings,
            ))
        }

        Self {
            position,
            meshes,
            lod_index: 0,
        }
    }

    pub fn request_chunk_generation(
        position: (i32, i32),
        materials: &Vec<Material>,
        noise_map_settings: &NoiseMapSettings,
        mesh_settings: &MeshSettings,
        level_of_details: &Vec<LevelOfDetailInfo>,
    ) -> JoinHandle<Chunk> {
        let material_clone = materials.clone();
        let noise_map_settings_clone = noise_map_settings.clone();
        let mesh_settings_clone = mesh_settings.clone();
        let level_of_details_clone = level_of_details.to_vec();

        thread::spawn(move || {
            let chunk = Chunk::create_chunk(
                position,
                &material_clone,
                &noise_map_settings_clone,
                &mesh_settings_clone,
                &level_of_details_clone,
            );

            chunk
        })
    }

    pub fn rebind_vaos(&mut self) {
        for mesh in &mut self.meshes {
            unsafe { mesh.create_vao() };
        }
    }

    pub fn get_scene_node(&self, shader_id: u32, lod: usize) -> SceneNode {
        let mesh_to_use = &self.meshes[lod];

        SceneNode {
            vao_id: mesh_to_use.vao_id,
            index_count: mesh_to_use.index_count,
            shader_program: shader_id,

            position: glm::vec3(
                self.position.0 as f32 * 240.0,
                0.0,
                self.position.1 as f32 * 240.0,
            ),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
            reference_point: glm::vec3(0.0, 0.0, 0.0),
        }
    }
}
