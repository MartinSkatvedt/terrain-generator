pub mod noise_map_settings;
use lininterp::InvLerp;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

use self::noise_map_settings::NoiseMapSettings;
use rand::{Rng, SeedableRng};

pub struct NoiseMap {
    data: Vec<Vec<f64>>,
}

impl NoiseMap {
    pub fn new(settings: NoiseMapSettings) -> NoiseMap {
        let NoiseMapSettings {
            width,
            height,
            scale,
            octaves,
            persistence,
            lacunarity,
            seed,
            offset_x,
            offset_y,
        } = settings;

        let mut noise_map = vec![vec![0.0; height as usize]; width as usize];

        let mut max_possible_height = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;

        let perlin = Perlin::new(seed as u32);
        let mut max_noise_height = f64::MIN;
        let mut min_noise_height = f64::MAX;

        let half_width = width as f64 / 2.0;
        let half_height = height as f64 / 2.0;

        let clamped_scale = scale.clamp(0.001, 100.0);
        let mut r = StdRng::seed_from_u64(seed as u64);

        let mut offsets: Vec<[f64; 2]> = Vec::new();

        for _octave in 0..octaves {
            let r_offset_x = r.gen_range(-100000.0..100000.0) + offset_x;
            let r_offset_y = r.gen_range(-100000.0..100000.0) + offset_y;

            offsets.push([r_offset_x, r_offset_y]);

            max_possible_height += amplitude;
            amplitude *= persistence;
        }

        for y in 0..height {
            for x in 0..width {
                amplitude = 1.0;
                frequency = 1.0;
                let mut noise_height = 0.0;

                for octave in 0..octaves {
                    let sample_x = (x as f64 - half_width + offsets[octave as usize][0])
                        / clamped_scale
                        * frequency;
                    let sample_y = (y as f64 - half_height - offsets[octave as usize][1])
                        / clamped_scale
                        * frequency;

                    let noise_value = perlin.get([sample_x as f64, sample_y as f64]);

                    noise_height += noise_value * amplitude;
                    amplitude *= persistence;
                    frequency *= lacunarity;
                }

                if noise_height > max_noise_height {
                    max_noise_height = noise_height;
                } else if noise_height < min_noise_height {
                    min_noise_height = noise_height;
                }

                noise_map[x as usize][y as usize] =
                    (noise_height + max_possible_height) / (max_possible_height * 2.0);
            }
        }

        NoiseMap { data: noise_map }
    }

    pub fn get_height_map(&self) -> &Vec<Vec<f64>> {
        &self.data
    }
}
