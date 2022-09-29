use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct GoogleDriveDestConfig {
    path: String,
    token: String,
}
