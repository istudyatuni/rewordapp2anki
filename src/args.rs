use clap::Parser;

/// Convert words lists from Reword apps
#[derive(Debug, Parser)]
pub struct Cli {
    /// Do not use cached extracted data
    #[arg(long)]
    pub no_cache: bool,
}
