mod cli;
mod config;
mod core;
mod utils;

use crate::core::pull::base::PullStrategy;
use crate::core::pull::pgsql::PgsqlPullStrategy;
use clap::Parser;
use config::sources::base::SourceConfig::Pgsql;
use path_absolutize::*;
use std::fs;
use std::path::Path;
use utils::log;

fn main() {
    let args = cli::Args::parse();

    let path = Path::new(&args.config);

    let contents = path
        .absolutize()
        .and_then(fs::read_to_string)
        .and_then(config::parse_config);

    match contents {
        Ok(root_config) => {
            let services = root_config.xylo.services;

            services.into_iter().for_each(|(key, config)| {
                log::info(&key, "Starting to dump");

                let strategy = match &config.source {
                    Pgsql(config) => PgsqlPullStrategy::new(&key, &config),
                };

                let result = strategy.pull();

                match result {
                    Ok(_) => log::success(key, "Successfully dumped"),
                    Err(err) => {
                        log::error(&key, format!("Error happened during dump creation:\n{err}"))
                    }
                }
            });
        }
        Err(err) => println!("{}", err),
    }
}
