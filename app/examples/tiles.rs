use std::error::Error;

use bevy::prelude::*;
use bevy::render::render_resource::{FilterMode, SamplerDescriptor};
use bevy::render::texture::ImageSampler;
use bevy::sprite::MaterialMesh2dBundle;
use defendio_app::state::AppStatePlugin;
use defendio_app::surface::{SurfacePlugin, TiledSurfaceMaterial};

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_system(bevy::window::close_on_esc)

        .add_plugin(AppStatePlugin {})

        .add_plugin(SurfacePlugin {})
        .run();
    Ok(())
}

#[derive(Resource, Default)]
struct ImageResourceSet {
    tiles_texture: Handle<Image>,
}

fn load_assets_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let tiles_texture: Handle<Image> = asset_server.load("graphics/tiles.png");

    loading.0.push(tiles_texture.clone_untyped());

    // commands.insert_resource(ImageResourceSet {
    //     tiles_texture,
    // });
}

fn check_assets_ready(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    use bevy::asset::LoadState;

    match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            // one of our assets had an error
        }
        LoadState::Loaded => {
            // all assets are now ready

            // this might be a good place to transition into your in-game state

            // remove the resource to drop the tracking handles
            commands.remove_resource::<AssetsLoading>();
            // (note: if you don't have any other handles to the assets
            // elsewhere, they will get unloaded after this)
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}

fn update_images_system(
    mut images: ResMut<Assets<Image>>,
    image_resource_set: Res<ImageResourceSet>,
) {
    // let image = images.get_mut(&image_resource_set.tiles_texture).unwrap();
    // image.sampler_descriptor = ImageSampler::nearest();
}

fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TiledSurfaceMaterial>>,
    // asset_server: Res<AssetServer>,
    image_resource_set: Res<ImageResourceSet>,
) {
    let tiled_mesh = Mesh::from(shape::Quad::new(Vec2::new(1.0f32, 1.0f32)));

    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(tiled_mesh).into(),
        transform: Transform::default().with_scale(Vec3::splat(400.0f32)),
        material: materials.add(
            TiledSurfaceMaterial {
                texture: image_resource_set.tiles_texture.clone(),
            }
        ),
        ..Default::default()
    });
}

// fn post_startup_system(
//     mut ev_asset: EventReader<AssetEvent<Image>>,
//     mut images: ResMut<Assets<Image>>,
//     image_resource_set: Res<ImageResourceSet>,
// ) {
//
//     let image = images.get_mut(&image_resource_set.handle).unwrap();
//     image.sampler_descriptor = ImageSampler::nearest();
//
//     // for ev in ev_asset.iter() {
//     //     if let AssetEvent::Created { handle } = ev {
//     //         // if *handle == image_resource_set.handle {
//     //         let image = images.get_mut(handle).unwrap();
//     //         image.sampler_descriptor = ImageSampler::linear();
//     //         println!("{}", 42);
//     //         // }
//     //     }
//     // }
// }
