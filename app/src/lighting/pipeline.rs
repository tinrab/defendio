use crate::lighting::light_mesh::ATTRIBUTE_INTENSITY;
use crate::lighting::{LightComponent, LightingComponent};
use bevy::core_pipeline::core_3d::Transparent3d;
use bevy::ecs::query::{QueryItem, ROQueryItem};
use bevy::ecs::system::lifetimeless::{Read, SRes};
use bevy::ecs::system::SystemParamItem;
use bevy::pbr::{
    MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup, SetMeshViewBindGroup,
};
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::mesh::{GpuBufferInfo, MeshVertexBufferLayout};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{
    DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult, RenderPhase, SetItemPipeline,
    TrackedRenderPass,
};
use bevy::render::render_resource::{
    Buffer, BufferInitDescriptor, BufferUsages, PipelineCache, RenderPipelineDescriptor,
    SpecializedMeshPipeline, SpecializedMeshPipelineError, SpecializedMeshPipelines,
    SpecializedRenderPipeline, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::ExtractedView;
use bevy::sprite::{
    Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, SetMesh2dBindGroup, SetMesh2dViewBindGroup,
};
use bytemuck::{Pod, Zeroable};
use itertools::Itertools;

#[derive(Resource)]
pub struct LightingPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: Mesh2dPipeline,
}

pub type DrawLighting = (
    SetItemPipeline,
    SetMesh2dViewBindGroup<0>,
    SetMesh2dBindGroup<1>,
    DrawMeshInstanced,
);

pub struct DrawMeshInstanced;

#[derive(Component)]
pub struct ExtractedLight {
    instance: LightInstanceBin,
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct LightInstanceBin {
    pub position: Vec3,
    pub scale: f32,
    pub color: [f32; 4],
}

#[derive(Component)]
pub struct ExtractedLighting;

impl ExtractComponent for ExtractedLight {
    type Query = (&'static LightComponent, &'static Transform);
    type Filter = ();
    type Out = Self;

    fn extract_component((light, transform): QueryItem<'_, Self::Query>) -> Option<Self> {
        Some(ExtractedLight {
            instance: LightInstanceBin {
                position: transform.translation,
                scale: light.scale,
                color: light.color.as_rgba_f32(),
            }
        })
    }
}

impl ExtractComponent for ExtractedLighting {
    type Query = &'static LightingComponent;
    type Filter = ();
    type Out = Self;

    fn extract_component(_item: QueryItem<'_, Self::Query>) -> Option<Self> {
        Some(ExtractedLighting {})
    }
}

impl FromWorld for LightingPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let shader = asset_server.load("shaders/light.wgsl");

        let mesh_pipeline = world.resource::<Mesh2dPipeline>();

        LightingPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
        }
    }
}

impl SpecializedMeshPipeline for LightingPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;
        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers = vec![layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_INTENSITY.at_shader_location(1),
        ])?];
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<LightInstanceBin>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                // position and scale
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 2,
                },
                // color
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: VertexFormat::Float32x4.size(),
                    shader_location: 3,
                },
            ],
        });
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        Ok(descriptor)
    }
}

impl<P: PhaseItem> RenderCommand<P> for DrawMeshInstanced {
    type Param = SRes<RenderAssets<Mesh>>;
    type ViewWorldQuery = ();
    type ItemWorldQuery = (Read<Mesh2dHandle>, Read<InstanceBuffer>);

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: (),
        (mesh_handle, instance_buffer): (&'w Mesh2dHandle, &'w InstanceBuffer),
        meshes: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        // TODO pass.set_push_constants()
        let gpu_mesh = match meshes.into_inner().get(&mesh_handle.0) {
            Some(gpu_mesh) => gpu_mesh,
            None => return RenderCommandResult::Failure,
        };

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                index_format,
                count,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..instance_buffer.length as u32);
            }
            GpuBufferInfo::NonIndexed { vertex_count } => {
                pass.draw(0..*vertex_count, 0..instance_buffer.length as u32);
            }
        }
        RenderCommandResult::Success
    }
}

#[derive(Component)]
pub struct InstanceBuffer {
    buffer: Buffer,
    length: usize,
}

pub fn prepare_instance_buffers(
    mut commands: Commands,
    mesh_query: Query<Entity, With<ExtractedLighting>>,
    light_query: Query<&ExtractedLight>,
    render_device: Res<RenderDevice>,
) {
    let entity = mesh_query.single();
    let lights = light_query
        .iter()
        .map(|light| light.instance)
        .collect_vec();

    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("instance data buffer"),
        contents: bytemuck::cast_slice(lights.as_slice()),
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
    });
    commands.entity(entity).insert(InstanceBuffer {
        buffer,
        length: lights.len(),
    });
}
