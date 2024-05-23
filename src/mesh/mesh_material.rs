#[derive(Clone)]
pub struct MeshMaterial {
    pub ambient: Vec<f32>,
    pub diffuse: Vec<f32>,
    pub specular: Vec<f32>,
    pub shininess: Vec<f32>,
}
