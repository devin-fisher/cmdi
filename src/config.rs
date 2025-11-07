use clap::Parser;
use config::{Config, Environment, File};
use directories::ProjectDirs;
use serde::Deserialize;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(name = "cmdi", version, about)]
pub struct Cli {
    /// The [COMMAND] line program to build
    pub cmd: String,
}

/// Main app configuration
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Settings {}

impl Default for Settings {
    fn default() -> Self {
        Self {}
    }
}

/// Load settings from defaults, file, env, and CLI
pub fn load(_cli: &Cli, directories: ProjectDirs) -> Result<Settings, config::ConfigError> {
    let config_file = directories
        .config_dir()
        .join("config");

    let builder = Config::builder()
        .add_source(
            File::with_name(
                config_file
                    .to_str()
                    .unwrap(),
            )
            .required(false),
        )
        .add_source(Environment::with_prefix("CMDI_"));

    //TODO add useful commandline args here

    builder
        .build()?
        .try_deserialize()
}
