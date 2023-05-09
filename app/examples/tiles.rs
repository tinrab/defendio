use std::error::Error;

use bevy::prelude::*;
use bevy::render::render_resource::{FilterMode, SamplerDescriptor};
use bevy::render::texture::ImageSampler;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResolution;
use defendio_app::asset::CoreAssetSet;
use defendio_app::camera::{MainCameraBundle, MainCameraPlugin};
use defendio_app::interaction::input_action::InputActionPlugin;
use defendio_app::interaction::InteractionPlugin;
use defendio_app::state::{AppState, AppStatePlugin};
use defendio_app::tilemap::material::TilemapMaterial;
use defendio_app::tilemap::plugin::TilemapPlugin;
use defendio_app::tilemap::TilemapBundle;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1280.0, 720.0).with_scale_factor_override(1.0),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_system(bevy::window::close_on_esc)

        .add_plugin(AppStatePlugin {})
        .add_plugin(InteractionPlugin{})
        .add_plugin(TilemapPlugin {})
        .add_plugin(MainCameraPlugin{})

        .add_system(on_game_state_enter.in_schedule(OnEnter(AppState::Game)))
        .add_system(on_game_state_update.in_set(OnUpdate(AppState::Game)))
        .run();
    Ok(())
}

fn on_game_state_enter (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TilemapMaterial>>,
    images:Res<Assets<Image>>,
    core_asset_set: Res<CoreAssetSet>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    commands.spawn(TilemapBundle::build(
        meshes,
        materials,
        images,
        core_asset_set,
        texture_atlases,
    ));
}

fn on_game_state_update (
) {
}

// fn update_images_system(
//     mut images: ResMut<Assets<Image>>,
//     image_resource_set: Res<ImageResourceSet>,
// ) {
//     // let image = images.get_mut(&image_resource_set.tiles_texture).unwrap();
//     // image.sampler_descriptor = ImageSampler::nearest();
// }
