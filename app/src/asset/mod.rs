use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CoreAssetSet {
    // pub tiles_image: Handle<Image>,
    pub tiles_atlas: Handle<TextureAtlas>,
}

#[derive(Resource, Default)]
pub struct ShaderAssetSet {
    pub tilemap_shader: Handle<Shader>,
}
