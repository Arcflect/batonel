mod cli;
mod commands;
mod config;
mod generator;
pub mod model;

use clap::Parser;
use cli::{Cli, Commands, ComplianceReportFormat, GuardHook};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            preset,
            project_name,
            dry_run,
        } => {
            commands::init::execute(preset.as_deref(), project_name.as_deref(), dry_run);
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
        Commands::Audit { strict } => {
            commands::audit::execute(strict);
        }
        Commands::Fix { dry_run, apply } => {
            commands::fix::execute(dry_run, apply);
        }
        Commands::PresetPublish {
            preset_dir,
            registry_dir,
        } => {
            commands::preset_registry::publish(&preset_dir, &registry_dir);
        }
        Commands::PresetInstall {
            preset,
            preset_version,
            registry_dir,
            destination_dir,
        } => {
            commands::preset_registry::install(
                &preset,
                preset_version.as_deref(),
                &registry_dir,
                &destination_dir,
            );
        }
        Commands::Guard { hook, strict } => {
            let hook_point = match hook {
                GuardHook::Init => commands::guard::GuardHookPoint::Init,
                GuardHook::Plan => commands::guard::GuardHookPoint::Plan,
                GuardHook::Ci => commands::guard::GuardHookPoint::Ci,
            };
            commands::guard::execute_cli(hook_point, strict);
        }
        Commands::PresetVerify { preset_dir, strict } => {
            commands::preset_verify::execute_cli(&preset_dir, strict);
        }
        Commands::PolicyResolve {
            org_policy,
            team_policy,
            project_policy,
        } => {
            commands::policy_resolve::execute_cli(
                org_policy.as_deref(),
                team_policy.as_deref(),
                project_policy.as_deref(),
            );
        }
        Commands::ComplianceReport {
            repos,
            repos_file,
            format,
            output,
            baseline_json,
        } => {
            let report_format = match format {
                ComplianceReportFormat::Json => {
                    commands::compliance_report::ReportFormat::Json
                }
                ComplianceReportFormat::Csv => {
                    commands::compliance_report::ReportFormat::Csv
                }
            };
            commands::compliance_report::execute_cli(
                &repos,
                repos_file.as_deref(),
                report_format,
                &output,
                baseline_json.as_deref(),
            );
        }
        Commands::PresetMigrationPlan {
            preset,
            from_version,
            to_version,
            registry_dir,
            project_dir,
        } => {
            commands::preset_migrate::execute_plan_cli(
                &preset,
                &from_version,
                &to_version,
                &registry_dir,
                &project_dir,
            );
        }
        Commands::PresetMigrationApply {
            preset,
            from_version,
            to_version,
            registry_dir,
            project_dir,
            dry_run,
        } => {
            commands::preset_migrate::execute_apply_cli(
                &preset,
                &from_version,
                &to_version,
                &registry_dir,
                &project_dir,
                dry_run,
            );
        }
        Commands::Triage { top, json } => {
            commands::triage::execute(top, json);
        }
        Commands::FixRolloutPlan { output } => {
            commands::fix_rollout::execute_plan(std::path::Path::new("."), &output);
        }
        Commands::FixRolloutApprove {
            plan_file,
            all,
            ids,
            approver,
            reject,
        } => {
            commands::fix_rollout::execute_approve(
                &plan_file,
                &ids,
                all,
                approver.as_deref(),
                reject,
            );
        }
        Commands::FixRolloutApply { plan_file, strict } => {
            commands::fix_rollout::execute_apply(&plan_file, strict);
        }
    }
}
