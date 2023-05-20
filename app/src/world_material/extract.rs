use crate::world_material::material::{WorldMaterial, WorldMaterialKey};
use crate::world_material::pipeline::WorldMaterialPipeline;
use crate::world_material::render_command::DrawWorldMaterial;
use bevy::core_pipeline::core_2d::Transparent2d;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{DrawFunctions, RenderPhase};
use bevy::render::render_resource::{
    AsBindGroup, AsBindGroupError, PipelineCache, SpecializedMeshPipelines,
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

pub struct PrepareNextFrameMaterials<M: Material2d> {
    assets: Vec<(Handle<M>, M)>,
}

impl<M: Material2d> Default for PrepareNextFrameMaterials<M> {
    fn default() -> Self {
        Self {
            assets: Default::default(),
        }
    }
}

#[derive(Resource)]
pub struct ExtractedMaterials2d<M: Material2d> {
    extracted: Vec<(Handle<M>, M)>,
    removed: Vec<Handle<M>>,
}

impl<M: Material2d> Default for ExtractedMaterials2d<M> {
    fn default() -> Self {
        Self {
            extracted: Default::default(),
            removed: Default::default(),
        }
    }
}

pub fn extract_materials(
    mut commands: Commands,
    mut event_reader: Extract<EventReader<AssetEvent<WorldMaterial>>>,
    assets: Extract<Res<Assets<WorldMaterial>>>,
) {
    let mut changed_assets = HashSet::default();
    let mut removed = Vec::new();
    for event in event_reader.iter() {
        match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                changed_assets.insert(handle.clone_weak());
            }
            AssetEvent::Removed { handle } => {
                changed_assets.remove(handle);
                removed.push(handle.clone_weak());
            }
        }
    }

    let mut extracted_assets = Vec::new();
    for handle in changed_assets.drain() {
        if let Some(asset) = assets.get(&handle) {
            extracted_assets.push((handle, asset.clone()));
        }
    }

    commands.insert_resource(ExtractedMaterials2d {
        extracted: extracted_assets,
        removed,
    });
}

pub fn prepare_materials(
    mut prepare_next_frame: Local<PrepareNextFrameMaterials<WorldMaterial>>,
    mut extracted_assets: ResMut<ExtractedMaterials2d<WorldMaterial>>,
    mut render_materials: ResMut<RenderMaterials2d<WorldMaterial>>,
    render_device: Res<RenderDevice>,
    images: Res<RenderAssets<Image>>,
    fallback_image: Res<FallbackImage>,
    pipeline: Res<WorldMaterialPipeline>,
) {
    let queued_assets = std::mem::take(&mut prepare_next_frame.assets);
    for (handle, material) in queued_assets {
        match prepare_material2d(
            &material,
            &render_device,
            &images,
            &fallback_image,
            &pipeline,
        ) {
            Ok(prepared_asset) => {
                render_materials.insert(handle, prepared_asset);
            }
            Err(AsBindGroupError::RetryNextUpdate) => {
                prepare_next_frame.assets.push((handle, material));
            }
        }
    }

    for removed in std::mem::take(&mut extracted_assets.removed) {
        render_materials.remove(&removed);
    }

    for (handle, material) in std::mem::take(&mut extracted_assets.extracted) {
        match prepare_material2d(
            &material,
            &render_device,
            &images,
            &fallback_image,
            &pipeline,
        ) {
            Ok(prepared_asset) => {
                render_materials.insert(handle, prepared_asset);
            }
            Err(AsBindGroupError::RetryNextUpdate) => {
                prepare_next_frame.assets.push((handle, material));
            }
        }
    }
}

fn prepare_material2d<M: Material2d>(
    material: &M,
    render_device: &RenderDevice,
    images: &RenderAssets<Image>,
    fallback_image: &FallbackImage,
    pipeline: &WorldMaterialPipeline,
) -> Result<PreparedMaterial2d<M>, AsBindGroupError> {
    let prepared = material.as_bind_group(
        &pipeline.material_layout,
        render_device,
        images,
        fallback_image,
    )?;
    Ok(PreparedMaterial2d {
        bindings: prepared.bindings,
        bind_group: prepared.bind_group,
        key: prepared.data,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn queue_meshes(
    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
    material_pipeline: Res<WorldMaterialPipeline>,
    mut pipelines: ResMut<SpecializedMeshPipelines<WorldMaterialPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    msaa: Res<Msaa>,
    render_meshes: Res<RenderAssets<Mesh>>,
    render_materials: Res<RenderMaterials2d<WorldMaterial>>,
    material_meshes: Query<(&Handle<WorldMaterial>, &Mesh2dHandle, &Mesh2dUniform)>,
    mut views: Query<(
        &ExtractedView,
        &VisibleEntities,
        Option<&Tonemapping>,
        Option<&DebandDither>,
        &mut RenderPhase<Transparent2d>,
    )>,
) {
    if material_meshes.is_empty() {
        return;
    }

    for (view, visible_entities, tonemapping, dither, mut transparent_phase) in &mut views {
        let draw_function_id = transparent_draw_functions.read().id::<DrawWorldMaterial>();

        let mut view_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples())
            | Mesh2dPipelineKey::from_hdr(view.hdr);

        if !view.hdr {
            if let Some(tonemapping) = tonemapping {
                view_key |= Mesh2dPipelineKey::TONEMAP_IN_SHADER;
                view_key |= match tonemapping {
                    Tonemapping::None => Mesh2dPipelineKey::TONEMAP_METHOD_NONE,
                    Tonemapping::Reinhard => Mesh2dPipelineKey::TONEMAP_METHOD_REINHARD,
                    Tonemapping::ReinhardLuminance => {
                        Mesh2dPipelineKey::TONEMAP_METHOD_REINHARD_LUMINANCE
                    }
                    Tonemapping::AcesFitted => Mesh2dPipelineKey::TONEMAP_METHOD_ACES_FITTED,
                    Tonemapping::AgX => Mesh2dPipelineKey::TONEMAP_METHOD_AGX,
                    Tonemapping::SomewhatBoringDisplayTransform => {
                        Mesh2dPipelineKey::TONEMAP_METHOD_SOMEWHAT_BORING_DISPLAY_TRANSFORM
                    }
                    Tonemapping::TonyMcMapface => Mesh2dPipelineKey::TONEMAP_METHOD_TONY_MC_MAPFACE,
                    Tonemapping::BlenderFilmic => Mesh2dPipelineKey::TONEMAP_METHOD_BLENDER_FILMIC,
                };
            }
            if let Some(DebandDither::Enabled) = dither {
                view_key |= Mesh2dPipelineKey::DEBAND_DITHER;
            }
        }

        for visible_entity in visible_entities.entities.iter() {
            if let Ok((material2d_handle, mesh2d_handle, mesh2d_uniform)) =
                material_meshes.get(*visible_entity)
            {
                if let Some(material2d) = render_materials.get(material2d_handle) {
                    if let Some(mesh) = render_meshes.get(&mesh2d_handle.0) {
                        let mesh_key = view_key
                            | Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology);

                        let pipeline_id = pipelines.specialize(
                            &pipeline_cache,
                            &material_pipeline,
                            WorldMaterialKey {
                                mesh_key,
                                bind_group_data: material2d.key.clone(),
                            },
                            &mesh.layout,
                        );

                        let pipeline_id = match pipeline_id {
                            Ok(id) => id,
                            Err(err) => {
                                error!("{}", err);
                                continue;
                            }
                        };

                        let mesh_z = mesh2d_uniform.transform.w_axis.z;
                        transparent_phase.add(Transparent2d {
                            entity: *visible_entity,
                            draw_function: draw_function_id,
                            pipeline: pipeline_id,
                            sort_key: FloatOrd(mesh_z),
                            batch_range: None,
                        });
                    }
                }
            }
        }
    }
}
