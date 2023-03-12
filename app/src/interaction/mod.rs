use bevy::prelude::*;

pub mod input_action;

pub struct InteractionPlugin;

#[derive(Resource)]
pub struct MousePosition(pub Vec2);

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MousePosition(Default::default()))
            .add_system(update_mouse_position);
    }
}

fn update_mouse_position(
    mut cursor_events: EventReader<CursorMoved>,
    mut mouse_position: ResMut<MousePosition>,
) {
    for cursor_event in  cursor_events.iter() {
        mouse_position.0 = cursor_event.position;
    }
}
