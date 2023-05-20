use crate::lighting::camera::{light_camera_update, LightCameraBundle};
use crate::lighting::light_mesh::make_light_mesh;
use crate::lighting::pipeline::{
    prepare_instance_buffers, DrawLighting, ExtractedLight, ExtractedLighting, LightingPipeline,
};
use crate::lighting::uniform::{prepare_lighting_uniform_buffer, LightingUniformBuffer};
use crate::tilemap::material::TilemapMaterial;
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
use bevy::render::view::{ExtractedView, NoFrustumCulling, RenderLayers, VisibleEntities};
use bevy::render::{RenderApp, RenderSet};
use bevy::sprite::{
    DrawMesh2d, Material2d, MaterialMesh2dBundle, Mesh2dHandle, Mesh2dPipelineKey, Mesh2dUniform,
};
use bevy::utils::FloatOrd;
use bevy::window::WindowResized;

mod camera;
mod light_mesh;
mod pipeline;
pub mod uniform;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<ExtractedLight>::default())
            .add_plugin(ExtractComponentPlugin::<ExtractedLighting>::default())
            .add_startup_system(setup_lighting)
            .add_system(window_resize_system)
            .add_system(light_camera_update)
            .add_system(lighting_update_system)
            .add_system(material_update_system);

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .add_render_command::<Transparent2d, DrawLighting>()
                .init_resource::<LightingPipeline>()
                .init_resource::<SpecializedMeshPipelines<LightingPipeline>>()
                .init_resource::<LightingUniformBuffer>()
                .add_system(queue_lighting_mesh.in_set(RenderSet::Queue))
                .add_system(prepare_instance_buffers.in_set(RenderSet::Prepare))
                .add_system(prepare_lighting_uniform_buffer.in_set(RenderSet::Prepare));
        }
    }
}

#[derive(Component, Clone)]
pub struct LightComponent {
    pub scale: f32,
    pub color: Color,
}

#[derive(Component, Debug)]
pub struct LightingComponent {
    pub map_image: Handle<Image>,
}

#[derive(Bundle)]
pub struct LightBundle {
    light: LightComponent,
    transform: Transform,
}

const LIGHT_RENDER_LAYER: RenderLayers = RenderLayers::layer(1);

impl LightBundle {
    pub fn new(position: Vec3, scale: f32, color: Color) -> Self {
        LightBundle {
            light: LightComponent { scale, color },
            transform: Transform::from_translation(position),
        }
    }
}

fn setup_lighting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    window: Query<&Window>,
) {
    let size = if let Ok(window) = window.get_single() {
        Extent3d {
            width: window.width() as u32,
            height: window.height() as u32,
            ..Default::default()
        }
    } else {
        Extent3d {
            width: 1024,
            height: 1024,
            ..Default::default()
        }
    };
    let mut map_image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            // format: TextureFormat::Bgra8UnormSrgb,
            format: TextureFormat::Rg11b10Float,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..Default::default()
    };
    let map_image_handle = images.add(map_image);

    commands.spawn((
        Mesh2dHandle::from(meshes.add(make_light_mesh())),
        SpatialBundle::INHERITED_IDENTITY,
        LightingComponent {
            map_image: map_image_handle.clone(),
        },
        NoFrustumCulling,
        LIGHT_RENDER_LAYER,
    ));
    commands.spawn((
        LightCameraBundle::new(map_image_handle.clone()),
        LIGHT_RENDER_LAYER,
    ));
}

#[allow(clippy::too_many_arguments)]
fn queue_lighting_mesh(
    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
    lighting_pipeline: Res<LightingPipeline>,
    mut pipelines: ResMut<SpecializedMeshPipelines<LightingPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    msaa: Res<Msaa>,
    meshes: Res<RenderAssets<Mesh>>,
    material_meshes: Query<(&Mesh2dUniform, &Mesh2dHandle), With<ExtractedLighting>>,
    mut views: Query<(
        &ExtractedView,
        &VisibleEntities,
        &mut RenderPhase<Transparent2d>,
    )>,
) {
    if material_meshes.is_empty() {
        return;
    }

    let draw_lighting = transparent_draw_functions.read().id::<DrawLighting>();

    let msaa_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples());

    for (view, visible_entities, mut transparent_phase) in &mut views {
        let view_key = msaa_key | Mesh2dPipelineKey::from_hdr(view.hdr);
        for visible_entity in visible_entities.entities.iter() {
            if let Ok((mesh_uniform, mesh_handle)) = material_meshes.get(*visible_entity) {
                if let Some(mesh) = meshes.get(&mesh_handle.0) {
                    let key = view_key
                        | Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology);
                    let pipeline = pipelines
                        .specialize(&pipeline_cache, &lighting_pipeline, key, &mesh.layout)
                        .unwrap();
                    let mesh_z = mesh_uniform.transform.w_axis.z;
                    transparent_phase.add(Transparent2d {
                        sort_key: FloatOrd(mesh_z),
                        entity: *visible_entity,
                        pipeline,
                        draw_function: draw_lighting,
                        batch_range: None,
                    });
                }
            }
        }
    }
}

fn window_resize_system(
    mut window_resized_events: EventReader<WindowResized>,
    lighting_query: Query<&LightingComponent>,
    mut images: ResMut<Assets<Image>>,
) {
    for event in window_resized_events.iter() {
        let map_image_handle = if let Ok(lighting) = lighting_query.get_single() {
            &lighting.map_image
        } else {
            return;
        };
        let map_image = images.get_mut(map_image_handle).unwrap();

        if event.width < 2.0 || event.height < 2.0 {
            break;
        }
        map_image.resize(Extent3d {
            width: event.width as u32,
            height: event.height as u32,
            ..Default::default()
        });
    }
}

fn lighting_update_system(
    lighting_query: Query<
        &LightingComponent,
        Or<(Added<LightingComponent>, Changed<LightingComponent>)>,
    >,
    mut tilemap_materials: ResMut<Assets<TilemapMaterial>>,
) {
    let map_image = if let Ok(lighting) = lighting_query.get_single() {
        &lighting.map_image
    } else {
        return;
    };

    for (_, material) in tilemap_materials.iter_mut() {
        material.lighting_texture = Some(map_image.clone());
    }
}

fn material_update_system(
    lighting_query: Query<&LightingComponent>,
    mut tilemap_material_event_reader: EventReader<AssetEvent<TilemapMaterial>>,
    mut tilemap_materials: ResMut<Assets<TilemapMaterial>>,
) {
    let map_image = if let Ok(lighting) = lighting_query.get_single() {
        &lighting.map_image
    } else {
        return;
    };

    for event in tilemap_material_event_reader.iter() {
        let handle = match event {
            AssetEvent::Created { handle } => handle,
            AssetEvent::Modified { handle } => handle,
            _ => continue,
        };
        if let Some(material) = tilemap_materials.get_mut(handle) {
            material.lighting_texture = Some(map_image.clone());
        }
    }
}
