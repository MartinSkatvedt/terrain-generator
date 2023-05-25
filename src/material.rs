#[derive(Copy, Clone)]
pub struct Material {
    pub ambient: glm::Vec3,
    pub diffuse: glm::Vec3,
    pub specular: glm::Vec3,
    pub shininess: f32,
}

impl Material {
    pub fn new(
        ambient: glm::Vec3,
        diffuse: glm::Vec3,
        specular: glm::Vec3,
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
            ambient: glm::vec3(1.0, 0.7, 0.81),
            diffuse: glm::vec3(1.0, 0.5, 0.31),
            specular: glm::vec3(0.5, 0.5, 0.5),
            shininess: 32.0,
        }
    }
}
