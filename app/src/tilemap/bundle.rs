use crate::asset::TilemapAssetGroup;
use crate::tilemap::data::ChunkData;
use crate::tilemap::generator::random::RandomTilemapGenerator;
use crate::tilemap::material::TilemapMaterial;
use crate::tilemap::TILEMAP_CHUNK_SIZE;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Bundle)]
pub struct TilemapBundle {
    #[bundle]
    obj: MaterialMesh2dBundle<TilemapMaterial>,
}

impl TilemapBundle {
    pub fn make(
        tilemap_asset_group: Res<TilemapAssetGroup>,
        mut materials: ResMut<Assets<TilemapMaterial>>,
        meshes: &mut Assets<Mesh>,
        images: Res<Assets<Image>>,
        texture_atlases: Res<Assets<TextureAtlas>>,
    ) -> Self {
        let tilemap = RandomTilemapGenerator::generate();
        let chunk = tilemap.get_chunk(IVec2::ZERO).unwrap();
        let texture_atlas = texture_atlases
            .get(&tilemap_asset_group.texture_atlas)
            .unwrap();
        let texture = images.get(&texture_atlas.texture).unwrap();

        let mesh = make_chunk_mesh(chunk, texture_atlas, texture);

        TilemapBundle {
            obj: MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                transform: Transform::default(),
                material: materials.add(TilemapMaterial {
                    color_texture: texture_atlas.texture.clone(),
                    lighting_texture: Default::default(),
                }),
                ..Default::default()
            },
        }
    }
}

const QUAD_VERTEX_POSITIONS: [Vec2; 4] = [
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(0.0, 1.0),
];
const QUAD_INDICES: [u32; 6] = [0, 2, 3, 0, 1, 2];
const QUAD_UVS: [Vec2; 4] = [
    Vec2::new(0., 1.),
    Vec2::new(1., 1.),
    Vec2::new(1., 0.),
    Vec2::new(0., 0.),
];

fn make_chunk_mesh(chunk: &ChunkData, texture_atlas: &TextureAtlas, texture: &Image) -> Mesh {
    const DEFAULT_CAPACITY: usize = TILEMAP_CHUNK_SIZE as usize * 4;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(DEFAULT_CAPACITY);
    // let mut normals: Vec<[f32; 3]> = Vec::with_capacity(DEFAULT_CAPACITY);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(DEFAULT_CAPACITY);
    let mut indices: Vec<u32> = Vec::with_capacity(DEFAULT_CAPACITY);

    let mut stride = 0u32;
    for y in 0..TILEMAP_CHUNK_SIZE {
        for x in 0..TILEMAP_CHUNK_SIZE {
            let tile = chunk.get_tile_at(x, y);

            positions.extend(QUAD_VERTEX_POSITIONS.map(|p| [p.x + x as f32, p.y + y as f32, 0.0]));

            let rect = texture_atlas.textures[tile.atlas_index];
            let uv_min = rect.min / texture.size();
            let uv_max = rect.max / texture.size();
            uvs.extend([
                [uv_min.x, uv_max.y],
                [uv_max.x, uv_max.y],
                [uv_max.x, uv_min.y],
                [uv_min.x, uv_min.y],
            ]);

            indices.extend(QUAD_INDICES.map(|i| i + stride));
            stride += 4;
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}
