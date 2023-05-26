#[derive(Clone, Copy)]
pub struct Material {
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub shininess: f32,
}

impl Material {
    pub fn new(
        ambient: [f32; 3],
        diffuse: [f32; 3],
        specular: [f32; 3],
        shininess: f32,
    ) -> Material {
        Material {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn standard_material() -> Material {
        Material {
            ambient: [1.0, 0.7, 0.81],
            diffuse: [1.0, 0.5, 0.31],
            specular: [0.5, 0.5, 0.5],
            shininess: 32.0,
        }
    }
}
