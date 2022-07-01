//! src/configuration.rs
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let _base_path = std::env::current_dir().expect("Failed to determine the current directory.");
    //join solution:
    //https://github.com/damccull/ocieguide/blob/master/src/configuration.rs#L19

    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build();
    settings.unwrap().try_deserialize()
}
