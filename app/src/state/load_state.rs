use bevy::asset::LoadState;
use bevy::prelude::*;
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
) {
    // let tiles_image: Handle<Image> = asset_server.load("graphics/tiles.png");
    // let grass_image: Handle<Image> = asset_server.load("graphics/hr-grass-2.png");
    // let machine_image: Handle<Image> = asset_server.load("graphics/hr-assembling-machine-1.png");
    let images = asset_server.load_folder("graphics").unwrap();

    commands.insert_resource(AssetLoadState {
        check_timer: Timer::from_seconds(0.1f32, TimerMode::Repeating),
        handles: images,
        loaded_handles: Default::default(),
        // handles:vec![tiles_image.clone_untyped(), grass_image.clone_untyped(), machine_image.clone_untyped()],
    });
}

pub fn on_load_exit() {}

pub fn on_load_update(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    time: Res<Time>,
    mut asset_load_state: ResMut<AssetLoadState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if asset_load_state.handles.is_empty() {
        commands.remove_resource::<AssetLoadState>();
        next_state.set(AppState::MainMenu);
        return;
    }
    // if asset_load_state.check_timer.tick(time.delta()).just_finished() {
    // }

    let current_handles = asset_load_state.handles.clone();
    for (i, handle) in current_handles.into_iter().enumerate().rev() {
        // let path = asset_server.get_handle_path(&handle).unwrap();
        let load_state = asset_server.get_load_state(handle);
        match load_state {
            LoadState::NotLoaded => {}
            LoadState::Loading => {}
            LoadState::Loaded => {
                let loaded_handle = asset_load_state.handles.remove(i);
                asset_load_state.loaded_handles.push(loaded_handle);
            }
            LoadState::Failed => {}
            LoadState::Unloaded => {}
        }
    }

    println!("{}", asset_load_state.loaded_handles.len() as f32 / (asset_load_state.loaded_handles.len() + asset_load_state.handles.len()) as f32);
}
