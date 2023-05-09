use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::interaction::input_action::InputAction;
use crate::state::AppState;

pub struct MainCameraPlugin {}

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(on_game_state_enter.in_schedule(OnEnter(AppState::Game)))
            .add_system(move_camera.in_set(OnUpdate(AppState::Game)));
    }
}

#[derive(Component)]
pub struct MainCameraComponent {
    target_position: Vec3,
}

#[derive(Bundle)]
pub struct MainCameraBundle {
    camera: MainCameraComponent,
    #[bundle]
    camera2d: Camera2dBundle,
}

const CAMERA_BASE_SPEED: f32 = 300.0f32;

fn on_game_state_enter(
    mut commands: Commands,
) {
    commands.spawn(MainCameraBundle {
        camera: MainCameraComponent {
            target_position: Vec3::ZERO,
        },
        camera2d: Default::default(),
    });
}

fn move_camera(
    time: Res<Time>,
    mut query: Query<(&mut MainCameraComponent, &mut Transform)>,
    input_query: Query<&ActionState<InputAction>>,
//     mut query: Query<(&mut MainCameraComponent, &mut Transform)>,
) {
    let  (camera, mut camera_transform) = query.single_mut();
    // let player_transform = player_query.single();

    // camera.target_position = Some(player_transform.translation);
    let action_state = input_query.single();
    let mut dv = Vec3::ZERO;
    if action_state.pressed(InputAction::Left) {
        dv.x -= 1.0f32;
    } else  if action_state.pressed(InputAction::Right) {
        dv.x += 1.0f32;
    }
    if action_state.pressed(InputAction::Up) {
        dv.y += 1.0f32;
    } else  if action_state.pressed(InputAction::Down) {
        dv.y -= 1.0f32;
    }
    camera_transform.translation += (dv) * CAMERA_BASE_SPEED * time.delta_seconds();
}

// fn follow_target(
//     mut query: Query<(&mut MainCameraComponent, &mut Transform)>,
//     time: Res<Time>,
// ) {
//     let (camera, mut transform) = query.single_mut();
//
//     // if let Some(target_position) = camera.target_position {
//     //     let mut dv = ((target_position - transform.translation) / 200.0);
//     //     dv.z = 0.0;
//     //     transform.translation += (dv) * CAMERA_BASE_SPEED * time.delta_seconds();
//     // }
// }
