use std::error::Error;

use bevy::prelude::*;
use bevy::render::render_resource::{FilterMode, SamplerDescriptor};
use bevy::render::texture::ImageSampler;
use bevy::render::view::RenderLayers;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResolution;
use defendio_app::asset::TilemapAssetGroup;
use defendio_app::camera::{MainCameraBundle, MainCameraComponent, MainCameraPlugin};
use defendio_app::interaction::input_action::InputActionPlugin;
use defendio_app::interaction::{InteractionPlugin, MousePosition};
use defendio_app::lighting::{LightBundle, LightingPlugin};
use defendio_app::plugin::AppCorePlugin;
use defendio_app::state::AppState;
use defendio_app::tilemap::bundle::TilemapBundle;
use defendio_app::tilemap::material::TilemapMaterial;
use defendio_app::tilemap::plugin::TilemapPlugin;
use defendio_app::world_material::material::WorldMaterial;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280.0, 720.0)
                            .with_scale_factor_override(1.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .add_system(bevy::window::close_on_esc)
        .add_plugin(AppCorePlugin)
        .add_system(on_game_state_enter.in_schedule(OnEnter(AppState::Game)))
        .run();
    Ok(())
}

fn on_game_state_enter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TilemapMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut world_materials: ResMut<Assets<WorldMaterial>>,
    mut images: Res<Assets<Image>>,
    tilemap_asset_group: Res<TilemapAssetGroup>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
}
