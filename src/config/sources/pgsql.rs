use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct PgsqlSourceConfig {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: Option<String>,
}
