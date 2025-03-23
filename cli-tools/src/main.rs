mod cli;
mod commands;
mod github;

use clap::Parser;
use cli::*;
use commands::*;

fn main() {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::List => list::run(),
        Commands::DeleteOld { days } => delete_old::run(days),
        Commands::CiClean { repo } => ci_clean::run(&repo),
    }
}
