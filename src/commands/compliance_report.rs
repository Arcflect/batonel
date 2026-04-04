use crate::commands::audit::{self, AuditFinding, Severity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Json,
    Csv,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSummary {
    pub rule_id: String,
    pub error_count: usize,
    pub warning_count: usize,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryCompliance {
    pub repository: String,
    pub errors: usize,
    pub warnings: usize,
    pub total_findings: usize,
    pub status: String,
    pub rule_summaries: Vec<RuleSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSummary {
    pub repository_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub total_errors: usize,
    pub total_warnings: usize,
    pub total_findings: usize,
    pub rule_summaries: Vec<RuleSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleTrend {
    pub rule_id: String,
    pub error_delta: isize,
    pub warning_delta: isize,
    pub total_delta: isize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityTrend {
    pub error_delta: isize,
    pub warning_delta: isize,
    pub finding_delta: isize,
    pub failed_repo_delta: isize,
    pub passed_repo_delta: isize,
    pub rule_trends: Vec<RuleTrend>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceExport {
    pub generated_at_unix: u64,
    pub repositories: Vec<RepositoryCompliance>,
    pub summary: ComplianceSummary,
    pub trend: Option<SeverityTrend>,
}

pub fn execute_cli(
    repos: &[String],
    repos_file: Option<&str>,
    format: ReportFormat,
    output: &str,
    baseline_json: Option<&str>,
) {
    let repo_list = match resolve_repo_list(repos, repos_file) {
        Ok(list) => list,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    let export = match build_export(&repo_list, baseline_json) {
        Ok(export) => export,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = write_export(output, format, &export) {
        eprintln!("[!] {}", err);
        std::process::exit(1);
    }

    println!("Archflow Compliance Report Export");
    println!("===============================");
    println!("repositories: {}", export.summary.repository_count);
    println!("failed: {}", export.summary.failed_count);
    println!("passed: {}", export.summary.passed_count);
    println!("errors: {}", export.summary.total_errors);
    println!("warnings: {}", export.summary.total_warnings);
    if export.trend.is_some() {
        println!("trend: included");
    }
    println!("output: {}", output);
}

fn resolve_repo_list(repos: &[String], repos_file: Option<&str>) -> Result<Vec<String>, String> {
    let mut entries: Vec<String> = repos.to_vec();

    if let Some(path) = repos_file {
        let contents = fs::read_to_string(path)
            .map_err(|err| format!("failed to read repos file '{}': {}", path, err))?;
        for line in contents.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            entries.push(trimmed.to_string());
        }
    }

    let mut dedup = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for entry in entries {
        if seen.insert(entry.clone()) {
            dedup.push(entry);
        }
    }

    if dedup.is_empty() {
        return Err(
            "no repositories provided; use --repos and/or --repos-file to specify targets"
                .to_string(),
        );
    }

    Ok(dedup)
}

fn build_export(repositories: &[String], baseline_json: Option<&str>) -> Result<ComplianceExport, String> {
    let mut repo_reports = Vec::new();

    for repo in repositories {
        let path = Path::new(repo);
        if !path.is_dir() {
            return Err(format!("repository path does not exist: {}", repo));
        }

        let report = audit::run_for_root(path)
            .map_err(|err| format!("audit failed for '{}': {}", repo, err))?;

        let repo_rule_summary = summarize_rules(&report.findings);

        repo_reports.push(RepositoryCompliance {
            repository: repo.clone(),
            errors: report.errors,
            warnings: report.warnings,
            total_findings: report.findings.len(),
            status: if report.errors > 0 {
                "failed".to_string()
            } else if report.warnings > 0 {
                "passed_with_warnings".to_string()
            } else {
                "passed".to_string()
            },
            rule_summaries: repo_rule_summary,
        });
    }

    let summary = summarize_compliance(&repo_reports);

    let trend = if let Some(path) = baseline_json {
        let contents = fs::read_to_string(path)
            .map_err(|err| format!("failed to read baseline report '{}': {}", path, err))?;
        let baseline: ComplianceExport = serde_json::from_str(&contents)
            .map_err(|err| format!("failed to parse baseline report '{}': {}", path, err))?;
        Some(compute_trend(&baseline.summary, &summary))
    } else {
        None
    };

    Ok(ComplianceExport {
        generated_at_unix: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0),
        repositories: repo_reports,
        summary,
        trend,
    })
}

fn summarize_rules(findings: &[AuditFinding]) -> Vec<RuleSummary> {
    let mut map: HashMap<String, (usize, usize)> = HashMap::new();

    for finding in findings {
        let entry = map.entry(finding.rule_id.clone()).or_insert((0, 0));
        match finding.severity {
            Severity::Error => entry.0 += 1,
            Severity::Warn => entry.1 += 1,
        }
    }

    let mut rows: Vec<RuleSummary> = map
        .into_iter()
        .map(|(rule_id, (error_count, warning_count))| RuleSummary {
            rule_id,
            error_count,
            warning_count,
            total_count: error_count + warning_count,
        })
        .collect();

    rows.sort_by(|a, b| b.total_count.cmp(&a.total_count).then_with(|| a.rule_id.cmp(&b.rule_id)));
    rows
}

fn summarize_compliance(repositories: &[RepositoryCompliance]) -> ComplianceSummary {
    let repository_count = repositories.len();
    let failed_count = repositories.iter().filter(|repo| repo.errors > 0).count();
    let passed_count = repository_count.saturating_sub(failed_count);
    let total_errors = repositories.iter().map(|repo| repo.errors).sum();
    let total_warnings = repositories.iter().map(|repo| repo.warnings).sum();
    let total_findings = repositories.iter().map(|repo| repo.total_findings).sum();

    let mut aggregate: HashMap<String, (usize, usize)> = HashMap::new();
    for repo in repositories {
        for rule in &repo.rule_summaries {
            let entry = aggregate.entry(rule.rule_id.clone()).or_insert((0, 0));
            entry.0 += rule.error_count;
            entry.1 += rule.warning_count;
        }
    }

    let mut rule_summaries: Vec<RuleSummary> = aggregate
        .into_iter()
        .map(|(rule_id, (error_count, warning_count))| RuleSummary {
            rule_id,
            error_count,
            warning_count,
            total_count: error_count + warning_count,
        })
        .collect();

    rule_summaries.sort_by(|a, b| b.total_count.cmp(&a.total_count).then_with(|| a.rule_id.cmp(&b.rule_id)));

    ComplianceSummary {
        repository_count,
        passed_count,
        failed_count,
        total_errors,
        total_warnings,
        total_findings,
        rule_summaries,
    }
}

fn compute_trend(previous: &ComplianceSummary, current: &ComplianceSummary) -> SeverityTrend {
    let mut prev_map: HashMap<String, (usize, usize)> = HashMap::new();
    for rule in &previous.rule_summaries {
        prev_map.insert(rule.rule_id.clone(), (rule.error_count, rule.warning_count));
    }

    let mut curr_map: HashMap<String, (usize, usize)> = HashMap::new();
    for rule in &current.rule_summaries {
        curr_map.insert(rule.rule_id.clone(), (rule.error_count, rule.warning_count));
    }

    let mut all_rules: Vec<String> = prev_map
        .keys()
        .chain(curr_map.keys())
        .cloned()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    all_rules.sort();

    let mut rule_trends = Vec::new();
    for rule_id in all_rules {
        let (prev_err, prev_warn) = prev_map.get(&rule_id).copied().unwrap_or((0, 0));
        let (curr_err, curr_warn) = curr_map.get(&rule_id).copied().unwrap_or((0, 0));
        let error_delta = curr_err as isize - prev_err as isize;
        let warning_delta = curr_warn as isize - prev_warn as isize;
        rule_trends.push(RuleTrend {
            rule_id,
            error_delta,
            warning_delta,
            total_delta: error_delta + warning_delta,
        });
    }

    rule_trends.sort_by(|a, b| b.total_delta.abs().cmp(&a.total_delta.abs()).then_with(|| a.rule_id.cmp(&b.rule_id)));

    SeverityTrend {
        error_delta: current.total_errors as isize - previous.total_errors as isize,
        warning_delta: current.total_warnings as isize - previous.total_warnings as isize,
        finding_delta: current.total_findings as isize - previous.total_findings as isize,
        failed_repo_delta: current.failed_count as isize - previous.failed_count as isize,
        passed_repo_delta: current.passed_count as isize - previous.passed_count as isize,
        rule_trends,
    }
}

fn write_export(path: &str, format: ReportFormat, export: &ComplianceExport) -> Result<(), String> {
    match format {
        ReportFormat::Json => {
            let serialized = serde_json::to_string_pretty(export)
                .map_err(|err| format!("failed to serialize JSON report: {}", err))?;
            fs::write(path, serialized)
                .map_err(|err| format!("failed to write JSON report '{}': {}", path, err))
        }
        ReportFormat::Csv => write_csv(path, export),
    }
}

fn write_csv(path: &str, export: &ComplianceExport) -> Result<(), String> {
    let mut lines = Vec::new();
    lines.push(
        "section,repository,status,errors,warnings,total_findings,rule_id,error_count,warning_count,total_count,error_delta,warning_delta,total_delta".to_string(),
    );

    lines.push(csv_row(&[
        "summary".to_string(),
        "all".to_string(),
        "summary".to_string(),
        export.summary.total_errors.to_string(),
        export.summary.total_warnings.to_string(),
        export.summary.total_findings.to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    ]));

    for repo in &export.repositories {
        lines.push(csv_row(&[
            "repo".to_string(),
            repo.repository.clone(),
            repo.status.clone(),
            repo.errors.to_string(),
            repo.warnings.to_string(),
            repo.total_findings.to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ]));
    }

    for rule in &export.summary.rule_summaries {
        lines.push(csv_row(&[
            "rule".to_string(),
            "all".to_string(),
            "rule".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            rule.rule_id.clone(),
            rule.error_count.to_string(),
            rule.warning_count.to_string(),
            rule.total_count.to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ]));
    }

    if let Some(trend) = &export.trend {
        lines.push(csv_row(&[
            "trend".to_string(),
            "all".to_string(),
            "trend".to_string(),
            trend.error_delta.to_string(),
            trend.warning_delta.to_string(),
            trend.finding_delta.to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            trend.error_delta.to_string(),
            trend.warning_delta.to_string(),
            trend.finding_delta.to_string(),
        ]));

        for rule in &trend.rule_trends {
            lines.push(csv_row(&[
                "trend_rule".to_string(),
                "all".to_string(),
                "trend_rule".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                rule.rule_id.clone(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                rule.error_delta.to_string(),
                rule.warning_delta.to_string(),
                rule.total_delta.to_string(),
            ]));
        }
    }

    fs::write(path, lines.join("\n"))
        .map_err(|err| format!("failed to write CSV report '{}': {}", path, err))
}

fn csv_row(fields: &[String]) -> String {
    fields
        .iter()
        .map(|field| escape_csv(field))
        .collect::<Vec<_>>()
        .join(",")
}

fn escape_csv(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo(name: &str, errors: usize, warnings: usize, rules: Vec<RuleSummary>) -> RepositoryCompliance {
        RepositoryCompliance {
            repository: name.to_string(),
            errors,
            warnings,
            total_findings: errors + warnings,
            status: if errors > 0 {
                "failed".to_string()
            } else {
                "passed".to_string()
            },
            rule_summaries: rules,
        }
    }

    #[test]
    fn summarize_compliance_aggregates_counts_and_rules() {
        let repos = vec![
            repo(
                "repo-a",
                1,
                1,
                vec![RuleSummary {
                    rule_id: "required-root-file".to_string(),
                    error_count: 1,
                    warning_count: 0,
                    total_count: 1,
                }],
            ),
            repo(
                "repo-b",
                0,
                2,
                vec![RuleSummary {
                    rule_id: "artifact-path-aligns-role".to_string(),
                    error_count: 0,
                    warning_count: 2,
                    total_count: 2,
                }],
            ),
        ];

        let summary = summarize_compliance(&repos);
        assert_eq!(summary.repository_count, 2);
        assert_eq!(summary.failed_count, 1);
        assert_eq!(summary.passed_count, 1);
        assert_eq!(summary.total_errors, 1);
        assert_eq!(summary.total_warnings, 3);
        assert_eq!(summary.total_findings, 4);
        assert_eq!(summary.rule_summaries.len(), 2);
    }

    #[test]
    fn compute_trend_reports_deltas() {
        let previous = ComplianceSummary {
            repository_count: 2,
            passed_count: 1,
            failed_count: 1,
            total_errors: 4,
            total_warnings: 2,
            total_findings: 6,
            rule_summaries: vec![RuleSummary {
                rule_id: "required-root-file".to_string(),
                error_count: 4,
                warning_count: 0,
                total_count: 4,
            }],
        };

        let current = ComplianceSummary {
            repository_count: 2,
            passed_count: 2,
            failed_count: 0,
            total_errors: 1,
            total_warnings: 3,
            total_findings: 4,
            rule_summaries: vec![
                RuleSummary {
                    rule_id: "required-root-file".to_string(),
                    error_count: 1,
                    warning_count: 0,
                    total_count: 1,
                },
                RuleSummary {
                    rule_id: "artifact-path-aligns-role".to_string(),
                    error_count: 0,
                    warning_count: 3,
                    total_count: 3,
                },
            ],
        };

        let trend = compute_trend(&previous, &current);
        assert_eq!(trend.error_delta, -3);
        assert_eq!(trend.warning_delta, 1);
        assert_eq!(trend.finding_delta, -2);
        assert_eq!(trend.failed_repo_delta, -1);
        assert_eq!(trend.passed_repo_delta, 1);
        assert!(!trend.rule_trends.is_empty());
    }
}
