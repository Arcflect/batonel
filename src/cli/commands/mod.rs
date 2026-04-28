use crate::cli::{Commands, ComplianceReportFormat, GuardHook, PlanOutputFormat};
use crate::cli::runner;

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
        // ==========================================
        // Primary Workflow Commands
        // ==========================================
        Commands::Init {
            preset,
            project_name,
            dry_run,
        } => {
            let input = crate::app::usecase::InitProjectInput {
                preset: preset.clone(),
                project_name,
                dry_run,
            };

            if let Some(preset_id) = preset.as_deref() {
                runner::print_command_header(&format!("Batonel Initialization (preset: {})", preset_id));
            } else {
                runner::print_command_header("Batonel Initialization");
            }

            if dry_run {
                println!("  [i] Dry run mode enabled. No files will be written.");
            }

            let output = runner::run_usecase("init project", || {
                crate::app::usecase::InitProjectUseCase::execute(input)
            });

            for result in &output.file_results {
                if dry_run {
                    if result.created {
                        println!("  [plan] create {}", result.filename);
                    } else {
                        println!("  [plan] skip {} (already exists)", result.filename);
                    }
                } else if result.created {
                    println!("  [+] Generated {}", result.filename);
                } else {
                    println!("  [~] {} already exists, skipping.", result.filename);
                }
            }

            println!();
            if dry_run {
                println!(
                    "Dry run complete. {} file(s) would be generated, {} file(s) would be skipped.",
                    output.generated_count, output.skipped_count
                );
                println!("Review the plan above, then run the same command without --dry-run to generate files.");
            } else if output.generated_count > 0 {
                println!("Initialization complete! Explore your configuration files, then run:");
                println!("  batonel plan");
                println!("  batonel scaffold");
            } else {
                println!("Initialization finished. No new configuration files were generated.");
            }

            runner::exit_on_failure(output.success, "init project");
        }
        Commands::Plan { format } => {
            let usecase_format = match format {
                PlanOutputFormat::Text => crate::app::usecase::PlanRenderFormat::Text,
                PlanOutputFormat::Json => crate::app::usecase::PlanRenderFormat::Json,
                PlanOutputFormat::Markdown => crate::app::usecase::PlanRenderFormat::Markdown,
            };

            let usecase_output = runner::run_usecase("plan architecture", || {
                crate::app::usecase::PlanArchitectureUseCase::execute(
                    crate::app::usecase::PlanArchitectureInput {
                        format: usecase_format,
                    },
                )
            });

            let rendered = match format {
                PlanOutputFormat::Text => {
                    crate::infra::PlanRendererAdapter::render_text(&usecase_output)
                }
                PlanOutputFormat::Json => {
                    runner::run_usecase("render json output", || {
                        crate::infra::PlanRendererAdapter::render_json(&usecase_output)
                    })
                }
                PlanOutputFormat::Markdown => {
                    crate::infra::PlanRendererAdapter::render_markdown(&usecase_output)
                }
            };

            runner::write_output(&rendered);
            runner::exit_on_failure(usecase_output.success, "plan architecture");
        }
        Commands::Scaffold => {
            let output = runner::run_usecase("generate artifacts", || {
                crate::app::usecase::GenerateArtifactsUseCase::execute(
                    crate::app::usecase::GenerateArtifactsInput,
                )
            });
            println!();
            println!(
                "Scaffold result: {} generated, {} errors.",
                output.generated_count, output.error_count
            );
            runner::exit_on_failure(output.success, "generate artifacts");
        }
        Commands::Verify => {
            runner::print_command_header("Batonel Architectural Verification");

            let output = runner::run_usecase("validate project", || {
                crate::app::usecase::ValidateProjectUseCase::execute(
                    crate::app::usecase::ValidateProjectInput,
                )
            });

            if output.structural_errors > 0 || output.structural_warnings > 0 {
                println!(
                    "Structural validation: {} error(s), {} warning(s)",
                    output.structural_errors, output.structural_warnings
                );
            }

            crate::commands::verify::render_report(&output.report);
            runner::exit_on_failure(output.success, "validate project");
        }

        // ==========================================
        // Advanced Workflow Commands
        // ==========================================
        Commands::Prompt { target, mode } => {
            crate::commands::prompt::execute(&target, mode);
        }
        Commands::Handoff { target } => {
            crate::commands::handoff::execute(&target);
        }
        Commands::Triage { top, json } => {
            crate::commands::triage::execute(top, json);
        }

        // ==========================================
        // Governance Commands
        // ==========================================
        Commands::Audit { strict, evidence_export } => {
            crate::commands::audit::execute(strict, evidence_export);
        }
        Commands::Guard { hook, strict } => {
            let hook_point = match hook {
                GuardHook::Init => crate::commands::guard::GuardHookPoint::Init,
                GuardHook::Plan => crate::commands::guard::GuardHookPoint::Plan,
                GuardHook::Ci => crate::commands::guard::GuardHookPoint::Ci,
            };
            crate::commands::guard::execute_cli(hook_point, strict);
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
        Commands::Fix { dry_run, apply } => {
            crate::commands::fix::execute(dry_run, apply);
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
        Commands::PolicyResolve {
            org_policy,
            team_policy,
            project_policy,
            actor,
        } => {
            crate::commands::policy_resolve::execute_cli(
                org_policy.as_deref(),
                team_policy.as_deref(),
                project_policy.as_deref(),
                actor.as_deref(),
            );
        }

        // ==========================================
        // Preset Commands
        // ==========================================
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
        Commands::PresetPublish {
            preset_dir,
            registry_dir,
        } => {
            crate::commands::preset_registry::publish(&preset_dir, &registry_dir);
        }
        Commands::PresetVerify { preset_dir, strict } => {
            crate::commands::preset_verify::execute_cli(&preset_dir, strict);
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
