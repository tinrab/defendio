use crate::world_material::bind_groups::queue_lighting_bind_groups;
use crate::world_material::extract::{
    extract_materials, prepare_materials, queue_meshes, ExtractedMaterials2d,
};
use crate::world_material::material::WorldMaterial;
use crate::world_material::pipeline::WorldMaterialPipeline;
use crate::world_material::render_command::DrawWorldMaterial;
use bevy::asset::load_internal_asset;
use bevy::core_pipeline::core_2d::Transparent2d;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::extract_component::ExtractComponentPlugin;
use bevy::render::render_asset::PrepareAssetSet;
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::render_resource::{SpecializedMeshPipelines, SpecializedRenderPipelines};
use bevy::render::{RenderApp, RenderSet};
use bevy::sprite::{Material2dPlugin, RenderMaterials2d};

pub struct WorldMaterialPlugin;

pub const WORLD_MATERIAL_TYPES_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2_000_000);
pub const WORLD_MATERIAL_BINDINGS_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2_000_001);

impl Plugin for WorldMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            WORLD_MATERIAL_TYPES_SHADER_HANDLE,
            "assets/world_material_types.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            WORLD_MATERIAL_BINDINGS_SHADER_HANDLE,
            "assets/world_material_bindings.wgsl",
            Shader::from_wgsl
        );

        app.add_asset::<WorldMaterial>()
            .add_plugin(ExtractComponentPlugin::<Handle<WorldMaterial>>::extract_visible());

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .add_render_command::<Transparent2d, DrawWorldMaterial>()
                .init_resource::<WorldMaterialPipeline>()
                .init_resource::<SpecializedMeshPipelines<WorldMaterialPipeline>>()
                .init_resource::<ExtractedMaterials2d<WorldMaterial>>()
                .init_resource::<RenderMaterials2d<WorldMaterial>>()
                .add_system(extract_materials.in_schedule(ExtractSchedule))
                .add_system(
                    prepare_materials
                        .in_set(RenderSet::Prepare)
                        .after(PrepareAssetSet::PreAssetPrepare),
                )
                .add_system(queue_meshes.in_set(RenderSet::Queue))
                .add_system(queue_lighting_bind_groups.in_set(RenderSet::Queue));
        }
    }
}
