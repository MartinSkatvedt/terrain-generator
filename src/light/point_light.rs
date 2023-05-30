use super::point_light_settings::PointLightSettings;

pub struct PointLight {
    pub position: glm::Vec3,
    pub ambient: glm::Vec3,
    pub diffuse: glm::Vec3,
    pub specular: glm::Vec3,
}

impl PointLight {
    pub fn new(settings: &PointLightSettings) -> PointLight {
        PointLight {
            position: settings.position,
            ambient: settings.ambient,
            diffuse: settings.diffuse,
            specular: settings.specular,
        }
    }
}
