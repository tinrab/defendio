use bevy::{prelude::*,
           reflect::TypeUuid,
           render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy::sprite::{Material2d, Material2dPlugin};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "4284d12f-56dc-49f5-9cc1-68e9d14a7ebc"]
pub struct TiledSurfaceMaterial {
    // #[uniform(0)]
    // color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for TiledSurfaceMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tiled_surface_material.wgsl".into()
    }
}

pub struct SurfacePlugin;

impl Plugin for SurfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<TiledSurfaceMaterial>::default());
    }
}
