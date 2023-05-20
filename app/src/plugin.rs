use crate::asset::load::AssetLoadPlugin;
use crate::camera::MainCameraPlugin;
use crate::interaction::InteractionPlugin;
use crate::lighting::LightingPlugin;
use crate::state::AppState;
use crate::tilemap::plugin::TilemapPlugin;
use crate::world_material::plugin::WorldMaterialPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct AppCorePlugin;

impl Plugin for AppCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugin(AssetLoadPlugin)
            .add_plugin(InteractionPlugin)
            .add_plugin(WorldMaterialPlugin)
            .add_plugin(LightingPlugin)
            .add_plugin(TilemapPlugin)
            .add_plugin(MainCameraPlugin);
    }
}
