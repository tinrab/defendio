use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::asset::load::AssetLoadPlugin;
use crate::camera::MainCameraPlugin;
use crate::input_manager::InputManagerPlugin;
use crate::lighting::LightingPlugin;
use crate::state::AppState;
use crate::tilemap::plugin::TilemapPlugin;
use crate::world_material::plugin::WorldMaterialPlugin;

pub struct AppCorePlugin;

impl Plugin for AppCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugin(AssetLoadPlugin)
            .add_plugin(InputManagerPlugin)
            .add_plugin(WorldMaterialPlugin)
            .add_plugin(LightingPlugin)
            .add_plugin(TilemapPlugin)
            .add_plugin(MainCameraPlugin);
    }
}
