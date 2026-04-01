use std::fs;
use std::path::Path;

use crate::model::verify::{CheckResult, VerifyStatus, VerifyTarget};

/// Parsed identity fields extracted from a generated `.prompt.md` file.
struct PromptIdentity {
    artifact_name: Option<String>,
    role: Option<String>,
    module: Option<String>,
}

/// Attempts to extract identity fields from a generated prompt markdown.
///
/// Generated prompts follow the structure produced by `Prompt::format_standard`:
/// ```text
/// # Artifact Prompt: {name}
/// …
/// ## Role
/// {role}
/// ## Module
/// {module}
/// ```
///
/// Returns `None` if the file does not look like a generated prompt (e.g. a
/// manual sidecar override that only contains an HTML comment).
fn parse_prompt_identity(contents: &str) -> Option<PromptIdentity> {
    let mut artifact_name: Option<String> = None;
    let mut role: Option<String> = None;
    let mut module: Option<String> = None;

    let mut lines = contents.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        if trimmed.starts_with("# Artifact Prompt:") {
            let name = trimmed
                .trim_start_matches("# Artifact Prompt:")
                .trim()
                .to_string();
            if !name.is_empty() {
                artifact_name = Some(name);
            }
        } else if trimmed == "## Role" {
            if let Some(next) = lines.next() {
                let v = next.trim().to_string();
                if !v.is_empty() {
                    role = Some(v);
                }
            }
        } else if trimmed == "## Module" {
            if let Some(next) = lines.next() {
                let v = next.trim().to_string();
                if !v.is_empty() {
                    module = Some(v);
                }
            }
        }
    }

    // A generated prompt must at least have the artifact name heading.
    if artifact_name.is_none() && role.is_none() && module.is_none() {
        return None;
    }

    Some(PromptIdentity {
        artifact_name,
        role,
        module,
    })
}

/// Validates the naming and identity fields of an existing prompt file.
///
/// Checks performed:
/// - `prompt-artifact-naming`: The `# Artifact Prompt:` heading matches the expected artifact name.
/// - `prompt-contract-identity`: The `## Role` and `## Module` values match the contract-derived
///   expectations (artifact role and module from the plan).
///
/// If the file cannot be read, or it does not look like a generated prompt (e.g. a
/// manual sidecar override), a single Skip result is emitted for each check so
/// that the verification log remains complete without false failures.
pub fn validate_prompt_identity<P: AsRef<Path>>(
    prompt_path: P,
    expected_name: &str,
    expected_role: &str,
    expected_module: &str,
) -> Vec<CheckResult> {
    let prompt_path = prompt_path.as_ref();
    let mut results = Vec::new();

    let make_target = || VerifyTarget::Prompt {
        artifact_name: expected_name.to_string(),
        path: prompt_path.to_string_lossy().to_string(),
    };

    let contents = match fs::read_to_string(prompt_path) {
        Ok(s) => s,
        Err(e) => {
            results.push(CheckResult {
                check_id: "prompt-artifact-naming".to_string(),
                target: make_target(),
                status: VerifyStatus::Fail,
                message: format!(
                    "Could not read prompt file for '{}': {}",
                    expected_name, e
                ),
            });
            return results;
        }
    };

    let identity = match parse_prompt_identity(&contents) {
        Some(id) => id,
        None => {
            // Manual sidecar override — skip identity checks silently.
            results.push(CheckResult {
                check_id: "prompt-artifact-naming".to_string(),
                target: make_target(),
                status: VerifyStatus::Skip,
                message: format!(
                    "Prompt for '{}' appears to be a manual override; skipping naming check",
                    expected_name
                ),
            });
            results.push(CheckResult {
                check_id: "prompt-contract-identity".to_string(),
                target: make_target(),
                status: VerifyStatus::Skip,
                message: format!(
                    "Prompt for '{}' appears to be a manual override; skipping identity check",
                    expected_name
                ),
            });
            return results;
        }
    };

    // --- prompt-artifact-naming ---
    match &identity.artifact_name {
        Some(name) if name == expected_name => {
            results.push(CheckResult {
                check_id: "prompt-artifact-naming".to_string(),
                target: make_target(),
                status: VerifyStatus::Pass,
                message: format!(
                    "Prompt artifact name matches plan for '{}'",
                    expected_name
                ),
            });
        }
        Some(name) => {
            results.push(CheckResult {
                check_id: "prompt-artifact-naming".to_string(),
                target: make_target(),
                status: VerifyStatus::Fail,
                message: format!(
                    "Prompt artifact name mismatch for '{}': expected '{}', found '{}'",
                    expected_name, expected_name, name
                ),
            });
        }
        None => {
            results.push(CheckResult {
                check_id: "prompt-artifact-naming".to_string(),
                target: make_target(),
                status: VerifyStatus::Warn,
                message: format!(
                    "Prompt for '{}' is missing the '# Artifact Prompt:' heading",
                    expected_name
                ),
            });
        }
    }

    // --- prompt-contract-identity (role + module) ---
    let mut mismatches = Vec::new();

    match &identity.role {
        Some(r) if r != expected_role => {
            mismatches.push(format!(
                "role: expected '{}', found '{}'",
                expected_role, r
            ));
        }
        None => {
            mismatches.push(format!("role: expected '{}', not found", expected_role));
        }
        _ => {}
    }

    match &identity.module {
        Some(m) if m != expected_module => {
            mismatches.push(format!(
                "module: expected '{}', found '{}'",
                expected_module, m
            ));
        }
        None => {
            mismatches.push(format!(
                "module: expected '{}', not found",
                expected_module
            ));
        }
        _ => {}
    }

    if mismatches.is_empty() {
        results.push(CheckResult {
            check_id: "prompt-contract-identity".to_string(),
            target: make_target(),
            status: VerifyStatus::Pass,
            message: format!(
                "Prompt identity matches contract for '{}'",
                expected_name
            ),
        });
    } else {
        results.push(CheckResult {
            check_id: "prompt-contract-identity".to_string(),
            target: make_target(),
            status: VerifyStatus::Fail,
            message: format!(
                "Prompt identity mismatch for '{}': {}",
                expected_name,
                mismatches.join(", ")
            ),
        });
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_prompt(content: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "{}", content).unwrap();
        f
    }

    const GENERATED_PROMPT: &str = "\
# Artifact Prompt: UserService

Implement the `UserService` artifact.

## Role
service

## Module
user
";

    #[test]
    fn pass_when_all_identity_fields_match() {
        let f = write_prompt(GENERATED_PROMPT);
        let results = validate_prompt_identity(f.path(), "UserService", "service", "user");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].check_id, "prompt-artifact-naming");
        assert_eq!(results[0].status, VerifyStatus::Pass);
        assert_eq!(results[1].check_id, "prompt-contract-identity");
        assert_eq!(results[1].status, VerifyStatus::Pass);
    }

    #[test]
    fn fail_artifact_naming_when_name_mismatch() {
        let f = write_prompt(GENERATED_PROMPT);
        // expected_name differs from what the file contains
        let results = validate_prompt_identity(f.path(), "OtherService", "service", "user");
        assert_eq!(results[0].check_id, "prompt-artifact-naming");
        assert_eq!(results[0].status, VerifyStatus::Fail);
    }

    #[test]
    fn fail_contract_identity_when_role_mismatch() {
        let f = write_prompt(GENERATED_PROMPT);
        let results = validate_prompt_identity(f.path(), "UserService", "repository", "user");
        assert_eq!(results[1].check_id, "prompt-contract-identity");
        assert_eq!(results[1].status, VerifyStatus::Fail);
        assert!(results[1].message.contains("role"));
    }

    #[test]
    fn fail_contract_identity_when_module_mismatch() {
        let f = write_prompt(GENERATED_PROMPT);
        let results = validate_prompt_identity(f.path(), "UserService", "service", "order");
        assert_eq!(results[1].check_id, "prompt-contract-identity");
        assert_eq!(results[1].status, VerifyStatus::Fail);
        assert!(results[1].message.contains("module"));
    }

    #[test]
    fn skip_when_manual_override_prompt() {
        let content = "<!-- Prompt override for: user (entity) -->\n";
        let f = write_prompt(content);
        let results = validate_prompt_identity(f.path(), "user", "entity", "user");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].status, VerifyStatus::Skip);
        assert_eq!(results[1].status, VerifyStatus::Skip);
    }

    #[test]
    fn warn_when_artifact_name_heading_missing() {
        let content = "## Role\nservice\n\n## Module\nuser\n";
        let f = write_prompt(content);
        let results = validate_prompt_identity(f.path(), "UserService", "service", "user");
        assert_eq!(results[0].check_id, "prompt-artifact-naming");
        assert_eq!(results[0].status, VerifyStatus::Warn);
    }
}
