use jsonwebtoken::jwk::Jwk;

use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Configuration {
    pub port: u16,
    pub address: String,
    pub signing_keys: Vec<Jwk>,
    pub default_signing_key_id: String,
    pub handle_manager_type: String,
}

impl Configuration {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("default"))
            .add_source(File::with_name(&run_mode).required(false))
            .add_source(File::with_name("local").required(false))
            .add_source(Environment::with_prefix("atproto_pds"))
            .build()?;

        s.try_deserialize()
    }

    pub(crate) fn default_signing_key(&self) -> Jwk {
        self.signing_keys
            .iter()
            .find(|jwk| jwk.common.key_id == Some(self.default_signing_key_id.clone()))
            .unwrap()
            .clone()
    }
}
