use serde::Deserialize;

use crate::config::common::MaybeEnv;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct LocalSourceConfig {
    pub path: MaybeEnv,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct LocalSourceUnpackedConfig {
    pub path: String,
}

impl LocalSourceConfig {
    pub fn unpack(&self) -> Option<LocalSourceUnpackedConfig> {
        Some(LocalSourceUnpackedConfig {
            path: self.path.get()?,
        })
    }
}
