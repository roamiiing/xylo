use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct LocalDestConfig {
    pub path: String,
}
