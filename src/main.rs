mod cli;
mod commands;
mod config;
mod generator;
pub mod model;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            commands::init::execute();
        }
        Commands::Plan => {
            commands::plan::execute();
        }
        Commands::Scaffold => {
            commands::scaffold::execute();
        }
        Commands::Prompt { target } => {
            commands::prompt::execute(&target);
        }
    }
}
