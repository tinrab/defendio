use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use crate::tilemap::material::TilemapMaterial;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<TilemapMaterial>::default());
    }
}
