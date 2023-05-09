use std::collections::BTreeMap;
use bevy::prelude::*;
use crate::tilemap::TILEMAP_CHUNK_SIZE;
use array_init::array_init;

#[derive(Debug, Clone)]
pub struct TilemapData {
    pub chunks: BTreeMap<i32, BTreeMap<i32, ChunkData>>,
}

#[derive(Debug, Clone)]
pub struct ChunkData {
    pub tiles: [TileData; TILEMAP_CHUNK_SIZE * TILEMAP_CHUNK_SIZE],
}

#[derive(Debug, Clone, Default)]
pub struct TileData {
    pub id: u16,
    pub color: Option<Color>,
}

impl TilemapData {
    pub fn new() -> Self {
        TilemapData { chunks: Default::default() }
    }

    pub fn get_chunk(&self, location:IVec2) -> Option<&ChunkData> {
        self.chunks.get(&location.x)?.get(&location.y)
    }

    pub fn set_tile(&mut self, location: IVec2, tile: TileData) {
        let chunk_location = Self::tilemap_to_chunk(location);
        let chunk = self.chunks.entry(chunk_location.x)
            .or_default()
            .entry(chunk_location.y).or_insert_with(|| ChunkData::new());
        chunk.set_tile(ChunkData::tilemap_to_chunk_tile(location), tile);
    }

    pub fn tilemap_to_chunk(tile_location: IVec2) -> IVec2 {
        tile_location / TILEMAP_CHUNK_SIZE as i32
    }

    pub fn get_chunk_rect(&self) -> Rect {
        // let mut min = Vec2::new(f32::INFINITY,f32::INFINITY);
        // let mut max = Vec2::new(f32::NEG_INFINITY,f32::NEG_INFINITY);
        let mut min = Vec2::ZERO;
        let mut max = Vec2::ZERO;
        for (x, columns) in self.chunks.iter() {
            for (y, _) in columns.iter() {
                min = min.min(Vec2::new((*x) as f32,(*y) as f32));
                max = max.max(Vec2::new((*x) as f32,(*y) as f32));
            }
        }
        Rect::from_corners(min, max)
    }
}

impl ChunkData {
    pub fn new() -> Self {
        ChunkData { tiles: array_init(|_| TileData::default()) }
    }

    pub fn set_tile(&mut self, location: UVec2, tile: TileData) {
        self.tiles[Self::tile_index(location)] = tile;
    }

    pub fn tile_index(location: UVec2) -> usize {
        location.y as usize + location.x as usize * TILEMAP_CHUNK_SIZE
    }

    pub fn tilemap_to_chunk_tile(location: IVec2) -> UVec2 {
        let x = location.x % TILEMAP_CHUNK_SIZE as i32;
        let y = location.y % TILEMAP_CHUNK_SIZE as i32;
        UVec2::new(if x < 0 { (TILEMAP_CHUNK_SIZE as i32 + x) as u32 } else { x as u32 }, if y < 0 { (TILEMAP_CHUNK_SIZE as i32 + y) as u32 } else { y as u32 })
    }
}

impl TileData {
    pub fn new(id: u16) -> Self {
        TileData {
            id,
            color: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tilemap::generator::random::RandomTilemapGenerator;
    use super::*;

    #[test]
    fn basic() {
        println!("size: {}", std::mem::size_of_val(&RandomTilemapGenerator::generate()));
    }

    #[test]
    fn locations() {
        assert_eq!(ChunkData::tilemap_to_chunk_tile(IVec2::new(40, 45)), UVec2::new(8, 13));
        assert_eq!(ChunkData::tilemap_to_chunk_tile(IVec2::new(-1, -5)), UVec2::new(31, 27));
    }
}
