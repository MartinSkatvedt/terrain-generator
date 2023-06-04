use std::thread::{self, JoinHandle};

use crate::{
    curve_editor::curve::Curve,
    material::Material,
    mesh::{mesh_settings::MeshSettings, Mesh},
    noise_map::noise_map_settings::NoiseMapSettings,
    scenenode::SceneNode,
};

#[derive(Clone)]
pub struct Chunk {
    pub position: (i32, i32),

    mesh: Mesh,
    pub vao_id: u32,
}

impl Chunk {
    pub fn new(position: (i32, i32), materials: &Vec<Material>) -> Self {
        let name = format!("Chunk ({}, {}) mesh", position.0, position.1);
        let cubic_curve = Curve::cubic();

        let noise_map_settings = NoiseMapSettings::new();
        let mesh_settings = MeshSettings::new(name, 10.0, cubic_curve, 0);
        let mesh = Mesh::create_terrain_mesh(&materials, &noise_map_settings, &mesh_settings);

        Self {
            position,

            mesh,
            vao_id: 0,
        }
    }

    pub fn create_chunk(
        position: (i32, i32),
        materials: &Vec<Material>,
        noise_map_settings: &NoiseMapSettings,
        mesh_settings: &MeshSettings,
    ) -> Self {
        let mesh = Mesh::create_terrain_mesh(materials, noise_map_settings, mesh_settings);

        Self {
            position,

            mesh,
            vao_id: 0,
        }
    }

    pub fn request_chunk_generation(
        position: (i32, i32),
        materials: &Vec<Material>,
        noise_map_settings: &NoiseMapSettings,
        mesh_settings: &MeshSettings,
    ) -> JoinHandle<Chunk> {
        let material_clone = materials.clone();
        let noise_map_settings_clone = noise_map_settings.clone();
        let mesh_settings_clone = mesh_settings.clone();

        thread::spawn(move || {
            let chunk = Chunk::create_chunk(
                position,
                &material_clone,
                &noise_map_settings_clone,
                &mesh_settings_clone,
            );

            chunk
        })
    }

    pub fn rebind_vao(&mut self) {
        self.vao_id = unsafe { self.mesh.create_vao() };
    }

    pub fn get_scene_node(&self, shader_id: u32) -> SceneNode {
        SceneNode {
            vao_id: self.vao_id,
            index_count: self.mesh.indices.len() as i32,
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
