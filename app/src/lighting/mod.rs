use bevy::core_pipeline::core_2d::Transparent2d;
use bevy::core_pipeline::core_3d::Transparent3d;
use bevy::ecs::system::lifetimeless::{Read, SRes};
use bevy::ecs::system::SystemParamItem;
use bevy::pbr::{MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup, SetMeshViewBindGroup};
use bevy::prelude::*;
use bevy::render::mesh::{GpuBufferInfo, MeshVertexBufferLayout};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{AddRenderCommand, DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult, RenderPhase, SetItemPipeline, TrackedRenderPass};
use bevy::render::render_resource::{Buffer, BufferInitDescriptor, BufferUsages, PipelineCache, RenderPipelineDescriptor, SpecializedMeshPipeline, SpecializedMeshPipelineError, SpecializedMeshPipelines, SpecializedRenderPipelines, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use bevy::render::{RenderApp, RenderSet};
use bevy::render::extract_component::ExtractComponentPlugin;
use bevy::render::renderer::RenderDevice;
use bevy::render::view::{ExtractedView, NoFrustumCulling};
use bevy::sprite::{DrawMesh2d, Mesh2dHandle, Mesh2dPipelineKey, Mesh2dUniform};
use bevy::utils::FloatOrd;
use crate::lighting::light_material::{LightInstanceData, InstanceMaterialData};
use crate::lighting::light_mesh::make_light_mesh;
use crate::lighting::pipeline::{DrawLighting, LightingPipeline, prepare_instance_buffers};

mod light_material;
mod light_mesh;
mod pipeline;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<InstanceMaterialData>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent2d, DrawLighting>()
            .init_resource::<LightingPipeline>()
            .init_resource::<SpecializedMeshPipelines<LightingPipeline>>()
            .add_system(queue_lighting_mesh.in_set(RenderSet::Queue))
            .add_system(prepare_instance_buffers.in_set(RenderSet::Prepare));
    }
}

#[derive(Bundle)]
pub struct LightBundle {
    // #[bundle]
    // obj: MaterialMesh2dBundle<LightMaterial>,
}

impl LightBundle {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
    ) {
        commands.spawn((
            Mesh2dHandle::from(meshes.add(make_light_mesh())),
            SpatialBundle::INHERITED_IDENTITY,
            InstanceMaterialData(
                (1..50).map(|x| LightInstanceData {
                    position: Vec3::new(rand::random::<f32>() * 70.0, rand::random::<f32>() * 70.0, 2.0),
                    scale: rand::random::<f32>() * 10.0 + 2.0,
                    color: Color::hsl(rand::random::<f32>() * 360.0, 0.9, 0.6).as_rgba_f32(),
                }).collect()
            ),
            NoFrustumCulling,
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn queue_lighting_mesh(
    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
    lighting_pipeline: Res<LightingPipeline>,
    mut pipelines: ResMut<SpecializedMeshPipelines<LightingPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    msaa: Res<Msaa>,
    meshes: Res<RenderAssets<Mesh>>,
    material_meshes: Query<(Entity, &Mesh2dUniform, &Mesh2dHandle), With<InstanceMaterialData>>,
    mut views: Query<(
        &ExtractedView,
        &mut RenderPhase<Transparent2d>,
    )>,
) {
    if material_meshes.is_empty() {
        return;
    }

    let draw_lighting = transparent_draw_functions.read().id::<DrawLighting>();

    let msaa_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples());

    for (view, mut transparent_phase) in &mut views {
        let view_key = msaa_key | Mesh2dPipelineKey::from_hdr(view.hdr);
        for (entity, mesh_uniform, mesh_handle) in &material_meshes {
            if let Some(mesh) = meshes.get(&mesh_handle.0) {
                let key =
                    view_key | Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology);
                let pipeline = pipelines
                    .specialize(&pipeline_cache, &lighting_pipeline, key, &mesh.layout)
                    .unwrap();
                let mesh_z = mesh_uniform.transform.w_axis.z;
                transparent_phase.add(Transparent2d {
                    sort_key: FloatOrd(mesh_z),
                    entity,
                    pipeline,
                    draw_function: draw_lighting,
                    batch_range: None,
                });
            }
        }
    }
}
