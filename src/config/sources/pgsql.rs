use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct PgsqlSourceConfig {
    host: Option<String>,
    port: Option<String>,
    username: String,
    password: String,
}
