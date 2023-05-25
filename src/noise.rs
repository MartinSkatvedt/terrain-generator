pub struct Noise {
    pub strength: f32,
    pub base_roughness: f32,
    pub roughness: f32,
    pub persistence: f32,
    pub center: glm::Vec3,
    pub num_layers: u32,
    pub min_value: f32,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            strength: 1.0,
            base_roughness: 1.0,
            roughness: 2.0,
            persistence: 0.5,
            center: glm::vec3(0.0, 0.0, 0.0),
            num_layers: 1,
            min_value: 0.0,
        }
    }
}
