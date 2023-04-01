use serde::Deserialize;

use crate::config::common::MaybeEnv;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct PgsqlSourceConfig {
    pub host: MaybeEnv,
    pub port: MaybeEnv,
    pub username: MaybeEnv,
    pub password: Option<MaybeEnv>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unpack() {
        let config = PgsqlSourceConfig {
            host: MaybeEnv::Value("localhost".to_owned()),
            port: MaybeEnv::Value("5432".to_owned()),
            username: MaybeEnv::Value("postgres".to_owned()),
            password: Some(MaybeEnv::Value("password".to_owned())),
        };

        let unpacked = config.unpack().unwrap();

        assert_eq!(
            unpacked,
            PgsqlSourceUnpackedConfig {
                host: "localhost".to_owned(),
                port: "5432".to_owned(),
                username: "postgres".to_owned(),
                password: "password".to_owned(),
            }
        );
    }
}
