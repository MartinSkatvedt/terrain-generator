extern crate nalgebra_glm as glm;

pub struct SceneNode {
    pub vao_id: u32,
    pub index_count: i32,
    pub shader_program: u32,

    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub scale: glm::Vec3,
    pub reference_point: glm::Vec3,
}
