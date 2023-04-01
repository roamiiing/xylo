use std::collections::HashMap;
use std::io;

use serde::Deserialize;

use super::destinations::base::DestinationConfig;
use super::sources::base::SourceConfig;

type DestinationsMap = HashMap<String, DestinationConfig>;

#[derive(Debug, PartialEq, Deserialize, Clone)]
#[serde(untagged)]
pub enum ServiceDestinationOption {
    ConfigObject(DestinationConfig),
    AliasString(String),
}

impl ServiceDestinationOption {
    pub fn unpack_from(&self, destinations_map: &DestinationsMap) -> Option<DestinationConfig> {
        match self {
            ServiceDestinationOption::ConfigObject(obj) => Some(obj.clone()),
            ServiceDestinationOption::AliasString(str) => {
                destinations_map.get(str).map(|cfg| cfg.clone())
            }
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ServiceDestinationConfig {
    Single(ServiceDestinationOption),
    Many(Vec<ServiceDestinationOption>),
}

impl ServiceDestinationConfig {
    pub fn get_configs_from(&self, destinations_map: &DestinationsMap) -> Vec<DestinationConfig> {
        match self {
            ServiceDestinationConfig::Many(vector) => vector.clone(),
            ServiceDestinationConfig::Single(config) => vec![config.clone()],
        }
        .into_iter()
        .map(|option| option.unpack_from(destinations_map))
        .flatten()
        .collect()
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ServiceConfig {
    pub source: SourceConfig,
    pub destination: ServiceDestinationConfig,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct XyloConfig {
    pub services: HashMap<String, ServiceConfig>,
    pub destinations: Option<DestinationsMap>,
    pub env_file: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct RootConfig {
    pub xylo: XyloConfig,
}

pub fn parse_config(contents: &str) -> Result<RootConfig, io::Error> {
    serde_yaml::from_str::<RootConfig>(contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
