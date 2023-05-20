use crate::lighting::uniform::LightingUniformBuffer;
use crate::world_material::material::{WorldMaterial, WorldMaterialKey};
use crate::world_material::pipeline::WorldMaterialPipeline;
use crate::world_material::render_command::DrawWorldMaterial;
use bevy::core_pipeline::core_2d::Transparent2d;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::ecs::query::ROQueryItem;
use bevy::ecs::system::lifetimeless::{Read, SRes};
use bevy::ecs::system::SystemParamItem;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{
    DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult, RenderPhase, TrackedRenderPass,
};
use bevy::render::render_resource::{
    AsBindGroup, AsBindGroupError, BindGroup, BindGroupDescriptor, BindGroupEntry, PipelineCache,
    SpecializedMeshPipelines,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::FallbackImage;
use bevy::render::view::{ExtractedView, VisibleEntities};
use bevy::render::Extract;
use bevy::sprite::{
    Material2d, Material2dKey, Material2dPipeline, Mesh2dHandle, Mesh2dPipelineKey, Mesh2dUniform,
    PreparedMaterial2d, RenderMaterials2d,
};
use bevy::utils::{FloatOrd, HashSet};

#[derive(Component)]
pub struct LightingBindGroup {
    pub value: BindGroup,
}

pub fn queue_lighting_bind_groups(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    pipeline: Res<WorldMaterialPipeline>,
    views: Query<Entity, With<ExtractedView>>,
    lighting_buffer: Res<LightingUniformBuffer>,
) {
    let lighting_binding = if let Some(binding) = lighting_buffer.buffer.binding() {
        binding
    } else {
        return;
    };
    for entity in &views {
        let lighting_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: lighting_binding.clone(),
            }],
            label: Some("lighting_bind_group"),
            layout: &pipeline.lighting_layout,
        });

        commands.entity(entity).insert(LightingBindGroup {
            value: lighting_bind_group,
        });
    }
}

pub struct SetWorldMaterialLightingBindGroup<const I: usize>;
impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetWorldMaterialLightingBindGroup<I> {
    type Param = ();
    type ViewWorldQuery = Read<LightingBindGroup>;
    type ItemWorldQuery = ();

    fn render<'w>(
        _item: &P,
        lighting_bind_group: ROQueryItem<'w, Self::ViewWorldQuery>,
        _view: (),
        _param: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        pass.set_bind_group(I, &lighting_bind_group.value, &[]);
        RenderCommandResult::Success
    }
}
