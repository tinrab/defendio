use std::env;
use std::path::Path;

use config::{Config, Environment, File};
use once_cell::sync::OnceCell;
use serde::Deserialize;

use crate::error::AppResult;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_PATH_ENV: &str = "DEFENDIO_CONFIG_PATH";
const ENV_PREFIX: &str = "DEFENDIO";
const DISTRIBUTION_VERSION_KEY: &str = "distribution.version";

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub distribution: DistributionConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DistributionConfig {
    pub name: String,
    pub version: String,
}

impl AppConfig {
    pub fn get() -> &'static Self {
        static INSTANCE: OnceCell<AppConfig> = OnceCell::new();
        INSTANCE.get_or_init(|| Self::load().unwrap())
    }

    pub fn load() -> AppResult<Self> {
        let mut config_builder =
            Config::builder().set_default(DISTRIBUTION_VERSION_KEY, VERSION)?;

        // Initial "default" configuration file
        let default_path = Path::new("config").join("default");
        config_builder = config_builder
            .add_source(File::with_name(default_path.to_str().unwrap()).required(false));

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        let local_path = Path::new("config").join("local");
        config_builder = config_builder
            .add_source(File::with_name(local_path.to_str().unwrap()).required(false));

        // Add override settings file.
        if let Some(override_path) = env::var(CONFIG_PATH_ENV).ok() {
            config_builder =
                config_builder.add_source(File::with_name(&override_path).required(false));
        }

        // Add in settings from the environment (with a prefix of APP)
        config_builder =
            config_builder.add_source(Environment::with_prefix(ENV_PREFIX).separator("__"));

        config_builder
            .build()?
            .try_deserialize()
            .map_err(Into::into)
    }
}
