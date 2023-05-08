use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CoreAssetSet {
    pub tiles_image: Handle<Image>,
}
