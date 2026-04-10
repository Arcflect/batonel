use clap::{Parser, Subcommand, ValueEnum};

pub mod commands;

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

#[derive(Clone, Debug, ValueEnum)]
pub enum GuardHook {
    Init,
    Plan,
    Ci,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ComplianceReportFormat {
    Json,
    Csv,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum PlanOutputFormat {
    Text,
    Json,
    Markdown,
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
    Plan {
        /// Render format for the plan output
        #[arg(long, value_enum, default_value = "text")]
        format: PlanOutputFormat,
    },
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
    /// Audit baseline architecture compliance checks for PR gate usage
    Audit {
        /// Treat warnings as failures (exit code 1)
        #[arg(long)]
        strict: bool,
    },
    /// Apply conservative automatic remediations with dry-run preview
    Fix {
        /// Preview candidate fixes without writing files
        #[arg(long)]
        dry_run: bool,
        /// Apply only low-risk fixes; review-required findings are never auto-applied
        #[arg(long)]
        apply: bool,
    },
    /// Publish a preset package into a local registry index
    PresetPublish {
        /// Preset directory path (e.g. presets/generic-layered)
        #[arg(long)]
        preset_dir: String,
        /// Local registry root directory
        #[arg(long, default_value = ".archflow/registry")]
        registry_dir: String,
    },
    /// Install a preset package from a local registry index
    PresetInstall {
        /// Preset id to install
        #[arg(long)]
        preset: String,
        /// Optional explicit version (defaults to latest compatible)
        #[arg(long = "preset-version")]
        preset_version: Option<String>,
        /// Local registry root directory
        #[arg(long, default_value = ".archflow/registry")]
        registry_dir: String,
        /// Destination directory to install into
        #[arg(long, default_value = "presets")]
        destination_dir: String,
    },
    /// Run sidecar guard checks explicitly for init/plan/ci hooks
    Guard {
        /// Hook point to evaluate
        #[arg(long, value_enum, default_value = "ci")]
        hook: GuardHook,
        /// Treat warnings as failures (exit code 1)
        #[arg(long)]
        strict: bool,
    },
    /// Verify contract-first and sidecar-first alignment of a preset directory
    PresetVerify {
        /// Preset directory to verify (e.g. presets/generic-layered)
        #[arg(long, default_value = ".")]
        preset_dir: String,
        /// Treat warnings as failures (exit code 1)
        #[arg(long)]
        strict: bool,
    },
    /// Generate a migration plan to upgrade a preset version
    PresetMigrationPlan {
        /// Preset id to migrate
        #[arg(long)]
        preset: String,
        /// Version currently used in the project
        #[arg(long)]
        from_version: String,
        /// Target preset version to migrate to
        #[arg(long)]
        to_version: String,
        /// Local registry root directory
        #[arg(long, default_value = ".archflow/registry")]
        registry_dir: String,
        /// Project directory to compare (defaults to current directory)
        #[arg(long, default_value = ".")]
        project_dir: String,
    },
    /// Apply a migration plan to upgrade a preset version
    PresetMigrationApply {
        /// Preset id to migrate
        #[arg(long)]
        preset: String,
        /// Version currently used in the project
        #[arg(long)]
        from_version: String,
        /// Target preset version to migrate to
        #[arg(long)]
        to_version: String,
        /// Local registry root directory
        #[arg(long, default_value = ".archflow/registry")]
        registry_dir: String,
        /// Project directory to apply migration into
        #[arg(long, default_value = ".")]
        project_dir: String,
        /// Preview changes without writing any files
        #[arg(long)]
        dry_run: bool,
    },
    /// Show the effective policy after applying org/team/project override precedence
    PolicyResolve {
        /// Path to org-level policy file (default: .archflow/org.policy.yaml)
        #[arg(long)]
        org_policy: Option<String>,
        /// Path to team-level policy file (default: .archflow/team.policy.yaml)
        #[arg(long)]
        team_policy: Option<String>,
        /// Path to project-level policy file (default: policy.profile.yaml)
        #[arg(long)]
        project_policy: Option<String>,
        /// Identity of the actor invoking the command, used to simulate RBAC evaluation
        #[arg(long)]
        actor: Option<String>,
    },
    /// Generate a structured fix rollout plan (JSON) from current audit findings
    FixRolloutPlan {
        /// Output file path for the generated plan
        #[arg(long, default_value = "fix-rollout-plan.json")]
        output: String,
    },
    /// Approve (or reject) items in a fix rollout plan before applying
    FixRolloutApprove {
        /// Path to the plan file generated by fix-rollout-plan
        plan_file: String,
        /// Approve (or reject) all pending review-required items
        #[arg(long)]
        all: bool,
        /// Approve (or reject) only items matching these IDs (e.g. fix-001)
        #[arg(long = "id")]
        ids: Vec<String>,
        /// Human-readable identifier recorded as the approver
        #[arg(long)]
        approver: Option<String>,
        /// Record a rejection instead of approval
        #[arg(long)]
        reject: bool,
    },
    /// Apply approved fixes from a fix rollout plan
    FixRolloutApply {
        /// Path to the approved plan file
        plan_file: String,
        /// Fail if any rejected items are present in the plan
        #[arg(long)]
        strict: bool,
    },
    /// Triage audit violations by priority and generate a prioritized remediation plan
    Triage {
        /// Limit output to the top N remediation groups
        #[arg(long)]
        top: Option<usize>,
        /// Emit the triage plan as JSON instead of human-readable text
        #[arg(long)]
        json: bool,
    },
    /// Aggregate compliance across multiple repositories and export JSON/CSV metrics
    ComplianceReport {
        /// Repository directories to audit (repeat flag to include multiple repositories)
        #[arg(long)]
        repos: Vec<String>,
        /// Optional newline-delimited file listing repository directories
        #[arg(long)]
        repos_file: Option<String>,
        /// Export format
        #[arg(long, value_enum, default_value = "json")]
        format: ComplianceReportFormat,
        /// Output file path for the generated report
        #[arg(long, default_value = "compliance-report.json")]
        output: String,
        /// Optional baseline JSON export to compute severity/rule trend deltas
        #[arg(long)]
        baseline_json: Option<String>,
    },
}

pub fn run() {
    let cli = Cli::parse();
    commands::handle(cli.command);
}
