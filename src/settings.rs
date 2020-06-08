#[derive(Debug, Deserialize)]
pub struct Settings {
    host: String,
    port: u32,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::new();

        settings
            .merge(config::File::with_name("config/default"))?
            .merge(config::Environment::with_prefix("ECHO_RS"))?;

        settings.merge(config::File::with_name("settings")).ok();

        settings.try_into()
    }

    pub fn port(&self) -> u32 {
        self.port
    }

    pub fn host(&self) -> &String {
        &self.host
    }
}
