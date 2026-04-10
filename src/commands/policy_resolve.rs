use crate::config::override_policy::{
    load_effective_policy, EffectivePolicy, OverrideLevel,
};
use crate::config::rbac::{GovernanceOperation, RbacEvaluator};
use std::path::Path;

pub fn execute_cli(
    org_policy: Option<&str>,
    team_policy: Option<&str>,
    project_policy: Option<&str>,
    actor: Option<&str>,
) {
    let org_path = org_policy.map(Path::new);
    let team_path = team_policy.map(Path::new);
    let project_path = project_policy.map(Path::new);

    let ep = match crate::config::override_policy::resolve(org_path, team_path, project_path) {
        Ok(ep) => ep,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    if let Some(actor_name) = actor {
        let auth_result = RbacEvaluator::authorize_operation(&ep, actor_name, GovernanceOperation::RunAudit);
        if !auth_result.is_allowed() {
            eprintln!("[!] RBAC Denied: actor '{}' is not authorized to run audit/policy commands.", actor_name);
            std::process::exit(1);
        }
    }

    render_effective_policy(&ep);
}

/// Load effective policy using default paths and render it.
/// Called without explicit path arguments — uses standard lookup paths.
#[allow(dead_code)]
pub fn execute_default_cli(actor: Option<&str>) {
    let ep = match load_effective_policy() {
        Ok(ep) => ep,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    if let Some(actor_name) = actor {
        let auth_result = RbacEvaluator::authorize_operation(&ep, actor_name, GovernanceOperation::RunAudit);
        if !auth_result.is_allowed() {
            eprintln!("[!] RBAC Denied: actor '{}' is not authorized to run audit/policy commands.", actor_name);
            std::process::exit(1);
        }
    }

    render_effective_policy(&ep);
}

pub fn render_effective_policy(ep: &EffectivePolicy) {
    println!("Archflow Effective Policy Resolution");
    println!("=====================================");
    println!();

    // Precedence chain
    println!("Precedence chain (highest → lowest):");
    println!("  org → team → project → default");
    println!();

    // Locked rules
    if ep.locked_rules.is_empty() {
        println!("Locked rules: (none)");
    } else {
        println!("Locked rules:");
        for locked in &ep.locked_rules {
            println!(
                "  - {} [locked by: {}]",
                locked.rule_id, locked.source_label
            );
        }
    }
    println!();

    // Naming rules
    let (module_rule, module_level, module_src) = &ep.module_naming;
    let (artifact_rule, artifact_level, artifact_src) = &ep.artifact_naming;
    println!("Naming rules:");
    println!(
        "  module:   {:?}  [source: {}]",
        module_rule, module_src
    );
    println!(
        "  artifact: {:?}  [source: {}]",
        artifact_rule, artifact_src
    );
    let _ = module_level;
    let _ = artifact_level;
    println!();

    // Required files
    println!("Required files:");
    for req in &ep.required_files {
        println!("  - {}  [source: {}]", req.filename, req.source_label);
    }
    println!();

    // Forbidden dependencies
    if ep.forbidden_dependencies.is_empty() {
        println!("Forbidden dependencies: (none)");
    } else {
        println!("Forbidden dependencies:");
        for dep in &ep.forbidden_dependencies {
            println!(
                "  role: {}  [source: {}]",
                dep.role, dep.source_label
            );
            for entry in &dep.forbidden_entries {
                println!("    - {}", entry);
            }
        }
    }
    println!();

    // Overrides
    if ep.overrides.is_empty() {
        println!("Overrides: (none)");
    } else {
        println!("Overrides:");
        for entry in &ep.overrides {
            println!(
                "  rule: {}  [source: {}]",
                entry.rule_id, entry.source_label
            );
            println!("    reason: {}", entry.reason);
            for target in &entry.targets {
                println!("    - {}", target);
            }
        }
    }
    println!();

    // Governance Roles
    if ep.governance_roles.is_empty() {
        println!("Governance roles: (none mapped)");
    } else {
        println!("Governance roles:");
        for role_binding in &ep.governance_roles {
            println!(
                "  role: {}  [source: {}]",
                role_binding.role, role_binding.source_label
            );
            println!("    members:");
            for member in &role_binding.members {
                println!("      - {}", member);
            }
        }
    }
    println!();

    // Summary of what cannot be overridden at project level
    let project_blocked: Vec<&str> = ep
        .locked_rules
        .iter()
        .filter(|locked| locked.source_level != OverrideLevel::Project)
        .map(|locked| locked.rule_id.as_str())
        .collect();
    if project_blocked.is_empty() {
        println!("Override constraints: projects may override all rules");
    } else {
        println!("Override constraints: the following rules are locked and cannot be overridden at project level:");
        for rule_id in project_blocked {
            println!("  - {}", rule_id);
        }
    }
}
