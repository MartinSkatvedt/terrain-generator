pub mod noise_map_settings;
use crate::{material::Material, mesh::Mesh};
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

        let perlin = Perlin::new(Perlin::DEFAULT_SEED);
        let mut max_noise_height = f64::MIN;
        let mut min_noise_height = f64::MAX;

        let half_width = width as f64 / 2.0;
        let half_height = height as f64 / 2.0;

        let clamped_scale = scale.clamp(0.001, 100.0);
        let mut r = StdRng::seed_from_u64(seed as u64);

        let offsets: Vec<[f64; 2]> = (0..octaves)
            .map(|_| {
                let offset_x = r.gen_range(-100000.0..100000.0) + offset_x;
                let offset_y = r.gen_range(-100000.0..100000.0) + offset_y;
                [offset_x, offset_y]
            })
            .collect();

        for y in 0..height {
            for x in 0..width {
                let mut amplitude = 1.0;
                let mut frequency = 1.0;
                let mut noise_height = 0.0;

                for octave in 0..octaves {
                    let sample_x = (x as f64 - half_width) / clamped_scale * frequency
                        + offsets[octave as usize][0];
                    let sample_y = (y as f64 - half_height) / clamped_scale * frequency
                        + offsets[octave as usize][1];

                    let noise_value = perlin.get([sample_x as f64, sample_y as f64]) * 2.0 - 1.0;

                    noise_height += noise_value * amplitude;
                    amplitude *= persistence;
                    frequency *= lacunarity;
                }

                if noise_height > max_noise_height {
                    max_noise_height = noise_height;
                } else if noise_height < min_noise_height {
                    min_noise_height = noise_height;
                }

                noise_map[x as usize][y as usize] = noise_height;
            }
        }
        //Normalize between 0 and 1
        for y in 0..height {
            for x in 0..width {
                noise_map[x as usize][y as usize] = InvLerp::inv_lerp(
                    &noise_map[x as usize][y as usize],
                    &min_noise_height,
                    &max_noise_height,
                );
            }
        }

        NoiseMap { data: noise_map }
    }

    pub fn generate_mesh(&self, materials: &Vec<Material>) -> Mesh {
        let mesh = Mesh::mesh_from_height_map(&self.data, materials);

        mesh
    }
}
