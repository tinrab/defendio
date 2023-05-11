use bevy::{prelude::*,
           reflect::TypeUuid,
           render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{BindGroupLayout, RenderPipelineDescriptor, SpecializedMeshPipelineError, VertexBufferLayout};
use bevy::sprite::{Material2d, Material2dKey};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "4284d12f-56dc-49f5-9cc1-68e9d14a7ebc"]
pub struct TilemapMaterial {
    // #[uniform(0)]
    // color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for TilemapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
                  layout: &MeshVertexBufferLayout,
                  key: Material2dKey<Self>) -> Result<(), SpecializedMeshPipelineError> {
        // dbg!(&descriptor.vertex.buffers);
        // let vertex_buffer_layout = layout.get_layout(&[
        //     Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
        //     Mesh::ATTRIBUTE_COLOR.at_shader_location(1),
        // ])?;

        Ok(())
    }
}
