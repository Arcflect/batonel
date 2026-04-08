use crate::commands::fix::{collect_findings, FixClass, FixFinding};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

// ── data model ─────────────────────────────────────────────────────────────

/// Approval decision on a single fix plan item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    /// No decision has been recorded yet. ReviewRequired items block `apply`.
    Pending,
    /// The fix has been explicitly approved and may be applied.
    Approved,
    /// The fix has been explicitly rejected and will be skipped even if applied.
    Rejected,
}

impl ApprovalStatus {
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        match self {
            ApprovalStatus::Pending => "pending",
            ApprovalStatus::Approved => "approved",
            ApprovalStatus::Rejected => "rejected",
        }
    }
}

/// One targeted fix together with its approval state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixPlanItem {
    /// Stable identifier within the plan (fix-001, fix-002, …).
    pub id: String,
    pub rule_id: String,
    pub fix_class: FixClass,
    pub target: String,
    pub message: String,
    pub remediation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_preview: Option<String>,
    /// `true` when a human must explicitly approve before `apply` proceeds.
    pub approval_required: bool,
    pub approval_status: ApprovalStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<String>,
    /// Unix timestamp (seconds) of the approval decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_at_unix: Option<u64>,
}

/// Top-level plan file written by `fix-rollout plan` and consumed by
/// `fix-rollout approve` / `fix-rollout apply`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixRolloutPlan {
    /// Monotonic Unix timestamp used as the plan's unique identifier.
    pub plan_id: u64,
    pub created_at_unix: u64,
    pub repository: String,
    pub total_items: usize,
    pub auto_fixable_count: usize,
    pub review_required_count: usize,
    pub items: Vec<FixPlanItem>,
    pub applied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_at_unix: Option<u64>,
}

// ── helpers ─────────────────────────────────────────────────────────────────

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn plan_item_from_finding(seq: usize, f: &FixFinding) -> FixPlanItem {
    let approval_required = f.class == FixClass::ReviewRequired;
    let approval_status = if approval_required {
        ApprovalStatus::Pending
    } else {
        // AutoFixable items are pre-approved; no human gate required.
        ApprovalStatus::Approved
    };
    FixPlanItem {
        id: format!("fix-{:03}", seq),
        rule_id: f.rule_id.to_string(),
        fix_class: f.class,
        target: f.target.clone(),
        message: f.message.clone(),
        remediation: f.remediation.clone(),
        patch_preview: f.patch_preview.clone(),
        approval_required,
        approval_status,
        approved_by: None,
        approved_at_unix: None,
    }
}

fn load_plan(path: &str) -> Result<FixRolloutPlan, String> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read plan file '{}': {}", path, e))?;
    serde_json::from_str(&raw)
        .map_err(|e| format!("cannot parse plan file '{}': {}", path, e))
}

fn save_plan(path: &str, plan: &FixRolloutPlan) -> Result<(), String> {
    let json = serde_json::to_string_pretty(plan)
        .map_err(|e| format!("cannot serialize plan: {}", e))?;
    std::fs::write(path, json).map_err(|e| format!("cannot write plan file '{}': {}", path, e))
}

// ── commands ─────────────────────────────────────────────────────────────────

/// Generate a new fix plan JSON file from the current project state.
pub fn execute_plan(root: &Path, output: &str) {
    let findings = collect_findings(root);

    if findings.is_empty() {
        println!("No fix candidates found. Plan not written.");
        return;
    }

    let now = now_unix();
    let items: Vec<FixPlanItem> = findings
        .iter()
        .enumerate()
        .map(|(i, f)| plan_item_from_finding(i + 1, f))
        .collect();

    let auto_fixable_count = items
        .iter()
        .filter(|item| item.fix_class == FixClass::AutoFixable)
        .count();
    let review_required_count = items
        .iter()
        .filter(|item| item.fix_class == FixClass::ReviewRequired)
        .count();

    let plan = FixRolloutPlan {
        plan_id: now,
        created_at_unix: now,
        repository: root.display().to_string(),
        total_items: items.len(),
        auto_fixable_count,
        review_required_count,
        items,
        applied: false,
        applied_at_unix: None,
    };

    match save_plan(output, &plan) {
        Ok(()) => {
            println!("Fix plan written to '{}'", output);
            println!(
                "  {} total item(s): {} auto-fixable, {} review-required",
                plan.total_items, plan.auto_fixable_count, plan.review_required_count,
            );
            if review_required_count > 0 {
                println!(
                    "  Run `archflow fix-rollout approve {}` to approve review-required items before applying.",
                    output,
                );
            }
        }
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    }
}

/// Stamp approval decisions onto plan items.
///
/// - `ids`: approve only items with these IDs; empty list + `all = false` is an error.
/// - `all`: approve all pending items.
/// - `approver`: optional human-readable identifier recorded in the plan.
/// - `reject`: record a rejection instead of approval.
pub fn execute_approve(plan_file: &str, ids: &[String], all: bool, approver: Option<&str>, reject: bool) {
    if ids.is_empty() && !all {
        eprintln!("[!] Specify --all to approve all pending items, or pass item IDs (e.g. fix-001).");
        std::process::exit(1);
    }

    let mut plan = match load_plan(plan_file) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    if plan.applied {
        eprintln!("[!] Plan '{}' has already been applied. Create a new plan.", plan_file);
        std::process::exit(1);
    }

    let now = now_unix();
    let new_status = if reject {
        ApprovalStatus::Rejected
    } else {
        ApprovalStatus::Approved
    };

    let mut changed = 0usize;
    for item in &mut plan.items {
        if !item.approval_required {
            continue; // AutoFixable items are already pre-approved.
        }
        let matches = all || ids.iter().any(|id| id == &item.id);
        if matches {
            // Skip items that already have a terminal decision (unless overriding).
            item.approval_status = new_status;
            item.approved_by = approver.map(str::to_string);
            item.approved_at_unix = Some(now);
            changed += 1;
        }
    }

    if changed == 0 {
        eprintln!("[!] No matching items updated. Check the IDs and try again.");
        std::process::exit(1);
    }

    match save_plan(plan_file, &plan) {
        Ok(()) => {
            let verb = if reject { "rejected" } else { "approved" };
            println!(
                "{} item(s) {} in '{}'",
                changed, verb, plan_file,
            );
            if let Some(name) = approver {
                println!("  Approver: {}", name);
            }
        }
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    }
}

/// Apply all approvable fixes from a plan file.
///
/// - AutoFixable + Approved items → applied automatically via `archflow init`.
/// - ReviewRequired + Approved items → patch guidance printed; operator applies manually.
/// - ReviewRequired + Pending/Rejected items → skipped (or fatal in strict mode).
///
/// The plan file is updated in-place to record the applied timestamp.
pub fn execute_apply(plan_file: &str, strict: bool) {
    let mut plan = match load_plan(plan_file) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    if plan.applied {
        eprintln!("[!] Plan '{}' has already been applied.", plan_file);
        std::process::exit(1);
    }

    // Gate: check for unapproved review-required items.
    let pending_items: Vec<&FixPlanItem> = plan
        .items
        .iter()
        .filter(|item| {
            item.approval_required && item.approval_status == ApprovalStatus::Pending
        })
        .collect();

    let rejected_items: Vec<&FixPlanItem> = plan
        .items
        .iter()
        .filter(|item| {
            item.approval_required && item.approval_status == ApprovalStatus::Rejected
        })
        .collect();

    if !pending_items.is_empty() {
        eprintln!(
            "[!] {} review-required item(s) are still pending approval. Run `archflow fix-rollout approve` first.",
            pending_items.len()
        );
        for item in &pending_items {
            eprintln!("    {} [{}] {}", item.id, item.rule_id, item.target);
        }
        std::process::exit(1);
    }

    let has_auto_fixable = plan
        .items
        .iter()
        .any(|item| item.fix_class == FixClass::AutoFixable && item.approval_status == ApprovalStatus::Approved);

    let approved_review_required: Vec<&FixPlanItem> = plan
        .items
        .iter()
        .filter(|item| {
            item.fix_class == FixClass::ReviewRequired
                && item.approval_status == ApprovalStatus::Approved
        })
        .collect();

    println!("Archflow Fix Rollout – Apply");
    println!("============================");
    println!("Plan  : {}", plan_file);
    println!(
        "Items : {} total  ({} auto-fixable, {} approved review-required, {} rejected)",
        plan.total_items,
        plan.auto_fixable_count,
        approved_review_required.len(),
        rejected_items.len(),
    );
    println!();

    // Apply AutoFixable items.
    if has_auto_fixable {
        println!("Applying auto-fixable items via `archflow init`…");
        crate::commands::init::execute(None, None, false);
        println!("  Auto-fixable items applied.");
        println!();
    }

    // Print guidance for approved ReviewRequired items.
    if !approved_review_required.is_empty() {
        println!("Approved review-required items (apply patches manually):");
        println!("---------------------------------------------------------");
        for item in &approved_review_required {
            let approver_label = item
                .approved_by
                .as_deref()
                .map(|a| format!("approved by {}", a))
                .unwrap_or_else(|| "approved".to_string());
            println!(
                "[{}] {} ({}) — {}",
                item.id, item.rule_id, approver_label, item.target
            );
            println!("  remediation: {}", item.remediation);
            if let Some(patch) = &item.patch_preview {
                println!("  patch preview:");
                for line in patch.lines() {
                    println!("    {}", line);
                }
            }
            println!();
        }
    }

    // Report rejected items if any.
    if !rejected_items.is_empty() {
        println!(
            "Skipped {} rejected item(s):",
            rejected_items.len()
        );
        for item in &rejected_items {
            println!("  {} [{}] {} — skipped (rejected)", item.id, item.rule_id, item.target);
        }
        println!();
        if strict {
            eprintln!(
                "[!] --strict: {} rejected item(s) present. Address them or remove from plan.",
                rejected_items.len()
            );
            std::process::exit(1);
        }
    }

    // Mark the plan as applied.
    plan.applied = true;
    plan.applied_at_unix = Some(now_unix());
    if let Err(err) = save_plan(plan_file, &plan) {
        eprintln!("[!] Plan applied but failed to update plan file: {}", err);
        std::process::exit(1);
    }

    println!(
        "Rollout complete. Plan recorded as applied in '{}'.",
        plan_file,
    );
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::fix::FixFinding;

    fn make_finding(rule_id: &'static str, class: FixClass, target: &str) -> FixFinding {
        FixFinding {
            rule_id,
            class,
            target: target.to_string(),
            message: format!("test: {}", rule_id),
            remediation: format!("fix {}", rule_id),
            patch_preview: None,
        }
    }

    fn make_finding_with_patch(rule_id: &'static str, class: FixClass, target: &str) -> FixFinding {
        FixFinding {
            rule_id,
            class,
            target: target.to_string(),
            message: format!("test: {}", rule_id),
            remediation: format!("fix {}", rule_id),
            patch_preview: Some("--- a/file\n+++ b/file\n@@\n-old\n+new\n".to_string()),
        }
    }

    // ── plan_item_from_finding ──────────────────────────────────────────────

    #[test]
    fn auto_fixable_item_is_pre_approved() {
        let finding = make_finding("required-root-file", FixClass::AutoFixable, "project.arch.yaml");
        let item = plan_item_from_finding(1, &finding);
        assert_eq!(item.id, "fix-001");
        assert!(!item.approval_required);
        assert_eq!(item.approval_status, ApprovalStatus::Approved);
        assert!(item.approved_by.is_none());
    }

    #[test]
    fn review_required_item_starts_pending() {
        let finding = make_finding("artifact-module-defined", FixClass::ReviewRequired, "artifact:foo");
        let item = plan_item_from_finding(2, &finding);
        assert_eq!(item.id, "fix-002");
        assert!(item.approval_required);
        assert_eq!(item.approval_status, ApprovalStatus::Pending);
    }

    #[test]
    fn patch_preview_is_propagated() {
        let finding = make_finding_with_patch("artifact-role-defined", FixClass::ReviewRequired, "artifact:bar");
        let item = plan_item_from_finding(3, &finding);
        assert!(item.patch_preview.is_some());
    }

    // ── plan construction ──────────────────────────────────────────────────

    fn build_plan_from_findings(findings: &[FixFinding]) -> FixRolloutPlan {
        let now = 0u64;
        let items: Vec<FixPlanItem> = findings
            .iter()
            .enumerate()
            .map(|(i, f)| plan_item_from_finding(i + 1, f))
            .collect();
        let auto_fixable_count = items
            .iter()
            .filter(|item| item.fix_class == FixClass::AutoFixable)
            .count();
        let review_required_count = items
            .iter()
            .filter(|item| item.fix_class == FixClass::ReviewRequired)
            .count();
        FixRolloutPlan {
            plan_id: now,
            created_at_unix: now,
            repository: ".".to_string(),
            total_items: items.len(),
            auto_fixable_count,
            review_required_count,
            items,
            applied: false,
            applied_at_unix: None,
        }
    }

    #[test]
    fn plan_counts_correct() {
        let findings = vec![
            make_finding("required-root-file", FixClass::AutoFixable, "project.arch.yaml"),
            make_finding("artifact-module-defined", FixClass::ReviewRequired, "artifact:a"),
            make_finding("artifact-role-defined", FixClass::ReviewRequired, "artifact:b"),
        ];
        let plan = build_plan_from_findings(&findings);
        assert_eq!(plan.total_items, 3);
        assert_eq!(plan.auto_fixable_count, 1);
        assert_eq!(plan.review_required_count, 2);
        assert!(!plan.applied);
    }

    #[test]
    fn empty_findings_produce_correct_plan() {
        let plan = build_plan_from_findings(&[]);
        assert_eq!(plan.total_items, 0);
        assert_eq!(plan.auto_fixable_count, 0);
        assert_eq!(plan.review_required_count, 0);
    }

    // ── approval logic ─────────────────────────────────────────────────────

    #[test]
    fn approve_all_transitions_pending_to_approved() {
        let findings = vec![
            make_finding("artifact-module-defined", FixClass::ReviewRequired, "artifact:a"),
            make_finding("artifact-role-defined", FixClass::ReviewRequired, "artifact:b"),
        ];
        let mut plan = build_plan_from_findings(&findings);
        let now = now_unix();
        for item in &mut plan.items {
            if item.approval_required {
                item.approval_status = ApprovalStatus::Approved;
                item.approved_by = Some("alice".to_string());
                item.approved_at_unix = Some(now);
            }
        }
        assert!(plan
            .items
            .iter()
            .all(|item| item.approval_status == ApprovalStatus::Approved));
    }

    #[test]
    fn reject_transitions_pending_to_rejected() {
        let findings = vec![
            make_finding("artifact-module-defined", FixClass::ReviewRequired, "artifact:a"),
        ];
        let mut plan = build_plan_from_findings(&findings);
        for item in &mut plan.items {
            if item.approval_required {
                item.approval_status = ApprovalStatus::Rejected;
            }
        }
        assert_eq!(plan.items[0].approval_status, ApprovalStatus::Rejected);
    }

    #[test]
    fn auto_fixable_items_are_not_gated_by_approval() {
        let findings = vec![
            make_finding("required-root-file", FixClass::AutoFixable, "project.arch.yaml"),
        ];
        let plan = build_plan_from_findings(&findings);
        // Gate check: no pending approval-required items.
        let pending = plan
            .items
            .iter()
            .filter(|item| item.approval_required && item.approval_status == ApprovalStatus::Pending)
            .count();
        assert_eq!(pending, 0); // can proceed to apply immediately
    }

    #[test]
    fn pending_items_block_apply_gate() {
        let findings = vec![
            make_finding("artifact-module-defined", FixClass::ReviewRequired, "artifact:a"),
        ];
        let plan = build_plan_from_findings(&findings);
        let pending = plan
            .items
            .iter()
            .filter(|item| item.approval_required && item.approval_status == ApprovalStatus::Pending)
            .count();
        assert_eq!(pending, 1); // apply must be blocked
    }

    // ── serialization round-trip ────────────────────────────────────────────

    #[test]
    fn plan_survives_json_round_trip() {
        let findings = vec![
            make_finding("required-root-file", FixClass::AutoFixable, "project.arch.yaml"),
            make_finding_with_patch("artifact-module-defined", FixClass::ReviewRequired, "artifact:x"),
        ];
        let plan = build_plan_from_findings(&findings);
        let json = serde_json::to_string_pretty(&plan).expect("serialize");
        let restored: FixRolloutPlan = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.total_items, plan.total_items);
        assert_eq!(restored.items[0].id, "fix-001");
        assert_eq!(restored.items[1].approval_status, ApprovalStatus::Pending);
        assert!(restored.items[1].patch_preview.is_some());
    }

    #[test]
    fn items_with_no_patch_omit_field_in_json() {
        let finding = make_finding("required-root-file", FixClass::AutoFixable, "project.arch.yaml");
        let item = plan_item_from_finding(1, &finding);
        let json = serde_json::to_string(&item).expect("serialize");
        assert!(!json.contains("patch_preview"));
    }

    #[test]
    fn approval_status_strings_are_lowercase() {
        assert_eq!(ApprovalStatus::Pending.as_str(), "pending");
        assert_eq!(ApprovalStatus::Approved.as_str(), "approved");
        assert_eq!(ApprovalStatus::Rejected.as_str(), "rejected");
    }
}
