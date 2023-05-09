use bracket_noise::prelude::{FastNoise, FractalType, Interp, NoiseType};
use bevy::prelude::*;
use crate::tilemap::data::{TileData, TilemapData};

pub struct RandomTilemapGenerator {}

impl RandomTilemapGenerator {
    pub fn generate() -> TilemapData {
        let mut noise = FastNoise::seeded(rand::random());
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::Billow);
        noise.set_interp(Interp::Quintic);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(2.0);

        let mut tilemap = TilemapData::new();

        for y in -10..=10 {
            for x in -10..=10 {
                let n = noise.get_noise((x as f32) / 100.0 , (y as f32) / 100.0);
                let id = (n.abs() * 5.0) as u16;
                tilemap.set_tile(IVec2::new(x, y), TileData::new(id));
            }
        }

        tilemap
    }
}
