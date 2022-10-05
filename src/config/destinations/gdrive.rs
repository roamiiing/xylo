use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct GoogleDriveDestConfig {
    pub path: String,
    pub token: String,
}
