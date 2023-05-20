use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{
    BindGroupLayout, RenderPipelineDescriptor, SpecializedMeshPipelineError, VertexBufferLayout,
};
use bevy::sprite::{Material2d, Material2dKey};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "4284d12f-56dc-49f5-9cc1-68e9d14a7ebc"]
pub struct TilemapMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub lighting_texture: Option<Handle<Image>>,
}

impl Material2d for TilemapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap.wgsl".into()
    }
}
