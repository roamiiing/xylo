use serde::Deserialize;

use super::local::LocalDestConfig;
use super::s3::S3DestConfig;

#[derive(Debug, PartialEq, Deserialize, Clone)]
#[serde(tag = "type", content = "config")]
pub enum DestinationConfig {
    #[serde(rename = "local")]
    Local(LocalDestConfig),

    #[serde(rename = "s3")]
    S3(S3DestConfig),
}
