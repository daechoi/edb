use config::Config;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub log_level: String,
}

#[derive(serde::Deserialize)]
pub struct ServerSettings {
    pub id: String,
    pub addr: String,
    pub threads: usize,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config/Config"))
        .build().unwrap();

    settings.try_deserialize::<Settings>()
}
