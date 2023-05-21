use array_init::array_init;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::utils::HashMap;

use num_enum::TryFromPrimitive;

use crate::input_manager::action_state::{
    ActionDataArray, InputActionData, InputActionTriggerState,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum InputAction {
    // Movement
    Left,
    Right,
    Up,
    Down,
    // Interaction
    Select,
}

// TODO: optimize storage
#[derive(Resource)]
pub struct InputActionMap(HashMap<InputAction, InputActionTrigger>);

pub enum InputActionTrigger {
    KeyCode(KeyCode),
    MouseButton(MouseButton),
}

impl InputActionMap {
    pub fn get_states(
        &self,
        input_key_codes: &Input<KeyCode>,
        input_mouse_buttons: &Input<MouseButton>,
    ) -> ActionDataArray {
        let mut state: ActionDataArray = array_init(|_| Default::default());
        for (action, trigger) in self.0.iter() {
            match trigger {
                InputActionTrigger::KeyCode(key_code) => {
                    if input_key_codes.pressed(*key_code) {
                        state[*action as usize].state = InputActionTriggerState::JustPressed;
                    }
                }
                InputActionTrigger::MouseButton(mouse_button) => {
                    if input_mouse_buttons.pressed(*mouse_button) {
                        state[*action as usize].state = InputActionTriggerState::JustPressed;
                    }
                }
            }
        }
        state
    }
}

impl Default for InputActionMap {
    fn default() -> Self {
        InputActionMap(
            [
                // Movement
                (InputAction::Left, KeyCode::A.into()),
                (InputAction::Right, KeyCode::D.into()),
                (InputAction::Up, KeyCode::W.into()),
                (InputAction::Down, KeyCode::S.into()),
                // Combat
                (InputAction::Select, MouseButton::Left.into()),
            ]
                .into_iter()
                .collect(),
        )
    }
}

impl From<KeyCode> for InputActionTrigger {
    fn from(key_code: KeyCode) -> Self {
        InputActionTrigger::KeyCode(key_code)
    }
}

impl From<MouseButton> for InputActionTrigger {
    fn from(mouse_button: MouseButton) -> Self {
        InputActionTrigger::MouseButton(mouse_button)
    }
}
