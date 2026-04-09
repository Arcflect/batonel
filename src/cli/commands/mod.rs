use crate::cli::{Commands, ComplianceReportFormat, GuardHook};

fn render_usecase_result(success: bool, context: &str) {
    if !success {
        eprintln!("[!] {} failed", context);
        std::process::exit(1);
    }
}

struct ComplianceReportInput {
    repos: Vec<String>,
    repos_file: Option<String>,
    format: ComplianceReportFormat,
    output: String,
    baseline_json: Option<String>,
}

struct FixRolloutApproveInput {
    plan_file: String,
    all: bool,
    ids: Vec<String>,
    approver: Option<String>,
    reject: bool,
}

pub fn handle(command: Commands) {
    match command {
        Commands::Init {
            preset,
            project_name,
            dry_run,
        } => {
            let input = crate::app::usecase::InitProjectInput {
                preset,
                project_name,
                dry_run,
            };
            let output = crate::app::usecase::InitProjectUseCase::execute(input);
            if let Some(preset) = output.resolved_preset.as_deref() {
                println!("Resolved preset: {}", preset);
            }
            render_usecase_result(output.success, "init project");
        }
        Commands::Plan => {
            let output = crate::app::usecase::PlanArchitectureUseCase::execute(
                crate::app::usecase::PlanArchitectureInput,
            );
            render_usecase_result(output.success, "plan architecture");
        }
        Commands::Scaffold => {
            let output = crate::app::usecase::GenerateArtifactsUseCase::execute(
                crate::app::usecase::GenerateArtifactsInput,
            );
            render_usecase_result(output.success, "generate artifacts");
        }
        Commands::Prompt { target, mode } => {
            crate::commands::prompt::execute(&target, mode);
        }
        Commands::Verify => {
            let output = crate::app::usecase::ValidateProjectUseCase::execute(
                crate::app::usecase::ValidateProjectInput,
            );
            render_usecase_result(output.success, "validate project");
        }
        Commands::Audit { strict } => {
            crate::commands::audit::execute(strict);
        }
        Commands::Fix { dry_run, apply } => {
            crate::commands::fix::execute(dry_run, apply);
        }
        Commands::PresetPublish {
            preset_dir,
            registry_dir,
        } => {
            crate::commands::preset_registry::publish(&preset_dir, &registry_dir);
        }
        Commands::PresetInstall {
            preset,
            preset_version,
            registry_dir,
            destination_dir,
        } => {
            crate::commands::preset_registry::install(
                &preset,
                preset_version.as_deref(),
                &registry_dir,
                &destination_dir,
            );
        }
        Commands::Guard { hook, strict } => {
            let hook_point = match hook {
                GuardHook::Init => crate::commands::guard::GuardHookPoint::Init,
                GuardHook::Plan => crate::commands::guard::GuardHookPoint::Plan,
                GuardHook::Ci => crate::commands::guard::GuardHookPoint::Ci,
            };
            crate::commands::guard::execute_cli(hook_point, strict);
        }
        Commands::PresetVerify { preset_dir, strict } => {
            crate::commands::preset_verify::execute_cli(&preset_dir, strict);
        }
        Commands::PolicyResolve {
            org_policy,
            team_policy,
            project_policy,
        } => {
            crate::commands::policy_resolve::execute_cli(
                org_policy.as_deref(),
                team_policy.as_deref(),
                project_policy.as_deref(),
            );
        }
        Commands::FixRolloutPlan { output } => {
            crate::commands::fix_rollout::execute_plan(std::path::Path::new("."), &output);
        }
        Commands::FixRolloutApprove {
            plan_file,
            all,
            ids,
            approver,
            reject,
        } => {
            let input = FixRolloutApproveInput {
                plan_file,
                all,
                ids,
                approver,
                reject,
            };
            crate::commands::fix_rollout::execute_approve(
                &input.plan_file,
                &input.ids,
                input.all,
                input.approver.as_deref(),
                input.reject,
            );
        }
        Commands::FixRolloutApply { plan_file, strict } => {
            crate::commands::fix_rollout::execute_apply(&plan_file, strict);
        }
        Commands::Triage { top, json } => {
            crate::commands::triage::execute(top, json);
        }
        Commands::ComplianceReport {
            repos,
            repos_file,
            format,
            output,
            baseline_json,
        } => {
            let input = ComplianceReportInput {
                repos,
                repos_file,
                format,
                output,
                baseline_json,
            };
            let report_format = match input.format {
                ComplianceReportFormat::Json => crate::commands::compliance_report::ReportFormat::Json,
                ComplianceReportFormat::Csv => crate::commands::compliance_report::ReportFormat::Csv,
            };
            crate::commands::compliance_report::execute_cli(
                &input.repos,
                input.repos_file.as_deref(),
                report_format,
                &input.output,
                input.baseline_json.as_deref(),
            );
        }
        Commands::PresetMigrationPlan {
            preset,
            from_version,
            to_version,
            registry_dir,
            project_dir,
        } => {
            crate::commands::preset_migrate::execute_plan_cli(
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
            crate::commands::preset_migrate::execute_apply_cli(
                &preset,
                &from_version,
                &to_version,
                &registry_dir,
                &project_dir,
                dry_run,
            );
        }
    }
}
