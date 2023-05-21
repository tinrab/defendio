use array_init::array_init;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

use crate::input_manager::action::{InputAction, InputActionMap};

pub type ActionDataArray = [InputActionData; 16usize];

#[derive(Resource)]
pub struct InputActionState {
    actions: ActionDataArray,
}

#[derive(Debug, Clone)]
pub struct InputActionData {
    pub state: InputActionTriggerState,
    pub consumed: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum InputActionTriggerState {
    Pressed,
    JustPressed,
    Released,
    JustReleased,
}

impl Default for InputActionState {
    fn default() -> Self {
        InputActionState {
            actions: array_init(|_| Default::default()),
        }
    }
}

impl InputActionState {
    fn update(&mut self, next_actions: ActionDataArray) {
        for (i, next_action) in next_actions.into_iter().enumerate() {
            let action = if let Ok(action) = InputAction::try_from(i as u8) {
                action
            } else {
                break;
            };
            match next_action.state {
                InputActionTriggerState::Pressed => self.press(action),
                InputActionTriggerState::JustPressed => self.press(action),
                InputActionTriggerState::Released => self.release(action),
                InputActionTriggerState::JustReleased => self.release(action),
            }
        }
    }

    fn get_action_mut(&mut self, action: InputAction) -> &mut InputActionData {
        &mut self.actions[action as usize]
    }

    fn press(&mut self, action: InputAction) {
        let action_data = &mut self.actions[action as usize];
        if action_data.consumed {
            return;
        }
        action_data.state.press();
    }

    fn release(&mut self, action: InputAction) {
        let action_data = &mut self.actions[action as usize];
        action_data.consumed = false;
        action_data.state.release();
    }

    pub fn consume(&mut self, action: InputAction) {
        let action_data = &mut self.actions[action as usize];
        action_data.consumed = true;
        action_data.state.release();
    }

    pub fn pressed(&self, action: InputAction) -> bool {
        self.actions[action as usize].state.pressed()
    }

    pub fn just_pressed(&self, action: InputAction) -> bool {
        self.actions[action as usize].state.just_pressed()
    }

    pub fn released(&self, action: InputAction) -> bool {
        self.actions[action as usize].state.released()
    }

    pub fn just_released(&self, action: InputAction) -> bool {
        self.actions[action as usize].state.just_released()
    }
}

impl InputActionTriggerState {
    pub fn tick(&mut self) {
        use InputActionTriggerState::*;
        *self = match self {
            JustPressed => Pressed,
            Pressed => Pressed,
            JustReleased => Released,
            Released => Released,
        }
    }

    pub fn press(&mut self) {
        if *self != InputActionTriggerState::Pressed {
            *self = InputActionTriggerState::JustPressed;
        }
    }

    pub fn release(&mut self) {
        if *self != InputActionTriggerState::Released {
            *self = InputActionTriggerState::JustReleased;
        }
    }

    pub fn pressed(&self) -> bool {
        *self == InputActionTriggerState::Pressed || *self == InputActionTriggerState::JustPressed
    }

    pub fn released(&self) -> bool {
        *self == InputActionTriggerState::Released || *self == InputActionTriggerState::JustReleased
    }

    pub fn just_pressed(&self) -> bool {
        *self == InputActionTriggerState::JustPressed
    }

    pub fn just_released(&self) -> bool {
        *self == InputActionTriggerState::JustReleased
    }
}

impl Default for InputActionData {
    fn default() -> Self {
        InputActionData {
            state: InputActionTriggerState::Released,
            consumed: false,
        }
    }
}

pub fn keyboard_input_system(
    mut input_action_state: ResMut<InputActionState>,
    input_action_map: Res<InputActionMap>,
    input_key_codes: Res<Input<KeyCode>>,
    input_mouse_buttons: Res<Input<MouseButton>>,
    // mut event_reader: EventReader<KeyboardInput>,
) {
    for action_data in input_action_state
        .bypass_change_detection()
        .actions
        .iter_mut()
    {
        action_data.state.tick();
    }

    let state = input_action_map.get_states(&input_key_codes, &input_mouse_buttons);
    input_action_state.update(state);
    // for event in event_reader.iter() {
    //     if let Some(key_code) = event.key_code {
    //         // let action = input_action_state.get_action_data_mut()
    //         match event.state {
    //             ButtonState::Pressed => {}
    //             ButtonState::Released => {}
    //         }
    //     }
    // }
}
