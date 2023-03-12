use bevy::prelude::*;

use crate::constants::WORLD_SCALE;

pub fn make_camera_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(move_camera)
        .with_system(follow_target)
}

#[derive(Component)]
pub struct MainCameraComponent {
    target_position: Option<Vec3>,
}

#[derive(Bundle)]
struct CameraBundle {
    camera: MainCameraComponent,
    #[bundle]
    camera2d: Camera2dBundle,
}

const CAMERA_BASE_SPEED: f32 = 7.0f32 * WORLD_SCALE;

pub fn spawn_camera(
    mut commands: Commands, ) {
    commands.spawn(CameraBundle {
        camera: MainCameraComponent {
            target_position: None,
        },
        camera2d: Camera2dBundle {
            projection: OrthographicProjection {
                far: 1000.0,
                scale: 0.03 * WORLD_SCALE,
                ..Default::default()
            },
            ..Camera2dBundle::default()
        },
    });
}

// fn move_camera(
//     mut query: Query<&mut MainCameraComponent, Without<PlayerComponent>>,
//     player_query: Query<&Transform, With<PlayerComponent>>,
// ) {
//     let mut camera = query.single_mut();
//     let player_transform = player_query.single();
//
//     camera.target_position = Some(player_transform.translation);
// }

// fn follow_target(
//     mut query: Query<(&mut MainCameraComponent, &mut Transform)>,
//     time: Res<Time>,
// ) {
//     let (camera, mut transform) = query.single_mut();
//
//     if let Some(target_position) = camera.target_position {
//         let mut dv = ((target_position - transform.translation) / 200.0);
//         dv.z = 0.0;
//         transform.translation += (dv) * CAMERA_BASE_SPEED * time.delta_seconds();
//     }
// }
