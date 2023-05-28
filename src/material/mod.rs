use self::material_settings::MaterialSettings;

pub mod material_settings;

#[derive(Clone, Copy)]
pub struct Material {
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub shininess: f32,
    pub height_limit: f32,
}

impl Material {
    pub fn new(settings: &MaterialSettings) -> Material {
        Material {
            ambient: settings.ambient,
            diffuse: settings.diffuse,
            specular: settings.specular,
            shininess: settings.shininess,
            height_limit: settings.height_limit,
        }
    }

    pub fn standard_material() -> Material {
        Material {
            ambient: [1.0, 0.7, 0.81],
            diffuse: [1.0, 0.5, 0.31],
            specular: [0.5, 0.5, 0.5],
            shininess: 32.0,
            height_limit: 0.5,
        }
    }
}
