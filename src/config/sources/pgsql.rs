use serde::Deserialize;

use crate::config::common::MaybeEnv;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct PgsqlSourceConfig {
    pub host: MaybeEnv,
    pub port: MaybeEnv,
    pub username: MaybeEnv,
    pub password: Option<MaybeEnv>,
}

pub struct PgsqlSourceUnpackedConfig {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
}

impl PgsqlSourceConfig {
    pub fn unpack(&self) -> Option<PgsqlSourceUnpackedConfig> {
        Some(PgsqlSourceUnpackedConfig {
            host: self.host.get()?,
            port: self.port.get()?,
            username: self.username.get()?,
            password: self
                .password
                .as_ref()
                .unwrap_or(&MaybeEnv::Value("".to_owned()))
                .get()?,
        })
    }
}
