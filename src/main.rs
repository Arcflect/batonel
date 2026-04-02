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
        Commands::Init { preset, project_name } => {
            commands::init::execute(preset.as_deref(), project_name.as_deref());
        }
        Commands::Plan => {
            commands::plan::execute();
        }
        Commands::Scaffold => {
            commands::scaffold::execute();
        }
        Commands::Prompt { target, mode } => {
            commands::prompt::execute(&target, mode);
        }
        Commands::Verify => {
            commands::verify::execute();
        }
    }
}
