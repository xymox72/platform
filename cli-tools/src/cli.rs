use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ghcr")]
#[command(about = "GHCR Tools CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all GHCR packages
    List,
    /// Delete old versions
    DeleteOld {
        #[arg(short, long, default_value = "30")]
        days: i64,
    },
    CiClean {
        #[arg(long, default_value = "")]
        repo: String,
    },
}
