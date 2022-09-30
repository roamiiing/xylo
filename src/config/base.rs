use std::collections::HashMap;
use std::io;

use serde::Deserialize;

use super::destinations::base::DestinationConfig;
use super::sources::base::SourceConfig;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ServiceDestinationOption {
    ConfigObject(DestinationConfig),
    AliasString(String),
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ServiceDestinationConfig {
    Single(ServiceDestinationOption),
    Many(Vec<ServiceDestinationOption>),
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ServiceConfig {
    pub source: SourceConfig,
    pub destination: ServiceDestinationConfig,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct XyloConfig {
    pub services: HashMap<String, ServiceConfig>,
    pub destinations: Option<HashMap<String, DestinationConfig>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct RootConfig {
    pub xylo: XyloConfig,
}

pub fn parse_config(contents: String) -> Result<RootConfig, io::Error> {
    serde_yaml::from_str::<RootConfig>(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
