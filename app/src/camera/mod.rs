use crate::input_manager::action::InputAction;
use crate::input_manager::action_state::InputActionState;
use crate::state::AppState;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::view::RenderLayers;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_game_state_enter.in_schedule(OnEnter(AppState::Game)))
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
    pub camera2d: Camera2dBundle,
}

const CAMERA_BASE_SPEED: f32 = 50.0f32;

impl MainCameraBundle {
    pub fn new() -> Self {
        MainCameraBundle {
            camera: MainCameraComponent {
                target_position: Vec3::ZERO,
            },
            camera2d: Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    ..Default::default()
                },
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::BLACK),
                },
                projection: OrthographicProjection {
                    far: 1000.0,
                    scaling_mode: ScalingMode::WindowSize(32.0),
                    ..Default::default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                // tonemapping: Tonemapping::None,
                transform: Transform::from_translation(Vec3::new(5.0, 5.0, 1000.0 - 0.1)),
                ..Default::default()
            },
        }
    }
}

fn on_game_state_enter(mut commands: Commands) {
    commands.spawn((MainCameraBundle::new(), BloomSettings::OLD_SCHOOL));
    // commands.spawn(MainCameraBundle::new());
}

fn move_camera(
    time: Res<Time>,
    mut query: Query<(&mut MainCameraComponent, &mut Transform)>,
    input_action_state: Res<InputActionState>,
    //     mut query: Query<(&mut MainCameraComponent, &mut Transform)>,
) {
    let (camera, mut camera_transform) = query.single_mut();
    // let player_transform = player_query.single();

    // camera.target_position = Some(player_transform.translation);
    let mut dv = Vec3::ZERO;
    if input_action_state.pressed(InputAction::Left) {
        dv.x -= 1.0f32;
    } else if input_action_state.pressed(InputAction::Right) {
        dv.x += 1.0f32;
    }
    if input_action_state.pressed(InputAction::Up) {
        dv.y += 1.0f32;
    } else if input_action_state.pressed(InputAction::Down) {
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
