mod cli;
mod config;
mod core;
mod utils;

use crate::core::pull::base::PullStrategy;
use crate::core::pull::pgsql::PgsqlPullStrategy;
use clap::Parser;
use colored::Colorize;
use config::sources::base::SourceConfig::Pgsql;
use path_absolutize::*;
use std::fs;
use std::path::Path;

fn main() {
    let args = cli::Args::parse();

    let path = Path::new(&args.config);

    let contents = path
        .absolutize()
        .and_then(fs::read_to_string)
        .and_then(config::parse_config);

    match contents {
        Ok(root_config) => {
            let services = &root_config.xylo.services;

            services.into_iter().for_each(|(key, config)| {
                let cyan_key = format!("{}", key).bold().bright_cyan();
                let green_key = format!("{}", key).bold().bright_green();
                let red_key = format!("{}", key).bold().bright_red();

                println!("\nℹ️  [{}] Starting to dump", cyan_key);
                let cloned_key = key.to_owned();

                let strategy = match &config.source {
                    Pgsql(config) => {
                        let cloned_config = config.clone();
                        let strategy = PgsqlPullStrategy::new(cloned_key, cloned_config);
                        strategy
                    }
                };

                let result = strategy.pull();

                match result {
                    Ok(_) => {
                        println!("✅ [{}] Successfully dumped", green_key);
                    }
                    Err(err) => {
                        println!(
                            "❌ [{}] Error happened during dump creation:\n{}",
                            red_key, err
                        )
                    }
                }
            });
        }
        Err(err) => println!("{}", err),
    }
}
