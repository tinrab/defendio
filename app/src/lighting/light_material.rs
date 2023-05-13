use bevy::{prelude::*,
           reflect::TypeUuid,
           render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy::ecs::query::QueryItem;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{BindGroupLayout, RenderPipelineDescriptor, SpecializedMeshPipelineError, VertexBufferLayout};
use bevy::sprite::{Material2d, Material2dKey};

use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct LightInstanceData {
    pub position: Vec3,
    pub scale: f32,
    pub color: [f32; 4],
}

#[derive(Component, Deref)]
pub struct InstanceMaterialData(pub Vec<LightInstanceData>);

impl ExtractComponent for InstanceMaterialData {
    type Query = &'static InstanceMaterialData;
    type Filter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::Query>) -> Option<Self> {
        Some(InstanceMaterialData(item.0.clone()))
    }
}
