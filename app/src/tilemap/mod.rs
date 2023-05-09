use std::collections::BTreeMap;
use bevy::math::Vec2Swizzles;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::sprite::{DrawMesh2d, MaterialMesh2dBundle};
use crate::asset::CoreAssetSet;
use crate::tilemap::data::ChunkData;
use crate::tilemap::generator::random::RandomTilemapGenerator;
use crate::tilemap::material::TilemapMaterial;

pub mod plugin;
pub mod material;
pub mod generator;
pub mod data;

pub const TILEMAP_CHUNK_SIZE: usize =32;

#[derive(Bundle)]
pub struct TilemapBundle {
    #[bundle]
    obj: MaterialMesh2dBundle<TilemapMaterial>,
}

const QUAD_INDICES: [usize; 6] = [0, 2, 3, 0, 1, 2];

const QUAD_VERTEX_POSITIONS: [Vec2; 4] = [
    Vec2::new(-0.5, -0.5),
    Vec2::new(0.5, -0.5),
    Vec2::new(0.5, 0.5),
    Vec2::new(-0.5, 0.5),
];

const QUAD_UVS: [Vec2; 4] = [
    Vec2::new(0., 1.),
    Vec2::new(1., 1.),
    Vec2::new(1., 0.),
    Vec2::new(0., 0.),
];

impl TilemapBundle {
    pub fn build(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<TilemapMaterial>>,
        images: Res<Assets<Image>>,
        core_asset_set: Res<CoreAssetSet>,
        texture_atlases: Res<Assets<TextureAtlas>>,
    ) -> Self {
        // let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 0.1], [0.0, 0.0, 0.1], [0.0, 0.0, 0.1]]);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 1.0], [1.0, 0.0], [1.0, 1.0]]);
        // mesh.set_indices(Some(Indices::U32(vec![0, 2, 1])));

        // let mut mesh = Mesh::from(shape::Quad::new(Vec2::new(1.0f32, 1.0f32)));

        let tilemap = RandomTilemapGenerator::generate();
        let chunk = tilemap.get_chunk(IVec2::ZERO).unwrap();
        let texture_atlas = texture_atlases.get(&core_asset_set.tiles_atlas).unwrap();
        // // let tiles_image = images.get_handle(&core_asset_set.tiles_atlas.clone_untyped().typed()).unwrap();

        let mesh = make_chunk_mesh(chunk);
        // let mesh = Mesh::from(shape::Quad::new(Vec2::splat(TILEMAP_CHUNK_SIZE as f32)));
        println!("{:#?}", mesh);

        TilemapBundle {
            obj: MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                transform: Transform::default().with_scale(Vec3::splat(100.0f32)),
                material: materials.add(TilemapMaterial {
                    texture: texture_atlas.texture.clone(),
                }),
                ..Default::default()
            }
        }
    }
}


fn make_chunk_mesh(chunk: &ChunkData) -> Mesh {
    let x_vertex_count = TILEMAP_CHUNK_SIZE as u32 + 2;
    let num_vertices = (x_vertex_count * x_vertex_count) as usize;
    let num_indices = ((x_vertex_count - 1) * (x_vertex_count - 1) * 6) as usize;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
    let mut indices: Vec<u32> = Vec::with_capacity(num_indices);


    for y in 0..x_vertex_count {
        for x in 0..x_vertex_count {
            let tx = x as f32 / (x_vertex_count - 1) as f32;
            let ty = y as f32 / (x_vertex_count - 1) as f32;
            positions.push([(-0.5 + tx) * TILEMAP_CHUNK_SIZE as f32,  (-0.5 + ty) * TILEMAP_CHUNK_SIZE as f32,0.0,]);
            normals.push(Vec3::Z.to_array());
            uvs.push([tx, 1.0 - ty]);
        }
    }

    for y in 0..x_vertex_count - 1 {
        for x in 0..x_vertex_count - 1 {
            let quad = y * x_vertex_count + x;
            indices.push(quad + x_vertex_count + 1);
            indices.push(quad + 1);
            indices.push(quad + x_vertex_count);
            indices.push(quad);
            indices.push(quad + x_vertex_count);
            indices.push(quad + 1);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}
