use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::render::render_resource::ShaderRef;
use bevy::render::texture::ImageSampler;
use crate::asset::{CoreAssetSet, ShaderAssetSet};
use crate::state::AppState;

#[derive(Resource, Default)]
pub struct AssetLoadState {
    pub check_timer: Timer,
    pub handles: Vec<HandleUntyped>,
    pub loaded_handles: Vec<HandleUntyped>,
}

pub fn on_load_enter(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tiles_image: Handle<Image> = asset_server.load("graphics/tiles.png");
    let tiles_atlas: Handle<TextureAtlas> = texture_atlases.add(TextureAtlas::from_grid(tiles_image.clone(), Vec2::new(8.0f32,8.0f32),8,8,None,None));
    let tilemap_shader: Handle<Shader> = asset_server.load("shaders/tilemap.wgsl");

    commands.insert_resource(AssetLoadState {
        check_timer: Timer::from_seconds(0.1f32, TimerMode::Repeating),
        handles: vec![
            tiles_image.clone_untyped(),
            tilemap_shader.clone_untyped(),
        ],
        loaded_handles: Default::default(),
    });
    commands.insert_resource(CoreAssetSet {
        tiles_atlas,
    });
    commands.insert_resource(ShaderAssetSet {
        tilemap_shader,
    });
}

pub fn on_load_exit() {}

pub fn on_load_update(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    time: Res<Time>,
    mut asset_load_state: ResMut<AssetLoadState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut image_assets: ResMut<Assets<Image>>,
) {
    if asset_load_state.handles.is_empty() {
        commands.remove_resource::<AssetLoadState>();
        next_state.set(AppState::Game);
        return;
    }
    // if asset_load_state.check_timer.tick(time.delta()).just_finished() {
    // }

    let current_handles = asset_load_state.handles.clone();
    for (i, handle) in current_handles.into_iter().enumerate().rev() {
        // let path = asset_server.get_handle_path(&handle).unwrap();
        let load_state = asset_server.get_load_state(&handle);
        match load_state {
            LoadState::NotLoaded => {}
            LoadState::Loading => {}
            LoadState::Loaded => {
                let loaded_handle = asset_load_state.handles.remove(i);
                asset_load_state.loaded_handles.push(loaded_handle);

                if let Some(image) = image_assets.get_mut(&handle.typed()) {
                    image.sampler_descriptor = ImageSampler::nearest();
                }
            }
            LoadState::Failed => {}
            LoadState::Unloaded => {}
        }
    }

    println!("{}", asset_load_state.loaded_handles.len() as f32 / (asset_load_state.loaded_handles.len() + asset_load_state.handles.len()) as f32);
}
