use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct GoogleDriveDestConfig {
    pub path: String,
    pub token: String,
}
