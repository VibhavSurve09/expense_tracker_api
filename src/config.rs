use config::Config;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let _config = Config::builder()
            .add_source(::config::Environment::default())
            .build()
            .unwrap();
        let config: ServerConfig = _config.try_deserialize().unwrap();
        config
    }
}
