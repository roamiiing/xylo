use serde::Deserialize;

use super::{local::LocalSourceConfig, pgsql::PgsqlSourceConfig};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum SourceConfig {
    #[serde(rename = "local")]
    Local(LocalSourceConfig),

    #[serde(rename = "pgsql")]
    Pgsql(PgsqlSourceConfig),
}
