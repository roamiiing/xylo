mod cli;
mod config;
mod core;
mod utils;

use clap::Parser;
use colored::Colorize;
use config::base::ServiceConfig;
use config::destinations::base::DestinationConfig;
use path_absolutize::*;
use std::{collections::HashMap, fs, io, path::Path};

use crate::core::archive::create_archive;
use crate::core::common::cleanup;
use crate::core::pull::{base::PullStrategy, create_pull_strategy};
use crate::core::push::{base::PushStrategy, create_push_strategy};
use crate::utils::log;

fn process_config(
    key: String,
    config: &ServiceConfig,
    destinations_map: &HashMap<String, DestinationConfig>,
) -> Result<(), io::Error> {
    let log_error = |error: io::Error| log::error(&key, error.to_string());

    log::info(&key, "Creating dump");

    let strategy: Box<dyn PullStrategy> = create_pull_strategy(&key, &config.source);

    let meta = strategy.pull()?;

    log::info(&key, format!("Dump: {}", meta.to_string().bright_cyan()));
    log::info(&key, "Creating archive");
    create_archive(&meta)?;

    log::info(&key, "Pushing archive");

    let push_strategies: Vec<Box<dyn PushStrategy>> = config
        .destination
        .get_configs_from(&destinations_map)
        .into_iter()
        .map(|config| create_push_strategy(&config, &meta))
        .collect();

    for strategy in push_strategies {
        strategy.push().map_err(log_error).ok();
        log::success(&key, "Successfully pushed")
    }

    log::info(&key, "Cleaning up");
    cleanup(&meta)?;

    log::success(&key, "Successfully dumped (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧");

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let args = cli::Args::parse();

    let path = Path::new(&args.config);

    let root_config = path
        .absolutize()
        .and_then(fs::read_to_string)
        .and_then(config::parse_config)?;

    let services = root_config.xylo.services;
    let destinations = root_config
        .xylo
        .destinations
        .unwrap_or_else(|| HashMap::new());

    for (key, config) in services {
        let log_error = |error: io::Error| log::error(&key, error.to_string());

        process_config(key.clone(), &config, &destinations)
            .map_err(log_error)
            .ok();
    }

    Ok(())
}
