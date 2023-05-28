#[derive(Clone, PartialEq)]
pub struct MaterialSettings {
    pub name: String,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub shininess: f32,
    pub height_limit: f32,
}

impl MaterialSettings {
    pub fn new(
        name: String,
        ambient: [f32; 3],
        diffuse: [f32; 3],
        specular: [f32; 3],
        shininess: f32,
        height_limit: f32,
    ) -> MaterialSettings {
        MaterialSettings {
            name,
            ambient,
            diffuse,
            specular,
            shininess,
            height_limit,
        }
    }

    pub fn standard_material_settings() -> MaterialSettings {
        MaterialSettings {
            name: "Standard".to_string(),
            ambient: [1.0, 0.7, 0.81],
            diffuse: [1.0, 0.5, 0.31],
            specular: [0.5, 0.5, 0.5],
            shininess: 32.0,
            height_limit: 0.5,
        }
    }
}
