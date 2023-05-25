use crate::material::Material;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub material: Material,
}


impl Vertex {
    pub fn new(position: glm::Vec3, material: Material) -> Vertex {
        Vertex { position, material }
    }
}


