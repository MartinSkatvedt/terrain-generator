use imgui::Ui;

use crate::CHUNK_PIXEL_SIZE;
#[derive(Clone, Copy, PartialEq)]
pub struct NoiseMapSettings {
    pub width: i32,
    pub height: i32,
    pub scale: f64,
    pub octaves: i32,
    pub persistence: f64,
    pub lacunarity: f64,
    pub seed: i32,
    pub offset_x: f64,
    pub offset_y: f64,
}

impl NoiseMapSettings {
    pub fn new() -> NoiseMapSettings {
        NoiseMapSettings {
            width: CHUNK_PIXEL_SIZE + 1,
            height: CHUNK_PIXEL_SIZE + 1,
            scale: 20.0,
            octaves: 5,
            persistence: 0.5,
            lacunarity: 2.0,
            seed: 0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    pub fn render(&mut self, ui: &Ui) {
        ui.slider("Scale", 0.0, 100.0, &mut self.scale);
        ui.slider("Octaves", 0, 20, &mut self.octaves);
        ui.slider("Persistance", 0.0, 1.0, &mut self.persistence);
        ui.slider("Lacunarity", 1.0, 10.0, &mut self.lacunarity);
        ui.slider("Seed", -100, 100, &mut self.seed);

        ui.slider("Offset x", -10.0, 10.0, &mut self.offset_x);

        ui.slider("Offset y", -10.0, 10.0, &mut self.offset_y);
    }
}
