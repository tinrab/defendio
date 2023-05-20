use bevy::math::Affine3A;
use std::error::Error;

use bevy::prelude::*;
use bevy::render::render_resource::{FilterMode, SamplerDescriptor};
use bevy::render::texture::ImageSampler;
use bevy::render::view::RenderLayers;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResolution;
use defendio_app::asset::TilemapAssetGroup;
use defendio_app::camera::{MainCameraBundle, MainCameraComponent, MainCameraPlugin};
use defendio_app::interaction::input_action::InputActionPlugin;
use defendio_app::interaction::{InteractionPlugin, MousePosition};
use defendio_app::lighting::{LightBundle, LightingPlugin};
use defendio_app::plugin::AppCorePlugin;
use defendio_app::state::AppState;
use defendio_app::tilemap::bundle::TilemapBundle;
use defendio_app::tilemap::material::TilemapMaterial;
use defendio_app::tilemap::plugin::TilemapPlugin;
use defendio_app::world_material::material::WorldMaterial;

#[derive(Component)]
struct LightMovement {}

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280.0, 720.0)
                            .with_scale_factor_override(1.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .add_system(bevy::window::close_on_esc)
        .add_plugin(AppCorePlugin)
        .add_system(on_game_state_enter.in_schedule(OnEnter(AppState::Game)))
        .add_system(on_game_state_update.in_set(OnUpdate(AppState::Game)))
        .add_system(move_light.in_set(OnUpdate(AppState::Game)))
        .run();
    Ok(())
}

fn on_game_state_enter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TilemapMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut world_materials: ResMut<Assets<WorldMaterial>>,
    mut images: Res<Assets<Image>>,
    tilemap_asset_group: Res<TilemapAssetGroup>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(TilemapBundle::make(
        &tilemap_asset_group,
        materials,
        meshes.as_mut(),
        &images,
        &texture_atlases,
    ));

    // for _ in 0..30 {
    //     commands.spawn(LightBundle::new(
    //         Vec3::new(
    //             rand::random::<f32>() * 50.0,
    //             rand::random::<f32>() * 50.0,
    //             0.0,
    //         ),
    //         rand::random::<f32>() * 20.0 + 5.0,
    //         Color::hsl(rand::random::<f32>() * 360.0, 0.6, 0.8),
    //     ));
    // }

    commands.spawn((
        LightBundle::new(Vec3::new(5.0, 0.0, 0.0), 10.0, Color::WHITE * 10.0),
        LightMovement {},
    ));

    let monkey_d: Handle<Image> = asset_server.load("graphics/monkey-d.png");
    let monkey_n: Handle<Image> = asset_server.load("graphics/monkey-n.png");
    for i in 0..=5 {
        commands.spawn(
            (MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(10.0, 10.0)).into())
                    .into(),
                material: world_materials.add(WorldMaterial {
                    base_color: Color::RED,
                    base_color_texture: Some(monkey_d.clone()),
                    normal_texture: Some(monkey_n.clone()),
                    ..Default::default()
                }),
                transform: Transform::from_translation(Vec3::new(
                    i as f32 * 10.0,
                    10.0,
                    rand::random::<f32>() * 0.1,
                ))
                .with_scale(Vec3::new(2.0, 2.0, 1.0)),
                ..Default::default()
            }),
        );
    }
}

fn on_game_state_update() {
    // std::thread::sleep(std::time::Duration::from_millis(500));
}

fn move_light(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCameraComponent>>,
    mut light_query: Query<(&mut Transform, &LightMovement)>,
    mouse_position: Res<MousePosition>,
) {
    let mut light = if let Ok((t, _)) = light_query.get_single_mut() {
        t
    } else {
        return;
    };

    let (camera, camera_transform) = camera_query.single();
    if let Some(world_position) = camera
        .viewport_to_world(camera_transform, mouse_position.0)
        .map(|ray| ray.origin.truncate())
    {
        light.translation = world_position.extend(0.0);
    }
}
