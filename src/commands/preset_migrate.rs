/// Preset versioning and migration tooling.
///
/// `preset-migration-plan`  — compare the current project's preset-origin files against
///   a target preset version and produce a structured migration plan with patch previews.
///
/// `preset-migration-apply` — apply safe patches from a migration plan with rollback backups.
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Public surface
// ---------------------------------------------------------------------------

/// Severity of a migration step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationClass {
    /// File is missing and will be added from the preset.
    AddFile,
    /// File content differs; the diff is shown for review before applying.
    UpdateFile,
    /// File is new in the target preset but already exists locally (conflict risk).
    ConflictFile,
    /// File is unchanged; no action required.
    Unchanged,
}

impl MigrationClass {
    fn label(self) -> &'static str {
        match self {
            MigrationClass::AddFile => "add",
            MigrationClass::UpdateFile => "update",
            MigrationClass::ConflictFile => "conflict",
            MigrationClass::Unchanged => "unchanged",
        }
    }

    fn is_auto_applicable(self) -> bool {
        matches!(self, MigrationClass::AddFile | MigrationClass::UpdateFile)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStep {
    pub file: String,
    pub class: MigrationClass,
    pub message: String,
    /// Patch preview shown in plan mode (unified diff-style text or new file content).
    pub patch_preview: Option<String>,
    pub remediation: String,
    /// The exact content to write when applying this step (only set for add/update classes).
    #[serde(skip)]
    pub target_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPlan {
    pub preset_id: String,
    pub from_version: String,
    pub to_version: String,
    pub steps: Vec<MigrationStep>,
}

impl MigrationPlan {
    pub fn has_conflicts(&self) -> bool {
        self.steps
            .iter()
            .any(|step| step.class == MigrationClass::ConflictFile)
    }

    #[allow(dead_code)]
    pub fn actionable_steps(&self) -> Vec<&MigrationStep> {
        self.steps
            .iter()
            .filter(|step| step.class != MigrationClass::Unchanged)
            .collect()
    }
}

// ---------------------------------------------------------------------------
// `preset-migration-plan` entry point
// ---------------------------------------------------------------------------

pub fn execute_plan_cli(
    preset_id: &str,
    from_version: &str,
    to_version: &str,
    registry_dir: &str,
    project_dir: &str,
) {
    let plan = match build_migration_plan(
        preset_id,
        from_version,
        to_version,
        Path::new(registry_dir),
        Path::new(project_dir),
    ) {
        Ok(plan) => plan,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    render_plan(&plan);

    if plan.has_conflicts() {
        std::process::exit(1);
    }
}

// ---------------------------------------------------------------------------
// `preset-migration-apply` entry point
// ---------------------------------------------------------------------------

pub fn execute_apply_cli(
    preset_id: &str,
    from_version: &str,
    to_version: &str,
    registry_dir: &str,
    project_dir: &str,
    dry_run: bool,
) {
    let plan = match build_migration_plan(
        preset_id,
        from_version,
        to_version,
        Path::new(registry_dir),
        Path::new(project_dir),
    ) {
        Ok(plan) => plan,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    render_plan(&plan);

    if plan.has_conflicts() {
        eprintln!(
            "\n[!] Migration has {} conflict(s); resolve them before applying.",
            plan.steps
                .iter()
                .filter(|step| step.class == MigrationClass::ConflictFile)
                .count()
        );
        std::process::exit(1);
    }

    let actionable: Vec<&MigrationStep> = plan
        .steps
        .iter()
        .filter(|step| step.class.is_auto_applicable())
        .collect();

    if actionable.is_empty() {
        println!("\nNothing to apply.");
        return;
    }

    if dry_run {
        println!("\n[i] Dry run: {} step(s) would be applied.", actionable.len());
        return;
    }

    let backup_dir = make_backup_dir(Path::new(project_dir), &plan.from_version);

    println!("\nApplying migration...");
    println!("  backup directory: {}", backup_dir.display());

    let mut applied = 0;
    let mut failed = 0;
    for step in &actionable {
        let target_path = Path::new(project_dir).join(&step.file);
        // Create a backup of any existing file before overwriting.
        if target_path.exists() {
            let backup_path = backup_dir.join(&step.file);
            if let Some(parent) = backup_path.parent() {
                if let Err(err) = fs::create_dir_all(parent) {
                    eprintln!("  [!] failed to create backup dir: {}", err);
                    failed += 1;
                    continue;
                }
            }
            if let Err(err) = fs::copy(&target_path, &backup_path) {
                eprintln!(
                    "  [!] failed to back up '{}': {}",
                    target_path.display(),
                    err
                );
                failed += 1;
                continue;
            }
        }

        // Write new content from the patch preview for AddFile steps.
        // For UpdateFile steps, overwrite with preset content.
        match write_step(step, &target_path) {
            Ok(_) => {
                println!("  [{}] {}", step.class.label(), step.file);
                applied += 1;
            }
            Err(err) => {
                eprintln!("  [!] failed to apply '{}': {}", step.file, err);
                failed += 1;
            }
        }
    }

    println!();
    println!("Migration complete: {} applied, {} failed.", applied, failed);
    if applied > 0 {
        println!(
            "  Rollback: restore files from backup at '{}'.",
            backup_dir.display()
        );
    }

    if failed > 0 {
        std::process::exit(1);
    }
}

// ---------------------------------------------------------------------------
// Core plan builder
// ---------------------------------------------------------------------------

/// Compare the current project directory against the target preset version and
/// produce a structured `MigrationPlan`.
pub fn build_migration_plan(
    preset_id: &str,
    from_version: &str,
    to_version: &str,
    registry_dir: &Path,
    project_dir: &Path,
) -> Result<MigrationPlan, String> {
    validate_semver(from_version, "from_version")?;
    validate_semver(to_version, "to_version")?;

    if compare_semver(to_version, from_version) != std::cmp::Ordering::Greater {
        return Err(format!(
            "to_version '{}' must be greater than from_version '{}'",
            to_version, from_version
        ));
    }

    let from_dir = registry_dir
        .join("packages")
        .join(preset_id)
        .join(from_version);
    if !from_dir.is_dir() {
        return Err(format!(
            "source preset version '{}' package not found at '{}'",
            from_version,
            from_dir.display()
        ));
    }

    let to_dir = registry_dir
        .join("packages")
        .join(preset_id)
        .join(to_version);
    if !to_dir.is_dir() {
        return Err(format!(
            "target preset version '{}' package not found at '{}'",
            to_version,
            to_dir.display()
        ));
    }

    // Collect all files from the target preset.
    let target_files = collect_preset_files(&to_dir)?;
    let mut steps = Vec::new();

    for relative_path in &target_files {
        // Skip metadata-only files that are not part of the project configuration.
        if relative_path == "preset.yaml" || relative_path == "README.md" {
            continue;
        }

        let from_content = read_file_opt(&from_dir.join(relative_path));
        let to_content = match fs::read_to_string(to_dir.join(relative_path)) {
            Ok(content) => content,
            Err(err) => {
                return Err(format!(
                    "failed to read target file '{}': {}",
                    relative_path, err
                ));
            }
        };
        let project_content = read_file_opt(&project_dir.join(relative_path));

        let step = classify_migration_step(
            relative_path,
            from_content.as_deref(),
            &to_content,
            project_content.as_deref(),
        );
        steps.push(step);
    }

    // Sort: add → update → conflict → unchanged
    steps.sort_by_key(|step| match step.class {
        MigrationClass::AddFile => 0,
        MigrationClass::UpdateFile => 1,
        MigrationClass::ConflictFile => 2,
        MigrationClass::Unchanged => 3,
    });

    Ok(MigrationPlan {
        preset_id: preset_id.to_string(),
        from_version: from_version.to_string(),
        to_version: to_version.to_string(),
        steps,
    })
}

// ---------------------------------------------------------------------------
// Step classification
// ---------------------------------------------------------------------------

fn classify_migration_step(
    file: &str,
    from_content: Option<&str>,
    to_content: &str,
    project_content: Option<&str>,
) -> MigrationStep {
    match (from_content, project_content) {
        // File is brand new in the target (not in from_ at all).
        (None, None) => MigrationStep {
            file: file.to_string(),
            class: MigrationClass::AddFile,
            message: format!(
                "'{}' is new in the target preset version; it will be added",
                file
            ),
            patch_preview: Some(format!("[+] {}\n{}", file, to_content)),
            remediation: "Run `preset-migration-apply` to add this file.".to_string(),
            target_content: Some(to_content.to_string()),
        },
        // File exists in project but was not part of the from_ preset version (potential local file).
        (None, Some(_)) => MigrationStep {
            file: file.to_string(),
            class: MigrationClass::ConflictFile,
            message: format!(
                "'{}' is new in the target preset but already exists locally; manual merge required",
                file
            ),
            patch_preview: Some(format!("[target]\n{}", to_content)),
            remediation:
                "Review the target preset content and merge manually into your local file."
                    .to_string(),
            target_content: None,
        },
        // File was in from_, exists in project, and to_ is different from from_.
        (Some(from), Some(project)) => {
            if to_content == from {
                // Target preset is same as source preset; no upgrade action needed.
                MigrationStep {
                    file: file.to_string(),
                    class: MigrationClass::Unchanged,
                    message: format!("'{}' unchanged between preset versions", file),
                    patch_preview: None,
                    remediation: String::new(),
                    target_content: None,
                }
            } else if project == from {
                // Project hasn't diverged from the old preset; overwrite is safe.
                MigrationStep {
                    file: file.to_string(),
                    class: MigrationClass::UpdateFile,
                    message: format!(
                        "'{}' changed in target preset and local copy matches the old preset (safe to apply)",
                        file
                    ),
                    patch_preview: Some(build_unified_diff(file, project, to_content)),
                    remediation: "Run `preset-migration-apply` to apply this update.".to_string(),
                    target_content: Some(to_content.to_string()),
                }
            } else {
                // Project has diverged from old preset; flag for human review.
                MigrationStep {
                    file: file.to_string(),
                    class: MigrationClass::ConflictFile,
                    message: format!(
                        "'{}' changed in both the project and the target preset; manual merge required",
                        file
                    ),
                    patch_preview: Some(build_unified_diff(file, from, to_content)),
                    remediation:
                        "Review both the local changes and the preset update; merge manually."
                            .to_string(),
                    target_content: None,
                }
            }
        }
        // File was in from_, target has it, but it doesn't currently exist in the project.
        (Some(_), None) => MigrationStep {
            file: file.to_string(),
            class: MigrationClass::AddFile,
            message: format!(
                "'{}' is missing locally but exists in the target preset; it will be added",
                file
            ),
            patch_preview: Some(format!("[+] {}\n{}", file, to_content)),
            remediation: "Run `preset-migration-apply` to restore this file.".to_string(),
            target_content: Some(to_content.to_string()),
        },
    }
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

pub fn render_plan(plan: &MigrationPlan) {
    println!("Batonel Preset Migration Plan");
    println!("==============================");
    println!("  preset:       {}", plan.preset_id);
    println!("  from version: {}", plan.from_version);
    println!("  to version:   {}", plan.to_version);
    println!();

    let adds: Vec<&MigrationStep> = plan
        .steps
        .iter()
        .filter(|step| step.class == MigrationClass::AddFile)
        .collect();
    let updates: Vec<&MigrationStep> = plan
        .steps
        .iter()
        .filter(|step| step.class == MigrationClass::UpdateFile)
        .collect();
    let conflicts: Vec<&MigrationStep> = plan
        .steps
        .iter()
        .filter(|step| step.class == MigrationClass::ConflictFile)
        .collect();
    let unchanged: Vec<&MigrationStep> = plan
        .steps
        .iter()
        .filter(|step| step.class == MigrationClass::Unchanged)
        .collect();

    println!(
        "Summary: {} add, {} update, {} conflict, {} unchanged",
        adds.len(),
        updates.len(),
        conflicts.len(),
        unchanged.len()
    );

    if plan.steps.iter().all(|step| step.class == MigrationClass::Unchanged) {
        println!("Status: UP-TO-DATE");
        return;
    }

    println!(
        "Status: {}",
        if plan.has_conflicts() {
            "CONFLICTS — manual review required"
        } else {
            "READY — no conflicts detected"
        }
    );

    for section in [&adds, &updates, &conflicts] {
        for step in section {
            println!();
            println!("[{}] {}", step.class.label(), step.file);
            println!("  {}", step.message);
            println!("  remediation: {}", step.remediation);
            if let Some(preview) = &step.patch_preview {
                println!("  patch preview:");
                for line in preview.lines() {
                    println!("    {}", line);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn collect_preset_files(dir: &Path) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    collect_files_recursive(dir, dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files_recursive(
    root: &Path,
    dir: &Path,
    files: &mut Vec<String>,
) -> Result<(), String> {
    for entry in
        fs::read_dir(dir).map_err(|err| format!("failed to read dir '{}': {}", dir.display(), err))?
    {
        let entry = entry.map_err(|err| format!("failed to read entry: {}", err))?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_recursive(root, &path, files)?;
        } else if path.is_file() {
            let rel = path
                .strip_prefix(root)
                .map_err(|err| format!("path strip error: {}", err))?;
            files.push(rel.to_string_lossy().to_string());
        }
    }
    Ok(())
}

fn read_file_opt(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

/// Build a minimal line-level diff preview (additions and removals) for display.
fn build_unified_diff(filename: &str, old: &str, new: &str) -> String {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();

    let mut output = format!("--- {}\n+++ {} (preset target)\n", filename, filename);

    // Naive line diff: emit removals (lines in old but not in new) and additions
    // (lines in new but not in old). This is intentionally simple – it gives
    // reviewers a clear picture of what changed without requiring an external diff library.
    let old_set: std::collections::HashSet<&str> = old_lines.iter().copied().collect();
    let new_set: std::collections::HashSet<&str> = new_lines.iter().copied().collect();

    for line in &old_lines {
        if !new_set.contains(*line) {
            output.push_str(&format!("- {}\n", line));
        }
    }
    for line in &new_lines {
        if !old_set.contains(*line) {
            output.push_str(&format!("+ {}\n", line));
        }
    }

    if output.len() == format!("--- {}\n+++ {} (preset target)\n", filename, filename).len() {
        output.push_str("(no line-level changes detected — file may differ only in whitespace)\n");
    }

    output
}

fn make_backup_dir(project_dir: &Path, from_version: &str) -> PathBuf {
    project_dir.join(format!(".batonel/migration-backup/{}", from_version))
}

fn write_step(step: &MigrationStep, target_path: &Path) -> Result<(), String> {
    let content = match &step.target_content {
        Some(c) => c.clone(),
        None => return Ok(()), // nothing to write for conflict/unchanged
    };

    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create parent dir: {}", err))?;
    }
    fs::write(target_path, content)
        .map_err(|err| format!("failed to write '{}': {}", target_path.display(), err))?;
    Ok(())
}

fn validate_semver(value: &str, label: &str) -> Result<(), String> {
    if !is_semver(value) {
        return Err(format!(
            "{} '{}' must follow semver format x.y.z",
            label, value
        ));
    }
    Ok(())
}

fn is_semver(value: &str) -> bool {
    let mut parts = value.split('.');
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(major), Some(minor), Some(patch), None) => {
            major.chars().all(|ch| ch.is_ascii_digit())
                && minor.chars().all(|ch| ch.is_ascii_digit())
                && patch.chars().all(|ch| ch.is_ascii_digit())
        }
        _ => false,
    }
}

fn compare_semver(left: &str, right: &str) -> std::cmp::Ordering {
    fn parse(v: &str) -> (u64, u64, u64) {
        let mut parts = v.split('.');
        let major = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
        let minor = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
        let patch = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
        (major, minor, patch)
    }
    parse(left).cmp(&parse(right))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write(dir: &Path, name: &str, content: &str) {
        if let Some(parent) = dir.join(name).parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(dir.join(name), content).unwrap();
    }

    fn make_preset_pkg(root: &Path, id: &str, version: &str, files: &[(&str, &str)]) {
        let pkg_dir = root.join("packages").join(id).join(version);
        fs::create_dir_all(&pkg_dir).unwrap();
        for (name, content) in files {
            write(&pkg_dir, name, content);
        }
    }

    #[test]
    fn migration_plan_detects_add_file() {
        let registry = tempdir().unwrap();
        let project = tempdir().unwrap();

        make_preset_pkg(
            registry.path(),
            "test-preset",
            "0.1.0",
            &[("placement.rules.yaml", "roles:\n  entity:\n    path: src/domain/\n")],
        );
        make_preset_pkg(
            registry.path(),
            "test-preset",
            "0.2.0",
            &[
                ("placement.rules.yaml", "roles:\n  entity:\n    path: src/domain/\n"),
                ("guard.sidecar.yaml", "version: 1\nhooks:\n  ci: true\n"),
            ],
        );
        // project only has the old file
        write(
            project.path(),
            "placement.rules.yaml",
            "roles:\n  entity:\n    path: src/domain/\n",
        );

        let plan = build_migration_plan(
            "test-preset",
            "0.1.0",
            "0.2.0",
            registry.path(),
            project.path(),
        )
        .unwrap();

        let add_steps: Vec<&MigrationStep> = plan
            .steps
            .iter()
            .filter(|s| s.class == MigrationClass::AddFile)
            .collect();
        assert_eq!(
            add_steps.len(),
            1,
            "Expected 1 add step, got: {:?}",
            plan.steps
                .iter()
                .map(|s| (&s.file, s.class.label()))
                .collect::<Vec<_>>()
        );
        assert_eq!(add_steps[0].file, "guard.sidecar.yaml");
    }

    #[test]
    fn migration_plan_detects_safe_update() {
        let registry = tempdir().unwrap();
        let project = tempdir().unwrap();

        make_preset_pkg(
            registry.path(),
            "test-preset",
            "0.1.0",
            &[("contracts.template.yaml", "role_templates:\n  entity: {}\n")],
        );
        make_preset_pkg(
            registry.path(),
            "test-preset",
            "0.2.0",
            &[(
                "contracts.template.yaml",
                "role_templates:\n  entity:\n    responsibilities:\n      - rep\n",
            )],
        );
        // project matches the old preset (no local divergence)
        write(
            project.path(),
            "contracts.template.yaml",
            "role_templates:\n  entity: {}\n",
        );

        let plan = build_migration_plan(
            "test-preset",
            "0.1.0",
            "0.2.0",
            registry.path(),
            project.path(),
        )
        .unwrap();

        let update_steps: Vec<&MigrationStep> = plan
            .steps
            .iter()
            .filter(|s| s.class == MigrationClass::UpdateFile)
            .collect();
        assert_eq!(update_steps.len(), 1, "Expected 1 update step");
        assert_eq!(update_steps[0].file, "contracts.template.yaml");
        assert!(!plan.has_conflicts());
    }

    #[test]
    fn migration_plan_detects_conflict_when_project_diverged() {
        let registry = tempdir().unwrap();
        let project = tempdir().unwrap();

        make_preset_pkg(
            registry.path(),
            "test-preset",
            "0.1.0",
            &[("placement.rules.yaml", "roles:\n  entity:\n    path: src/\n")],
        );
        make_preset_pkg(
            registry.path(),
            "test-preset",
            "0.2.0",
            &[("placement.rules.yaml", "roles:\n  entity:\n    path: src/domain/\n")],
        );
        // project has been locally customized
        write(
            project.path(),
            "placement.rules.yaml",
            "roles:\n  entity:\n    path: src/custom/\n",
        );

        let plan = build_migration_plan(
            "test-preset",
            "0.1.0",
            "0.2.0",
            registry.path(),
            project.path(),
        )
        .unwrap();

        assert!(plan.has_conflicts(), "Expected conflict step");
        let conflict_steps: Vec<&MigrationStep> = plan
            .steps
            .iter()
            .filter(|s| s.class == MigrationClass::ConflictFile)
            .collect();
        assert_eq!(conflict_steps.len(), 1);
    }

    #[test]
    fn migration_plan_reports_unchanged_when_no_diff() {
        let registry = tempdir().unwrap();
        let project = tempdir().unwrap();

        let content = "roles:\n  entity:\n    path: src/domain/\n";
        make_preset_pkg(
            registry.path(),
            "test-preset",
            "0.1.0",
            &[("placement.rules.yaml", content)],
        );
        make_preset_pkg(
            registry.path(),
            "test-preset",
            "0.2.0",
            &[("placement.rules.yaml", content)],
        );
        write(project.path(), "placement.rules.yaml", content);

        let plan = build_migration_plan(
            "test-preset",
            "0.1.0",
            "0.2.0",
            registry.path(),
            project.path(),
        )
        .unwrap();

        let unchanged: Vec<&MigrationStep> = plan
            .steps
            .iter()
            .filter(|s| s.class == MigrationClass::Unchanged)
            .collect();
        assert_eq!(unchanged.len(), 1);
        assert!(!plan.has_conflicts());
    }

    #[test]
    fn migration_plan_rejects_version_order() {
        let registry = tempdir().unwrap();
        let project = tempdir().unwrap();

        let result = build_migration_plan(
            "test-preset",
            "0.2.0",
            "0.1.0",
            registry.path(),
            project.path(),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be greater than"));
    }

    #[test]
    fn migration_apply_writes_add_file_and_creates_backup() {
        let registry = tempdir().unwrap();
        let project = tempdir().unwrap();

        make_preset_pkg(
            registry.path(),
            "test-preset",
            "0.1.0",
            &[("placement.rules.yaml", "roles:\n  entity:\n    path: src/\n")],
        );
        make_preset_pkg(
            registry.path(),
            "test-preset",
            "0.2.0",
            &[
                ("placement.rules.yaml", "roles:\n  entity:\n    path: src/\n"),
                ("contracts.template.yaml", "role_templates:\n  entity: {}\n"),
            ],
        );
        write(
            project.path(),
            "placement.rules.yaml",
            "roles:\n  entity:\n    path: src/\n",
        );

        let plan = build_migration_plan(
            "test-preset",
            "0.1.0",
            "0.2.0",
            registry.path(),
            project.path(),
        )
        .unwrap();

        assert!(!plan.has_conflicts());
        let add_steps: Vec<&MigrationStep> = plan
            .steps
            .iter()
            .filter(|s| s.class == MigrationClass::AddFile)
            .collect();
        assert_eq!(add_steps.len(), 1, "Expected 1 add step");

        // Simulate writing (not via CLI, directly call write_step).
        let target_path = project.path().join(&add_steps[0].file);
        write_step(add_steps[0], &target_path).expect("write_step failed");
        assert!(target_path.exists(), "contracts.template.yaml should have been created");
    }
}
