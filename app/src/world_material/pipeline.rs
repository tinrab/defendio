use crate::lighting::uniform::GpuLightingUniform;
use crate::world_material::material::{WorldMaterial, WorldMaterialKey};
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::mesh::{GpuBufferInfo, MeshVertexBufferLayout};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{
    DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult, RenderPhase, SetItemPipeline,
    TrackedRenderPass,
};
use bevy::render::render_resource::{
    AsBindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    Buffer, BufferBindingType, BufferInitDescriptor, BufferUsages, PipelineCache,
    RenderPipelineDescriptor, ShaderRef, ShaderStages, ShaderType, SpecializedMeshPipeline,
    SpecializedMeshPipelineError, SpecializedMeshPipelines, SpecializedRenderPipeline,
    VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::DefaultImageSampler;
use bevy::render::view::ExtractedView;
use bevy::sprite::{
    Material2d, Material2dKey, Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, SetMesh2dBindGroup,
    SetMesh2dViewBindGroup,
};
use std::hash::Hash;

#[derive(Resource)]
pub struct WorldMaterialPipeline {
    pub mesh_pipeline: Mesh2dPipeline,
    pub material_layout: BindGroupLayout,
    pub lighting_layout: BindGroupLayout,
    pub vertex_shader: Option<Handle<Shader>>,
    pub fragment_shader: Option<Handle<Shader>>,
}

impl FromWorld for WorldMaterialPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let render_device = world.resource::<RenderDevice>();
        let mesh_pipeline = world.resource::<Mesh2dPipeline>().clone();

        let material_layout = WorldMaterial::bind_group_layout(&render_device);

        let lighting_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: Some(GpuLightingUniform::min_size()),
                },
                count: None,
            }],
            label: Some("lighting_layout"),
        });

        WorldMaterialPipeline {
            mesh_pipeline,
            material_layout,
            lighting_layout,
            vertex_shader: match WorldMaterial::vertex_shader() {
                ShaderRef::Default => None,
                ShaderRef::Handle(handle) => Some(handle),
                ShaderRef::Path(path) => Some(asset_server.load(path)),
            },
            fragment_shader: match WorldMaterial::fragment_shader() {
                ShaderRef::Default => None,
                ShaderRef::Handle(handle) => Some(handle),
                ShaderRef::Path(path) => Some(asset_server.load(path)),
            },
        }
    }
}

impl SpecializedMeshPipeline for WorldMaterialPipeline {
    type Key = WorldMaterialKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key.mesh_key, layout)?;
        if let Some(vertex_shader) = &self.vertex_shader {
            descriptor.vertex.shader = vertex_shader.clone();
        }
        if let Some(fragment_shader) = &self.fragment_shader {
            descriptor.fragment.as_mut().unwrap().shader = fragment_shader.clone();
        }

        descriptor.layout = vec![
            self.mesh_pipeline.view_layout.clone(),
            self.material_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
            self.lighting_layout.clone(),
        ];

        WorldMaterial::specialize(&mut descriptor, layout, key)?;

        Ok(descriptor)
    }
}
