use config::{Config, ConfigError, File};
use log::debug;
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub db_uri: String,
    pub server_port: u16,
    pub cors_allowed_origins: Vec<String>,
    pub cors_allowed_headers: Vec<String>,
    pub cors_allowed_methods: Vec<String>,
}

impl Settings {
    pub fn init_configuration() -> Result<Self, ConfigError> {
        println!("initializing settings (1)...");
        debug!("Initializing settings....");
        let environment = env::var("DEV_BOARD_ENV").unwrap_or_else(|_| "development".into());
        let filename = format!("./{}", environment);
        debug!("loading setting file {}...", &filename);
        let settings = Config::builder()
            // default settings
            .add_source(File::with_name("./default").required(true))
            // here we override previous setting
            .add_source(File::with_name(&filename).required(true))
            .build()?
            .try_deserialize();
        debug!("Settings loaded correctly");
        settings
    }
}
