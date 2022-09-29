mod cli;
mod config;

use clap::Parser;
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
        Ok(root_config) => println!("{:#?}", root_config),
        Err(err) => println!("{}", err),
    }
}
