use bevy::prelude::*;
use crate::interaction::input_action::InputActionPlugin;

pub mod input_action;

pub struct InteractionPlugin;

#[derive(Resource, Default)]
pub struct MousePosition(pub Vec2);

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MousePosition>()
            .add_plugin(InputActionPlugin{})
            .add_system(update_mouse_position);
    }
}

fn update_mouse_position(
    mut cursor_events: EventReader<CursorMoved>,
    mut mouse_position: ResMut<MousePosition>,
) {
    for cursor_event in cursor_events.iter() {
        mouse_position.0 = cursor_event.position;
    }
}
