use serde::Deserialize;

use super::{gdrive::GoogleDriveDestConfig, local::LocalDestConfig};

#[derive(Debug, PartialEq, Deserialize, Clone)]
#[serde(tag = "type", content = "config")]
pub enum DestinationConfig {
    #[serde(rename = "gdrive")]
    GoogleDrive(GoogleDriveDestConfig),
    #[serde(rename = "local")]
    Local(LocalDestConfig),
}
