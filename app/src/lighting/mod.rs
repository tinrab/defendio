use crate::lighting::light_mesh::make_light_mesh;
use crate::lighting::pipeline::{
    prepare_instance_buffers, DrawLighting, ExtractedLight, LightingPipeline,
};
use bevy::core_pipeline::core_2d::Transparent2d;
use bevy::core_pipeline::core_3d::Transparent3d;
use bevy::ecs::query::QueryItem;
use bevy::ecs::system::lifetimeless::{Read, SRes};
use bevy::ecs::system::SystemParamItem;
use bevy::pbr::{
    MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup, SetMeshViewBindGroup,
};
use bevy::prelude::*;
use bevy::render::extract_component::{ExtractComponent, ExtractComponentPlugin};
use bevy::render::mesh::{GpuBufferInfo, MeshVertexBufferLayout};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{
    AddRenderCommand, DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult, RenderPhase,
    SetItemPipeline, TrackedRenderPass,
};
use bevy::render::render_resource::{
    Buffer, BufferInitDescriptor, BufferUsages, Extent3d, PipelineCache, RenderPipelineDescriptor,
    SpecializedMeshPipeline, SpecializedMeshPipelineError, SpecializedMeshPipelines,
    SpecializedRenderPipelines, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::{ExtractedView, NoFrustumCulling};
use bevy::render::{RenderApp, RenderSet};
use bevy::sprite::{DrawMesh2d, Mesh2dHandle, Mesh2dPipelineKey, Mesh2dUniform};
use bevy::utils::FloatOrd;
use bevy::window::WindowResized;

mod light_mesh;
mod pipeline;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<ExtractedLight>::default())
            .add_plugin(ExtractComponentPlugin::<LightMeshComponent>::default())
            .add_startup_system(init_lighting)
            .add_system(window_resize_system);
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent2d, DrawLighting>()
            .init_resource::<LightingPipeline>()
            .init_resource::<SpecializedMeshPipelines<LightingPipeline>>()
            .add_system(queue_lighting_mesh.in_set(RenderSet::Queue))
            .add_system(prepare_instance_buffers.in_set(RenderSet::Prepare));
    }
}

#[derive(Component, Clone)]
pub struct LightComponent {
    pub instances: Vec<LightInstanceData>,
}

#[derive(Component, Clone)]
pub struct LightMeshComponent;

#[derive(Debug, Clone, Copy)]
pub struct LightInstanceData {
    pub position: Vec3,
    pub scale: f32,
    pub color: Color,
}

#[derive(Bundle)]
pub struct LightBundle {
    light: LightComponent,
}

impl ExtractComponent for LightMeshComponent {
    type Query = &'static LightMeshComponent;
    type Filter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::Query>) -> Option<Self> {
        Some(item.clone())
    }
}

impl LightBundle {
    pub fn new(position: Vec3, scale: f32, color: Color) -> Self {
        LightBundle {
            light: LightComponent {
                instances: vec![LightInstanceData {
                    position,
                    scale,
                    color,
                }],
            },
        }
    }
}

fn init_lighting(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // let size = Extent3d {
    //     width: 512,
    //     height: 512,
    //     ..Default::default()
    // };
    // let mut image = Image {
    //     texture_descriptor: TextureDescriptor {
    //         label: None,
    //         size,
    //         dimension: TextureDimension::D2,
    //         format: TextureFormat::Bgra8UnormSrgb,
    //         mip_level_count: 1,
    //         sample_count: 1,
    //         usage: TextureUsages::TEXTURE_BINDING
    //             | TextureUsages::COPY_DST
    //             | TextureUsages::RENDER_ATTACHMENT,
    //         view_formats: &[],
    //     },
    //     ..Default::default()
    // };
    // // Fill with zeroes
    // image.resize(size);
    commands.spawn((
        Mesh2dHandle::from(meshes.add(make_light_mesh())),
        SpatialBundle::INHERITED_IDENTITY,
        LightMeshComponent,
        NoFrustumCulling,
    ));
}

fn window_resize_system(mut window_resized_events: EventReader<WindowResized>) {
    for event in window_resized_events.iter() {
        println!("width = {} height = {}", event.width, event.height);
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
    material_meshes: Query<(Entity, &Mesh2dUniform, &Mesh2dHandle), With<LightMeshComponent>>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent2d>)>,
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
