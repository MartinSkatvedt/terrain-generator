use std::thread;

use crate::{
    curve_editor::curve::Curve,
    material::Material,
    mesh::{mesh_settings::MeshSettings, Mesh},
    noise_map::noise_map_settings::NoiseMapSettings,
    scenenode::SceneNode,
};

#[derive(Clone)]
pub struct Chunk {
    position: (i32, i32),
    materials: Vec<Material>,
    noise_map_settings: NoiseMapSettings,
    mesh_settings: MeshSettings,

    mesh: Mesh,
    pub vao_id: u32,
}

impl Chunk {
    pub fn new(position: (i32, i32), materials: &Vec<Material>) -> Self {
        let name = format!("Chunk ({}, {}) mesh", position.0, position.1);
        let cubic_curve = Curve::cubic();

        let noise_map_settings = NoiseMapSettings::new();
        let mesh_settings = MeshSettings::new(name, 10.0, cubic_curve, 0);
        let mesh = Mesh::mesh_from_height_map(&materials, &noise_map_settings, &mesh_settings);
        let vao_id = unsafe { mesh.create_vao() };

        Self {
            position,
            materials: materials.clone(),
            noise_map_settings,
            mesh_settings,

            mesh,
            vao_id,
        }
    }

    pub fn request_new_chunk(position: (i32, i32), materials: &Vec<Material>, callback: fn(Chunk)) {
        let name = format!("Chunk ({}, {}) mesh", position.0, position.1);
        let cubic_curve = Curve::cubic();

        let noise_map_settings = NoiseMapSettings::new();
        let mesh_settings = MeshSettings::new(name, 10.0, cubic_curve, 0);
        let material_clone = materials.clone();

        let handle = thread::spawn(move || {
            let mesh =
                Mesh::mesh_from_height_map(&material_clone, &noise_map_settings, &mesh_settings);

            let chunk = Chunk {
                position,
                materials: material_clone,
                noise_map_settings,
                mesh_settings,

                mesh,
                vao_id: 0,
            };

            callback(chunk);
        });
    }

    pub fn update(
        &mut self,
        materials: &Vec<Material>,
        noise_map_settings: &NoiseMapSettings,
        mesh_settings: &MeshSettings,
    ) {
        self.materials = materials.clone();
        self.noise_map_settings = noise_map_settings.clone();
        self.mesh_settings = mesh_settings.clone();

        self.mesh = Mesh::mesh_from_height_map(
            &self.materials,
            &self.noise_map_settings,
            &self.mesh_settings,
        );

        unsafe {
            self.vao_id = self.mesh.create_vao();
        }
    }

    pub fn create_vao(&mut self) {
        unsafe {
            self.vao_id = self.mesh.create_vao();
        }
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
