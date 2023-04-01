use serde::Deserialize;

use crate::config::common::MaybeEnv;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct ScpDestConfig {
    pub host: MaybeEnv,
    pub port: Option<MaybeEnv>,
    pub user: MaybeEnv,
    pub password: MaybeEnv,
    pub path: MaybeEnv,
}

pub struct ScpDestUnpackedConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub path: String,
}

impl ScpDestConfig {
    pub fn unpack(&self) -> Option<ScpDestUnpackedConfig> {
        Some(ScpDestUnpackedConfig {
            host: self.host.get()?,
            port: self
                .port
                .as_ref()
                .unwrap_or(&MaybeEnv::Value("22".to_owned()))
                .get()?
                .parse()
                .ok()?,
            user: self.user.get()?,
            password: self.password.get()?,
            path: self.path.get()?,
        })
    }
}
