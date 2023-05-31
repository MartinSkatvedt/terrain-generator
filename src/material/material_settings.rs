use imgui::{CollapsingHeader, Ui};

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

    pub fn standard_water_material() -> Self {
        MaterialSettings {
            name: "Water".to_string(),
            ambient: [0.267, 0.322, 0.722],
            diffuse: [0.267, 0.322, 0.722],
            specular: [1.0, 1.0, 1.0],
            shininess: 16.0,
            height_limit: 0.4,
        }
    }

    pub fn standard_sand_material() -> Self {
        MaterialSettings {
            name: "Sand".to_string(),
            ambient: [0.8, 0.8, 0.4],
            diffuse: [0.8, 0.8, 0.4],
            specular: [0.5, 0.5, 0.5],
            shininess: 2.0,
            height_limit: 0.43,
        }
    }

    pub fn standard_grass_material() -> Self {
        MaterialSettings {
            name: "Grass".to_string(),
            ambient: [0.475, 0.91, 0.455],
            diffuse: [0.475, 0.91, 0.455],
            specular: [0.5, 0.5, 0.5],
            shininess: 2.0,
            height_limit: 0.8,
        }
    }

    pub fn standard_snow_material() -> Self {
        MaterialSettings {
            name: "Snow".to_string(),
            ambient: [1.0, 1.0, 1.0],
            diffuse: [1.0, 1.0, 1.0],
            specular: [1.0, 1.0, 1.0],
            shininess: 32.0,
            height_limit: 1.0,
        }
    }

    pub fn render(&mut self, ui: &Ui) {
        if CollapsingHeader::new(&self.name).build(ui) {
            ui.slider(
                format!("Ambient r##{}", self.name),
                0.0,
                1.0,
                &mut self.ambient[0],
            );
            ui.slider(
                format!("Ambient g##{}", self.name),
                0.0,
                1.0,
                &mut self.ambient[1],
            );
            ui.slider(
                format!("Ambient b##{}", self.name),
                0.0,
                1.0,
                &mut self.ambient[2],
            );

            ui.slider(
                format!("Diffuse r##{}", self.name),
                0.0,
                1.0,
                &mut self.diffuse[0],
            );
            ui.slider(
                format!("Diffuse g##{}", self.name),
                0.0,
                1.0,
                &mut self.diffuse[1],
            );
            ui.slider(
                format!("Diffuse b##{}", self.name),
                0.0,
                1.0,
                &mut self.diffuse[2],
            );

            ui.slider(
                format!("Specular r##{}", self.name),
                0.0,
                1.0,
                &mut self.specular[0],
            );
            ui.slider(
                format!("Specular g##{}", self.name),
                0.0,
                1.0,
                &mut self.specular[1],
            );
            ui.slider(
                format!("Specular b##{}", self.name),
                0.0,
                1.0,
                &mut self.specular[2],
            );
            ui.slider(
                format!("Shininess##{}", self.name),
                1.0,
                256.0,
                &mut self.shininess,
            );

            ui.slider(
                format!("Height limit##{}", self.name),
                0.0,
                1.0,
                &mut self.height_limit,
            );
        }
    }
}
