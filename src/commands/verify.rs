use std::collections::BTreeMap;

use crate::model::verify::{CheckResult, VerifyReport, VerifyStatus, VerifyTarget};

/// Renders a report to stdout.
pub fn render_report(report: &VerifyReport) {
    for line in build_report_lines(report) {
        println!("{}", line);
    }
}

pub fn build_report_lines(report: &VerifyReport) -> Vec<String> {
    let summary = report.summary();
    let mut lines = vec![
        "Verification Report".to_string(),
        "===================".to_string(),
        format!("Status: {}", overall_status_label(report, &summary)),
        format!(
            "Checks: total={} pass={} fail={} warn={} skip={}",
            summary.total, summary.passes, summary.failures, summary.warnings, summary.skips
        ),
    ];

    let failures: Vec<&CheckResult> = report
        .results
        .iter()
        .filter(|r| r.status == VerifyStatus::Fail)
        .collect();
    let warnings: Vec<&CheckResult> = report
        .results
        .iter()
        .filter(|r| r.status == VerifyStatus::Warn)
        .collect();

    if !failures.is_empty() {
        lines.push(String::new());
        lines.push("Failures".to_string());
        lines.push("--------".to_string());
        lines.extend(render_grouped_results(&failures));
    }

    if !warnings.is_empty() {
        lines.push(String::new());
        lines.push("Warnings".to_string());
        lines.push("--------".to_string());
        lines.extend(render_grouped_results(&warnings));
    }

    if failures.is_empty() && warnings.is_empty() {
        lines.push(String::new());
        lines.push("No failures or warnings detected.".to_string());
    }

    if summary.skips > 0 {
        lines.push(String::new());
        lines.push(format!(
            "Skipped checks: {} (hidden from detailed output)",
            summary.skips
        ));
    }

    lines
}

fn overall_status_label(report: &VerifyReport, summary: &crate::model::verify::VerifySummary) -> &'static str {
    if !report.is_success() {
        "FAILED"
    } else if summary.warnings > 0 {
        "PASSED WITH WARNINGS"
    } else {
        "PASSED"
    }
}

fn render_grouped_results(results: &[&CheckResult]) -> Vec<String> {
    let mut grouped: BTreeMap<String, Vec<&CheckResult>> = BTreeMap::new();
    for result in results {
        grouped
            .entry(format_target(&result.target))
            .or_default()
            .push(*result);
    }

    let mut lines = Vec::new();
    for (target, target_results) in grouped {
        lines.push(format!("{}", target));
        for result in target_results {
            lines.push(format!("  - [{}] {}", result.check_id, result.message));
        }
    }
    lines
}

fn format_target(target: &VerifyTarget) -> String {
    match target {
        VerifyTarget::RootConfig { name, .. } => format!("Root config: {}", name),
        VerifyTarget::Artifact { name } => format!("Artifact: {}", name),
        VerifyTarget::Contract { artifact_name, path } => {
            format!("Contract: {} ({})", artifact_name, path)
        }
        VerifyTarget::Prompt { artifact_name, path } => {
            format!("Prompt: {} ({})", artifact_name, path)
        }
        VerifyTarget::SourceFile { artifact_name, path } => {
            format!("Source file: {} ({})", artifact_name, path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::build_report_lines;
    use crate::model::verify::{CheckResult, VerifyReport, VerifyStatus, VerifyTarget};

    #[test]
    fn report_lines_group_failures_and_warnings() {
        let report = VerifyReport::new(vec![
            CheckResult {
                check_id: "root-file-exists".to_string(),
                target: VerifyTarget::RootConfig {
                    name: "project.baton.yaml".to_string(),
                    path: "project.baton.yaml".to_string(),
                },
                status: VerifyStatus::Fail,
                message: "Missing required file: project.baton.yaml".to_string(),
            },
            CheckResult {
                check_id: "prompt-exists".to_string(),
                target: VerifyTarget::Prompt {
                    artifact_name: "user".to_string(),
                    path: "src/user.prompt.md".to_string(),
                },
                status: VerifyStatus::Warn,
                message: "Prompt missing for user".to_string(),
            },
        ]);

        let lines = build_report_lines(&report);
        let output = lines.join("\n");

        assert!(output.contains("Status: FAILED"));
        assert!(output.contains("Failures"));
        assert!(output.contains("Warnings"));
        assert!(output.contains("Root config: project.baton.yaml"));
        assert!(output.contains("[root-file-exists] Missing required file: project.baton.yaml"));
        assert!(output.contains("Prompt: user (src/user.prompt.md)"));
        assert!(output.contains("[prompt-exists] Prompt missing for user"));
    }

    #[test]
    fn report_lines_show_clean_success_summary() {
        let report = VerifyReport::new(vec![CheckResult {
            check_id: "role-defined".to_string(),
            target: VerifyTarget::Artifact { name: "user".to_string() },
            status: VerifyStatus::Pass,
            message: "Role is valid".to_string(),
        }]);

        let lines = build_report_lines(&report);
        let output = lines.join("\n");

        assert!(output.contains("Status: PASSED"));
        assert!(output.contains("No failures or warnings detected."));
        assert!(!output.contains("Failures"));
        assert!(!output.contains("Warnings"));
    }
}
