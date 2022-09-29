use serde::Deserialize;

use super::pgsql::PgsqlSourceConfig;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum SourceConfig {
    #[serde(rename = "pgsql")]
    Pgsql(PgsqlSourceConfig),
}
