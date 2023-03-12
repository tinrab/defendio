use std::error::Error;

use bevy::prelude::*;

use defendio_app::config::AppConfig;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let config = AppConfig::load()?;
    println!("{} {}", config.distribution.name, config.distribution.version);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .run();

    Ok(())
}
