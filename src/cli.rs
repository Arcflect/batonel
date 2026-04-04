use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author = "hirontan", version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum OutputMode {
    #[default]
    Standard,
    Compact,
}

impl std::fmt::Display for OutputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputMode::Standard => write!(f, "standard"),
            OutputMode::Compact => write!(f, "compact"),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Archflow project configuration
    Init {
        /// Preset id under presets/ (e.g. generic-layered, rust-clean-hexagonal)
        #[arg(long)]
        preset: Option<String>,
        /// Override project.name in project.arch.yaml during initialization
        #[arg(long)]
        project_name: Option<String>,
        /// Preview generated files without writing them to disk
        #[arg(long)]
        dry_run: bool,
    },
    /// Plan the architecture based on contracts and schemas
    Plan,
    /// Scaffold code structure and artifact sidecars
    Scaffold,
    /// Generate an AI handoff prompt for a specific artifact or contract
    Prompt {
        /// The artifact name or path to the .contract.yaml file
        target: String,
        /// The format required for the output
        #[arg(short, long, default_value_t = OutputMode::Standard)]
        mode: OutputMode,
    },
    /// Verify the consistency of project structure and artifact contracts
    Verify,
}
