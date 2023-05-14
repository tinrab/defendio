use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputActionPlugin {}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum InputAction {
    // Movement
    Left,
    Right,
    Up,
    Down,
    // Combat
    Fire,
}

pub type CustomInputManagerBundle = InputManagerBundle<InputAction>;

pub type CustomActionState = ActionState<InputAction>;

impl Plugin for InputActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<InputAction>::default())
            .add_startup_system(spawn_input_manager);
    }
}

impl InputActionPlugin {
    pub fn default_input_map() -> InputMap<InputAction> {
        let mut m = InputMap::new([
            // Movement
            (KeyCode::A, InputAction::Left),
            (KeyCode::D, InputAction::Right),
            (KeyCode::W, InputAction::Up),
            (KeyCode::S, InputAction::Down),
        ]);
        // Combat
        m.insert(MouseButton::Left, InputAction::Fire);
        m
    }
}

fn spawn_input_manager(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<InputAction> {
        action_state: ActionState::default(),
        input_map: InputActionPlugin::default_input_map(),
    });
}
