use serde::Deserialize;

use crate::config::common::MaybeEnv;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct S3DestConfig {
    pub endpoint_url: Option<MaybeEnv>,
    pub bucket: MaybeEnv,
    pub region: MaybeEnv,
    pub access_key: MaybeEnv,
    pub secret_key: MaybeEnv,
    pub path: MaybeEnv,
}

pub struct S3DestUnpackedConfig {
    pub endpoint: String,
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub path: String,
}

impl S3DestConfig {
    pub fn unpack(&self) -> Option<S3DestUnpackedConfig> {
        Some(S3DestUnpackedConfig {
            endpoint: self
                .endpoint_url
                .as_ref()
                .unwrap_or(&MaybeEnv::Value("https://s3.amazonaws.com".to_owned()))
                .get()?,
            bucket: self.bucket.get()?,
            region: self.region.get()?,
            access_key: self.access_key.get()?,
            secret_key: self.secret_key.get()?,
            path: self.path.get()?,
        })
    }
}
