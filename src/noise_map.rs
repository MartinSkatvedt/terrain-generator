use noise::{NoiseFn, Perlin, Seedable};

pub struct NoiseMap {
    pub strength: f32,
    pub base_roughness: f32,
    pub roughness: f32,
    pub persistence: f32,
    pub center: glm::Vec3,
    pub num_layers: u32,
    pub min_value: f32,
}

impl NoiseMap {
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

    pub fn generate_noise_map(width: u32, height: u32, scale: f32) -> Vec<Vec<f64>> {
        let mut noise_map = vec![vec![0.0; height as usize]; width as usize];

        let perlin = Perlin::new(Perlin::DEFAULT_SEED);

        for y in 0..height {
            for x in 0..width {
                let sample_x = x as f32 / scale;
                let sample_y = y as f32 / scale;

                let noise_value = perlin.get([sample_x as f64, sample_y as f64]);
                noise_map[x as usize][y as usize] = noise_value;
            }
        }

        noise_map
    }
}
