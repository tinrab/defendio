use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MousePosition(pub Vec2);

pub fn mouse_position_system(
    mut cursor_event_reader: EventReader<CursorMoved>,
    mut mouse_position: ResMut<MousePosition>,
) {
    for cursor_event in cursor_event_reader.iter() {
        mouse_position.0 = cursor_event.position;
    }
}
