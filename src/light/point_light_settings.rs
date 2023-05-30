use imgui::{CollapsingHeader, Ui};

use super::point_light::PointLight;

#[derive(Clone)]
pub struct PointLightSettings {
    pub name: String,
    pub position: glm::Vec3,
    pub ambient: glm::Vec3,
    pub diffuse: glm::Vec3,
    pub specular: glm::Vec3,
}

impl PointLightSettings {
    pub fn new(
        name: String,
        position: glm::Vec3,
        ambient: glm::Vec3,
        diffuse: glm::Vec3,
        specular: glm::Vec3,
    ) -> PointLightSettings {
        PointLightSettings {
            name,
            position,
            ambient,
            diffuse,
            specular,
        }
    }

    pub fn get_point_light(&self) -> PointLight {
        PointLight::new(self)
    }

    pub fn render(&mut self, ui: &Ui) {
        if CollapsingHeader::new(&self.name).build(ui) {
            ui.slider("Ambient r", 0.0, 1.0, &mut self.ambient.x);
            ui.slider("Ambient g", 0.0, 1.0, &mut self.ambient.y);
            ui.slider("Ambient b", 0.0, 1.0, &mut self.ambient.z);

            ui.slider("Diffuse r", 0.0, 1.0, &mut self.diffuse.x);
            ui.slider("Diffuse g", 0.0, 1.0, &mut self.diffuse.y);
            ui.slider("Diffuse b", 0.0, 1.0, &mut self.diffuse.z);

            ui.slider("Specular r", 0.0, 1.0, &mut self.specular.x);
            ui.slider("Specular g", 0.0, 1.0, &mut self.specular.y);
            ui.slider("Specular b", 0.0, 1.0, &mut self.specular.z);
        }
    }
}
