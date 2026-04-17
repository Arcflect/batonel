use crate::commands::audit::{run_for_root, AuditFinding, Severity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Priority tier for a violation, ordered from most to least urgent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Critical = 0,
    High = 1,
    Medium = 2,
    Low = 3,
}

impl Priority {
    pub fn as_str(self) -> &'static str {
        match self {
            Priority::Critical => "critical",
            Priority::High => "high",
            Priority::Medium => "medium",
            Priority::Low => "low",
        }
    }
}

/// A single violation decorated with its triage priority.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriagedFinding {
    pub priority: Priority,
    pub rule_id: String,
    pub severity: Severity,
    pub target: String,
    pub message: String,
    pub remediation: String,
}

/// A group of violations sharing the same (priority, rule_id) pair.
/// Remediations within a group are identical in kind, so a single representative
/// remediation hint is shown together with all affected targets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationGroup {
    pub priority: Priority,
    pub rule_id: String,
    pub affected_count: usize,
    pub representative_remediation: String,
    pub targets: Vec<String>,
}

/// The full prioritized triage output for a repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriagePlan {
    pub repository: String,
    pub total_findings: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    /// Remediation groups ordered by priority (critical first), then by
    /// descending affected count (widest blast-radius first within a tier).
    pub remediation_groups: Vec<RemediationGroup>,
}

/// Assign a triage priority to a single audit finding.
///
/// The mapping is intentional:
/// - Config/schema parse failures block all downstream checks → Critical.
/// - Structural integrity violations (undefined module/role, missing forbidden
///   dependency entries) require schema changes → High.
/// - Naming convention violations are correctable by rename alone → Medium.
/// - Path deviations are non-blocking warnings → Low.
pub fn assign_priority(finding: &AuditFinding) -> Priority {
    match (finding.severity, finding.rule_id.as_str()) {
        (Severity::Error, "required-root-file")
        | (Severity::Error, "project-config-valid")
        | (Severity::Error, "placement-config-valid")
        | (Severity::Error, "artifacts-plan-valid")
        | (Severity::Error, "policy-profile-valid") => Priority::Critical,

        (Severity::Error, "artifact-module-defined")
        | (Severity::Error, "artifact-role-defined")
        | (Severity::Error, "policy-forbidden-dependencies-covered") => Priority::High,

        (Severity::Error, "module-name-policy") | (Severity::Error, "artifact-name-policy") => {
            Priority::Medium
        }

        (Severity::Warn, "artifact-path-aligns-role") => Priority::Low,

        // Default bucketing: any unclassified error is High, any warn is Low.
        (Severity::Error, _) => Priority::High,
        (Severity::Warn, _) => Priority::Low,
    }
}

/// Build a [`TriagePlan`] from a slice of raw audit findings.
pub fn build_triage_plan(repository: String, findings: &[AuditFinding]) -> TriagePlan {
    let mut triaged: Vec<TriagedFinding> = findings
        .iter()
        .map(|f| TriagedFinding {
            priority: assign_priority(f),
            rule_id: f.rule_id.clone(),
            severity: f.severity,
            target: f.target.clone(),
            message: f.message.clone(),
            remediation: f.remediation.clone(),
        })
        .collect();

    triaged.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then_with(|| a.rule_id.cmp(&b.rule_id))
            .then_with(|| a.target.cmp(&b.target))
    });

    // Group by (priority, rule_id).
    let mut group_map: HashMap<(u8, String), Vec<&TriagedFinding>> = HashMap::new();
    for f in &triaged {
        let key = (f.priority as u8, f.rule_id.clone());
        group_map.entry(key).or_default().push(f);
    }

    let mut remediation_groups: Vec<RemediationGroup> = group_map
        .into_iter()
        .map(|((_, rule_id), members)| {
            let priority = members[0].priority;
            let mut targets: Vec<String> = members.iter().map(|f| f.target.clone()).collect();
            targets.sort();
            let representative_remediation = members[0].remediation.clone();
            RemediationGroup {
                priority,
                rule_id,
                affected_count: members.len(),
                representative_remediation,
                targets,
            }
        })
        .collect();

    // Sort: priority ASC; within same priority, more affected targets first
    // (widest blast-radius should be addressed before narrow ones); then rule_id
    // alphabetically for determinism.
    remediation_groups.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then_with(|| b.affected_count.cmp(&a.affected_count))
            .then_with(|| a.rule_id.cmp(&b.rule_id))
    });

    let critical_count = triaged
        .iter()
        .filter(|f| f.priority == Priority::Critical)
        .count();
    let high_count = triaged
        .iter()
        .filter(|f| f.priority == Priority::High)
        .count();
    let medium_count = triaged
        .iter()
        .filter(|f| f.priority == Priority::Medium)
        .count();
    let low_count = triaged
        .iter()
        .filter(|f| f.priority == Priority::Low)
        .count();

    TriagePlan {
        repository,
        total_findings: findings.len(),
        critical_count,
        high_count,
        medium_count,
        low_count,
        remediation_groups,
    }
}

pub fn execute(top: Option<usize>, json: bool) {
    let report = match run_for_root(Path::new(".")) {
        Ok(report) => report,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    let plan = build_triage_plan(report.repository.clone(), &report.findings);

    if json {
        match serde_json::to_string_pretty(&plan) {
            Ok(s) => println!("{}", s),
            Err(err) => {
                eprintln!("[!] failed to serialize triage plan: {}", err);
                std::process::exit(1);
            }
        }
        return;
    }

    render_triage_plan(&plan, top);

    if plan.critical_count > 0 || plan.high_count > 0 {
        std::process::exit(1);
    }
}

fn render_triage_plan(plan: &TriagePlan, top: Option<usize>) {
    println!("Batonel Violation Triage");
    println!("=========================");
    println!("Repository : {}", plan.repository);
    println!(
        "Findings   : {} total  (critical={}, high={}, medium={}, low={})",
        plan.total_findings,
        plan.critical_count,
        plan.high_count,
        plan.medium_count,
        plan.low_count,
    );
    println!();

    if plan.remediation_groups.is_empty() {
        println!("Status: PASSED – no violations found.");
        return;
    }

    let groups = match top {
        Some(n) => &plan.remediation_groups[..n.min(plan.remediation_groups.len())],
        None => &plan.remediation_groups,
    };

    println!("Prioritized Remediation Plan:");
    println!("-----------------------------");
    for (i, group) in groups.iter().enumerate() {
        let affected_label = if group.affected_count == 1 {
            "1 target".to_string()
        } else {
            format!("{} targets", group.affected_count)
        };
        println!(
            "{}. [{}][{}] {} ({})",
            i + 1,
            group.priority.as_str().to_uppercase(),
            group.rule_id,
            group.representative_remediation,
            affected_label,
        );
        for target in &group.targets {
            println!("     - {}", target);
        }
    }

    if let Some(n) = top {
        if n < plan.remediation_groups.len() {
            println!(
                "\n(showing top {} of {} remediation group(s); omit --top to see all)",
                n,
                plan.remediation_groups.len()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::audit::{AuditFinding, Severity};

    fn make_finding(rule_id: &str, severity: Severity, target: &str) -> AuditFinding {
        AuditFinding {
            rule_id: rule_id.to_string(),
            severity,
            target: target.to_string(),
            message: format!("test message for {}", rule_id),
            remediation: format!("fix {}", rule_id),
        }
    }

    #[test]
    fn critical_rules_get_critical_priority() {
        for rule in &[
            "required-root-file",
            "project-config-valid",
            "placement-config-valid",
            "artifacts-plan-valid",
            "policy-profile-valid",
        ] {
            let f = make_finding(rule, Severity::Error, "target");
            assert_eq!(
                assign_priority(&f),
                Priority::Critical,
                "rule {} should be Critical",
                rule
            );
        }
    }

    #[test]
    fn structural_errors_get_high_priority() {
        for rule in &[
            "artifact-module-defined",
            "artifact-role-defined",
            "policy-forbidden-dependencies-covered",
        ] {
            let f = make_finding(rule, Severity::Error, "artifact:foo");
            assert_eq!(
                assign_priority(&f),
                Priority::High,
                "rule {} should be High",
                rule
            );
        }
    }

    #[test]
    fn naming_errors_get_medium_priority() {
        let f = make_finding("module-name-policy", Severity::Error, "module:BadModule");
        assert_eq!(assign_priority(&f), Priority::Medium);

        let f = make_finding("artifact-name-policy", Severity::Error, "artifact:BadArtifact");
        assert_eq!(assign_priority(&f), Priority::Medium);
    }

    #[test]
    fn path_warnings_get_low_priority() {
        let f = make_finding("artifact-path-aligns-role", Severity::Warn, "artifact:foo");
        assert_eq!(assign_priority(&f), Priority::Low);
    }

    #[test]
    fn unknown_error_defaults_to_high() {
        let f = make_finding("some-new-rule", Severity::Error, "target");
        assert_eq!(assign_priority(&f), Priority::High);
    }

    #[test]
    fn unknown_warn_defaults_to_low() {
        let f = make_finding("some-warn-rule", Severity::Warn, "target");
        assert_eq!(assign_priority(&f), Priority::Low);
    }

    #[test]
    fn empty_findings_produces_empty_plan() {
        let plan = build_triage_plan(".".to_string(), &[]);
        assert_eq!(plan.total_findings, 0);
        assert_eq!(plan.critical_count, 0);
        assert!(plan.remediation_groups.is_empty());
    }

    #[test]
    fn triage_plan_groups_and_sorts_by_priority() {
        let findings = vec![
            make_finding("artifact-name-policy", Severity::Error, "artifact:a"),
            make_finding("artifact-name-policy", Severity::Error, "artifact:b"),
            make_finding("required-root-file", Severity::Error, "project.baton.yaml"),
            make_finding("artifact-path-aligns-role", Severity::Warn, "artifact:c"),
        ];

        let plan = build_triage_plan(".".to_string(), &findings);
        assert_eq!(plan.total_findings, 4);
        assert_eq!(plan.critical_count, 1);
        assert_eq!(plan.high_count, 0);
        assert_eq!(plan.medium_count, 2);
        assert_eq!(plan.low_count, 1);

        // Critical group must be first.
        assert_eq!(plan.remediation_groups[0].priority, Priority::Critical);
        assert_eq!(plan.remediation_groups[0].rule_id, "required-root-file");

        // Medium group next, with 2 affected targets.
        assert_eq!(plan.remediation_groups[1].priority, Priority::Medium);
        assert_eq!(plan.remediation_groups[1].affected_count, 2);

        // Low group last.
        assert_eq!(plan.remediation_groups[2].priority, Priority::Low);
    }

    #[test]
    fn within_same_priority_wider_blast_radius_first() {
        let findings = vec![
            make_finding("artifact-module-defined", Severity::Error, "artifact:a"),
            make_finding("artifact-module-defined", Severity::Error, "artifact:b"),
            make_finding("artifact-module-defined", Severity::Error, "artifact:c"),
            make_finding("artifact-role-defined", Severity::Error, "artifact:x"),
        ];

        let plan = build_triage_plan(".".to_string(), &findings);
        assert_eq!(plan.remediation_groups.len(), 2);
        // artifact-module-defined has 3 affected vs 1 → should come first
        assert_eq!(plan.remediation_groups[0].rule_id, "artifact-module-defined");
        assert_eq!(plan.remediation_groups[0].affected_count, 3);
        assert_eq!(plan.remediation_groups[1].rule_id, "artifact-role-defined");
    }

    #[test]
    fn targets_within_group_are_sorted() {
        let findings = vec![
            make_finding("artifact-name-policy", Severity::Error, "artifact:z"),
            make_finding("artifact-name-policy", Severity::Error, "artifact:a"),
            make_finding("artifact-name-policy", Severity::Error, "artifact:m"),
        ];

        let plan = build_triage_plan(".".to_string(), &findings);
        let targets = &plan.remediation_groups[0].targets;
        assert_eq!(targets[0], "artifact:a");
        assert_eq!(targets[1], "artifact:m");
        assert_eq!(targets[2], "artifact:z");
    }
}
