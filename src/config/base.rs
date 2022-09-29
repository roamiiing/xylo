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
    source: SourceConfig,
    destination: ServiceDestinationConfig,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct XyloConfig {
    services: HashMap<String, ServiceConfig>,
    destinations: HashMap<String, DestinationConfig>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct RootConfig {
    xylo: XyloConfig,
}

pub fn parse_config(contents: String) -> Result<RootConfig, io::Error> {
    serde_yaml::from_str::<RootConfig>(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
