use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Xylo")]
#[command(version = "0.1.0")]
#[command(about = "Automatization, scheduling and uploading backups", long_about = None)]
pub struct Args {
    /// Path to config
    #[arg(short, long)]
    pub config: String,
}
