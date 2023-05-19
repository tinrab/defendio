use bevy::prelude::*;

pub mod load;

#[derive(Resource, Default)]
pub struct TilemapAssetGroup {
    pub texture_atlas: Handle<TextureAtlas>,
    pub shader: Handle<Shader>,
}
