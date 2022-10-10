use serde::Deserialize;

use super::local::LocalDestConfig;

#[derive(Debug, PartialEq, Deserialize, Clone)]
#[serde(tag = "type", content = "config")]
pub enum DestinationConfig {
    #[serde(rename = "local")]
    Local(LocalDestConfig),
}
