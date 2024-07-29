use config::Config;
use std::path::PathBuf;
use clap::{arg, value_parser, Command};


#[derive(serde::Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub log_level: String,
}

#[derive(serde::Deserialize)]
pub struct ServerSettings {
    pub id: String,
    pub port: u16,
    pub threads: usize,
    pub addr: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let matches = Command::new("esql")
        .version("0.1.0")
        .arg(arg!(-c --config <FILE> "Sets a custom file")
        .required(false)
        .value_parser(value_parser!(PathBuf))).get_matches();

    let mut config_file = "config/Config";
    if let Some(filename) = matches.get_one::<PathBuf>("config") {
        log::info!("Using configuration file: {:?}", filename);
        config_file = filename.to_str().unwrap();
    }

    let settings = Config::builder()
        .add_source(config::File::with_name(config_file))
        .build().unwrap();

    settings.try_deserialize::<Settings>()
}
