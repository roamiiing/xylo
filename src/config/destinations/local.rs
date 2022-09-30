use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct LocalDestConfig {
    pub path: String,
}
