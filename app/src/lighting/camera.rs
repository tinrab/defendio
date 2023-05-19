use crate::camera::{MainCameraBundle, MainCameraComponent};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::{CameraOutputMode, RenderTarget};
use bevy::render::render_resource::BlendState;
use bevy::render::view::RenderLayers;

#[derive(Component)]
pub struct LightCameraComponent;

#[derive(Bundle)]
pub struct LightCameraBundle {
    camera: LightCameraComponent,
    #[bundle]
    camera2d: Camera2dBundle,
    ui_camera: UiCameraConfig,
}

impl LightCameraBundle {
    pub fn new(map_image: Handle<Image>) -> Self {
        let main_camera = MainCameraBundle::new();
        LightCameraBundle {
            camera: LightCameraComponent,
            camera2d: Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    order: -1,
                    target: RenderTarget::Image(map_image),
                    ..Default::default()
                },
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::rgb(0.4, 0.4, 0.4)),
                },
                ..main_camera.camera2d
            },
            ui_camera: UiCameraConfig { show_ui: false },
        }
    }
}

pub fn light_camera_update(
    mut light_camera_query: Query<(&mut Transform, &LightCameraComponent)>,
    mut main_camera_query: Query<
        &Transform,
        (With<MainCameraComponent>, Without<LightCameraComponent>),
    >,
) {
    if let Ok((mut light_camera, _)) = light_camera_query.get_single_mut() {
        if let Ok(main_camera) = main_camera_query.get_single() {
            *light_camera = main_camera.clone();
        }
    }
}
