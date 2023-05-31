use crate::{
    curve_editor::curve::Curve,
    material::Material,
    mesh::{mesh_settings::MeshSettings, Mesh},
    noise_map::noise_map_settings::NoiseMapSettings,
    scenenode::SceneNode,
};

pub struct Chunk {
    position: (i32, i32),
    materials: Vec<Material>,
    noise_map_settings: NoiseMapSettings,
    mesh_settings: MeshSettings,
}

impl Chunk {
    pub fn new(position: (i32, i32), materials: &Vec<Material>) -> Self {
        let name = format!("Chunk ({}, {}) mesh", position.0, position.1);
        let cubic_curve = Curve::cubic();

        Self {
            position,
            materials: materials.clone(),
            noise_map_settings: NoiseMapSettings::new(),
            mesh_settings: MeshSettings::new(name, 10.0, cubic_curve, 0),
        }
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
    }

    pub fn get_scene_node(&self, shader_id: u32) -> SceneNode {
        let terrain_mesh = Mesh::mesh_from_height_map(
            &self.materials,
            &self.noise_map_settings,
            &self.mesh_settings,
        );
        let terrain_vao = unsafe { terrain_mesh.create_vao() };

        SceneNode {
            vao_id: terrain_vao,
            index_count: terrain_mesh.indices.len() as i32,
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
