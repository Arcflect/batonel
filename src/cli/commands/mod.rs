use crate::cli::{Commands, ComplianceReportFormat, GuardHook, PlanOutputFormat};

fn render_usecase_result(success: bool, context: &str) {
    if !success {
        eprintln!("[!] {} failed", context);
        std::process::exit(1);
    }
}

fn write_rendered_output(renderer_output: &str) {
    let mut output = crate::infra::ConsoleOutputAdapter;
    for line in renderer_output.lines() {
        crate::ports::OutputPort::write_line(&mut output, crate::ports::OutputLevel::Info, line);
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
            let output = match crate::app::usecase::InitProjectUseCase::execute(input) {
                Ok(output) => output,
                Err(err) => {
                    eprintln!("[!] init project failed: {}", err);
                    std::process::exit(1);
                }
            };
            if let Some(preset) = output.resolved_preset.as_deref() {
                println!("Resolved preset: {}", preset);
            }
            render_usecase_result(output.success, "init project");
        }
        Commands::Plan { format } => {
            let usecase_format = match format {
                PlanOutputFormat::Text => crate::app::usecase::PlanRenderFormat::Text,
                PlanOutputFormat::Json => crate::app::usecase::PlanRenderFormat::Json,
                PlanOutputFormat::Markdown => crate::app::usecase::PlanRenderFormat::Markdown,
            };

            let usecase_output = match crate::app::usecase::PlanArchitectureUseCase::execute(
                crate::app::usecase::PlanArchitectureInput {
                    format: usecase_format,
                },
            ) {
                Ok(result) => result,
                Err(err) => {
                    eprintln!("[!] plan architecture failed: {}", err);
                    std::process::exit(1);
                }
            };

            let rendered = match format {
                PlanOutputFormat::Text => {
                    crate::infra::PlanRendererAdapter::render_text(&usecase_output)
                }
                PlanOutputFormat::Json => {
                    match crate::infra::PlanRendererAdapter::render_json(&usecase_output) {
                        Ok(s) => s,
                        Err(err) => {
                            eprintln!("[!] failed to render json output: {}", err);
                            std::process::exit(1);
                        }
                    }
                }
                PlanOutputFormat::Markdown => {
                    crate::infra::PlanRendererAdapter::render_markdown(&usecase_output)
                }
            };

            write_rendered_output(&rendered);
            render_usecase_result(usecase_output.success, "plan architecture");
        }
        Commands::Scaffold => {
            let output = match crate::app::usecase::GenerateArtifactsUseCase::execute(
                crate::app::usecase::GenerateArtifactsInput,
            ) {
                Ok(output) => output,
                Err(err) => {
                    eprintln!("[!] generate artifacts failed: {}", err);
                    std::process::exit(1);
                }
            };
            render_usecase_result(output.success, "generate artifacts");
        }
        Commands::Prompt { target, mode } => {
            crate::commands::prompt::execute(&target, mode);
        }
        Commands::Verify => {
            let mut output_adapter = crate::infra::ConsoleOutputAdapter;
            let output = match crate::app::usecase::ValidateProjectUseCase::execute_with_output(
                crate::app::usecase::ValidateProjectInput,
                &mut output_adapter,
            ) {
                Ok(output) => output,
                Err(err) => {
                    eprintln!("[!] validate project failed: {}", err);
                    std::process::exit(1);
                }
            };
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
            actor,
        } => {
            crate::commands::policy_resolve::execute_cli(
                org_policy.as_deref(),
                team_policy.as_deref(),
                project_policy.as_deref(),
                actor.as_deref(),
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
