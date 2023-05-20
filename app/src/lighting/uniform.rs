use crate::lighting::pipeline::{ExtractedLight, GpuLight};
use crate::lighting::LightComponent;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::render_resource::{ShaderType, StorageBuffer, UniformBuffer};
use bevy::render::renderer::{RenderDevice, RenderQueue};

#[derive(Default, Clone, ShaderType)]
pub struct GpuLightingUniform {
    #[size(runtime)]
    lights: Vec<GpuLight>,
}

/// The buffer containing the [`GpuLightingUniform`]
#[derive(Resource, Default)]
pub struct LightingUniformBuffer {
    pub buffer: StorageBuffer<GpuLightingUniform>,
}

pub fn prepare_lighting_uniform_buffer(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    lights_query: Query<&ExtractedLight>,
    mut lighting_buffer: ResMut<LightingUniformBuffer>,
) {
    let buffer = lighting_buffer.buffer.get_mut();
    buffer.lights = lights_query.iter().map(|light| light.instance).collect();

    lighting_buffer
        .buffer
        .write_buffer(&render_device, &render_queue);
}
