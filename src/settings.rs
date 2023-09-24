use config::{Config, ConfigError, File, Environment};
use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Address {
    pub name: String,
    pub public: bool,
    pub host: String,
    pub proxy: Option<String>,
    pub port: Option<i64>,
    pub ports: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub addresses: Vec<Address>
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("to-check"))
            .add_source(
                Environment::with_prefix("APP")
                .separator("_")
            )
            .build()?;
        s.try_deserialize()
    }
}