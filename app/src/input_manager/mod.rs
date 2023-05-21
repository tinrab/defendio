use bevy::input::InputSystem;
use bevy::prelude::*;

use crate::input_manager::action::InputActionMap;
use crate::input_manager::action_state::{keyboard_input_system, InputActionState};
use crate::input_manager::mouse::{mouse_position_system, MousePosition};

pub mod action;
pub mod action_state;
pub mod mouse;

pub struct InputManagerPlugin;

impl Plugin for InputManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>()
            .init_resource::<InputActionMap>()
            .init_resource::<InputActionState>()
            .add_systems(
                (keyboard_input_system, mouse_position_system)
                    .chain()
                    .in_set(InputSystem),
            );
    }
}
