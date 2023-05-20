use crate::world_material::bind_groups::SetWorldMaterialLightingBindGroup;
use crate::world_material::material::WorldMaterial;
use bevy::ecs::query::ROQueryItem;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::{SystemParamItem, SystemState};
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::mesh::{GpuBufferInfo, MeshVertexBufferLayout};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{
    DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult, RenderPhase, SetItemPipeline,
    TrackedRenderPass,
};
use bevy::render::render_resource::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer,
    BufferBindingType, BufferInitDescriptor, BufferUsages, PipelineCache, RenderPipelineDescriptor,
    ShaderStages, SpecializedMeshPipeline, SpecializedMeshPipelineError, SpecializedMeshPipelines,
    SpecializedRenderPipeline, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::DefaultImageSampler;
use bevy::render::view::ExtractedView;
use bevy::sprite::{
    DrawMesh2d, Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, SetMaterial2dBindGroup,
    SetMesh2dBindGroup, SetMesh2dViewBindGroup,
};

pub type DrawWorldMaterial = (
    SetItemPipeline,
    SetMesh2dViewBindGroup<0>,
    SetMaterial2dBindGroup<WorldMaterial, 1>,
    SetMesh2dBindGroup<2>,
    SetWorldMaterialLightingBindGroup<3>,
    DrawMesh2d,
    // DrawWorldMaterialCommand,
);

// pub struct DrawWorldMaterialCommand;
// impl<P: PhaseItem> RenderCommand<P> for DrawWorldMaterialCommand {
//     type Param = SRes<RenderAssets<Mesh>>;
//     type ViewWorldQuery = ();
//     type ItemWorldQuery = ();
//
//     fn render<'w>(
//         item: &P,
//         view: ROQueryItem<'w, Self::ViewWorldQuery>,
//         entity: ROQueryItem<'w, Self::ItemWorldQuery>,
//         param: SystemParamItem<'w, '_, Self::Param>,
//         pass: &mut TrackedRenderPass<'w>,
//     ) -> RenderCommandResult {
//         todo!();
//         RenderCommandResult::Success
//     }
// }
