use bevy::prelude::*;
use crate::state::load_state::{on_load_enter, on_load_exit, on_load_update};

mod load_state;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    Load,
    MainMenu,
}

pub struct AppStatePlugin {}

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_system(on_load_enter.in_schedule(OnEnter(AppState::Load)))
            .add_system(on_load_exit.in_schedule(OnExit(AppState::Load)))
            .add_system(on_load_update.in_set(OnUpdate(AppState::Load)));
    }
}
