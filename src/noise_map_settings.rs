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
            width: 40,
            height: 40,
            scale: 20.0,
            octaves: 5,
            persistence: 0.5,
            lacunarity: 2.0,
            seed: 0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}
