use bevy::{prelude::*,
           reflect::TypeUuid,
           render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy::sprite::{Material2d};

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
}
