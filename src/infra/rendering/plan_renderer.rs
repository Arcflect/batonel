pub struct PlanRendererAdapter;

impl PlanRendererAdapter {
    pub fn render_text(output: &crate::app::usecase::PlanArchitectureOutput) -> String {
        let mut lines = vec![
            "Archflow Implementation Plan".to_string(),
            "============================".to_string(),
            format!("Project: {}", output.plan.project_name),
            format!("Style:   {}", output.plan.architecture_style),
            format!("Lang:    {}", output.plan.language),
            format!("Modules: {}", output.plan.module_count),
            String::new(),
            "Planned Artifacts:".to_string(),
        ];

        for artifact in &output.plan.artifacts {
            match artifact.status {
                crate::domain::planning::PlannedArtifactStatus::Planned => {
                    lines.push(format!(
                        "  - {} [{}] -> {}",
                        artifact.name,
                        artifact.role,
                        artifact.resolved_path.as_deref().unwrap_or("<unresolved>")
                    ));
                }
                crate::domain::planning::PlannedArtifactStatus::Error => {
                    lines.push(format!(
                        "  ! {} [{}]: Error: {}",
                        artifact.name,
                        artifact.role,
                        artifact.error_message.as_deref().unwrap_or("unknown error")
                    ));
                }
            }
        }

        lines.push(String::new());
        lines.push(format!(
            "Guard result: {} issue(s) (errors={}, warnings={})",
            output.guard_findings.len(),
            output.guard_errors,
            output.guard_warnings
        ));
        lines.push(format!(
            "Plan result: {} planned, {} errors.",
            output.plan.planned_count(),
            output.plan.error_count()
        ));

        if !output.guard_findings.is_empty() {
            lines.push(String::new());
            lines.push("Guard findings:".to_string());
            for finding in &output.guard_findings {
                lines.push(format!(
                    "- [{}][{}] {}",
                    finding.rule_id, finding.severity, finding.message
                ));
                lines.push(format!("  target: {}", finding.target));
                lines.push(format!("  remediation: {}", finding.remediation));
            }
        }

        lines.join("\n")
    }

    pub fn render_markdown(output: &crate::app::usecase::PlanArchitectureOutput) -> String {
        let mut md = String::new();
        md.push_str("# Archflow Implementation Plan\n\n");
        md.push_str(&format!("- Project: {}\n", output.plan.project_name));
        md.push_str(&format!("- Style: {}\n", output.plan.architecture_style));
        md.push_str(&format!("- Language: {}\n", output.plan.language));
        md.push_str(&format!("- Modules: {}\n\n", output.plan.module_count));

        md.push_str("## Planned Artifacts\n\n");
        for artifact in &output.plan.artifacts {
            match artifact.status {
                crate::domain::planning::PlannedArtifactStatus::Planned => md.push_str(&format!(
                    "- {} [{}] -> `{}`\n",
                    artifact.name,
                    artifact.role,
                    artifact.resolved_path.as_deref().unwrap_or("<unresolved>")
                )),
                crate::domain::planning::PlannedArtifactStatus::Error => md.push_str(&format!(
                    "- {} [{}] - Error: {}\n",
                    artifact.name,
                    artifact.role,
                    artifact.error_message.as_deref().unwrap_or("unknown error")
                )),
            }
        }

        md.push_str("\n## Summary\n\n");
        md.push_str(&format!(
            "- Planned: {}\n- Plan errors: {}\n- Guard errors: {}\n- Guard warnings: {}\n",
            output.plan.planned_count(),
            output.plan.error_count(),
            output.guard_errors,
            output.guard_warnings
        ));

        if !output.guard_findings.is_empty() {
            md.push_str("\n## Guard Findings\n\n");
            for finding in &output.guard_findings {
                md.push_str(&format!(
                    "- **{}** ({}) {}\n",
                    finding.rule_id, finding.severity, finding.message
                ));
                md.push_str(&format!("  - target: {}\n", finding.target));
                md.push_str(&format!("  - remediation: {}\n", finding.remediation));
            }
        }

        md
    }

    pub fn render_json(
        output: &crate::app::usecase::PlanArchitectureOutput,
    ) -> Result<String, String> {
        serde_json::to_string_pretty(output).map_err(|e| e.to_string())
    }
}
