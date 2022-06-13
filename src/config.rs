use config::Config;
use deadpool_postgres;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default)]
pub struct Configuration {
    pub host: String,
    pub port: u16,
    pub pg: deadpool_postgres::Config,
}
impl Configuration {
    pub fn from_env() -> Self {
        let _config = Config::builder()
            .add_source(::config::Environment::default())
            .build()
            .unwrap();
        let config: Configuration = _config.try_deserialize().unwrap();
        config
    }
}
