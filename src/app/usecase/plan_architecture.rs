#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanRenderFormat {
    Text,
    Json,
    Markdown,
}

#[derive(Debug, Clone)]
pub struct PlanArchitectureInput {
    pub format: PlanRenderFormat,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GuardFindingOutput {
    pub rule_id: String,
    pub severity: String,
    pub target: String,
    pub message: String,
    pub remediation: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlanArchitectureOutput {
    pub plan: crate::domain::planning::ArchitecturePlan,
    pub guard_findings: Vec<GuardFindingOutput>,
    pub guard_errors: usize,
    pub guard_warnings: usize,
    pub success: bool,
}

pub struct PlanArchitectureUseCase;

impl PlanArchitectureUseCase {
    pub fn execute(input: PlanArchitectureInput) -> Result<PlanArchitectureOutput, String> {
        let _requested_format = input.format;

        let guard_report = crate::commands::guard::run_hook(
            crate::commands::guard::GuardHookPoint::Plan,
            None,
        );

        let project = crate::config::ProjectConfig::load("project.arch.yaml")
            .map_err(|e| format!("Error loading project: {}", e))?;
        let placement = crate::config::PlacementRulesConfig::load("placement.rules.yaml")
            .map_err(|e| format!("Error loading placement rules: {}", e))?;
        let artifacts = crate::config::ArtifactsPlanConfig::load("artifacts.plan.yaml")
            .map_err(|e| format!("Error loading artifacts plan: {}", e))?;

        let context = crate::domain::project::ProjectContext::from_project_config(&project);
        let plan = crate::domain::planning::ArchitecturePlanner::plan(&context, &placement, &artifacts);

        let guard_findings: Vec<GuardFindingOutput> = guard_report
            .findings
            .iter()
            .map(|f| GuardFindingOutput {
                rule_id: f.rule_id.to_string(),
                severity: match f.severity {
                    crate::commands::guard::GuardSeverity::Error => "error".to_string(),
                    crate::commands::guard::GuardSeverity::Warn => "warn".to_string(),
                },
                target: f.target.clone(),
                message: f.message.clone(),
                remediation: f.remediation.clone(),
            })
            .collect();

        let guard_errors = guard_findings
            .iter()
            .filter(|f| f.severity == "error")
            .count();
        let guard_warnings = guard_findings
            .iter()
            .filter(|f| f.severity == "warn")
            .count();

        let success = plan.error_count() == 0 && guard_errors == 0;

        Ok(PlanArchitectureOutput {
            plan,
            guard_findings,
            guard_errors,
            guard_warnings,
            success,
        })
    }
}
