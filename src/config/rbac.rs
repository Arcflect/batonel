use crate::config::override_policy::EffectivePolicy;

/// Identifies a governance operation that requires authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernanceOperation {
    /// Authorize applying policy overrides to suppress audit bounds.
    #[allow(dead_code)]
    ApproveOverride,
    /// Authorize applying global config changes or locked rules.
    #[allow(dead_code)]
    EditPolicy,
    /// Authorize applying a patch via rollout.
    #[allow(dead_code)]
    FixRollout,
    /// Standard viewing/audit execution capabilities.
    RunAudit,
}

/// A parsed identity check outcome
#[derive(Debug, PartialEq, Eq)]
pub enum AuthorizationOutcome {
    Allowed,
    Denied(String),
}

impl AuthorizationOutcome {
    pub fn is_allowed(&self) -> bool {
        matches!(self, AuthorizationOutcome::Allowed)
    }
}

/// RBAC engine validating an arbitrary actor against loaded `EffectivePolicy` governance roles
pub struct RbacEvaluator;

impl RbacEvaluator {
    /// Check whether `actor` is allowed to perform `operation`, based on bindings in `policy`.
    /// Built-in roles:
    /// - `policy_admin`: Allowed to EditPolicy, RunAudit.
    /// - `architect`: Allowed to ApproveOverride, FixRollout, RunAudit.
    /// - `auditor`: Allowed to RunAudit.
    /// Default access for any non-mapped actor: `RunAudit`.
    pub fn authorize_operation(
        policy: &EffectivePolicy,
        actor: &str,
        operation: GovernanceOperation,
    ) -> AuthorizationOutcome {
        // Find which roles the actor belongs to based on the highest precedence level governance_roles
        let mut actor_roles = vec![];
        for binding in &policy.governance_roles {
            if binding.members.iter().any(|m| m == actor) {
                actor_roles.push(binding.role.as_str());
            }
        }

        match operation {
            GovernanceOperation::EditPolicy => {
                if actor_roles.contains(&"policy_admin") {
                    AuthorizationOutcome::Allowed
                } else {
                    AuthorizationOutcome::Denied(
                        "actor is missing required role: policy_admin".to_string(),
                    )
                }
            }
            GovernanceOperation::ApproveOverride => {
                if actor_roles.contains(&"architect") || actor_roles.contains(&"policy_admin") {
                    AuthorizationOutcome::Allowed
                } else {
                    AuthorizationOutcome::Denied(
                        "actor is missing required role: architect or policy_admin".to_string(),
                    )
                }
            }
            GovernanceOperation::FixRollout => {
                if actor_roles.contains(&"architect") || actor_roles.contains(&"policy_admin") {
                    AuthorizationOutcome::Allowed
                } else {
                    AuthorizationOutcome::Denied(
                        "actor is missing required role: architect or policy_admin".to_string(),
                    )
                }
            }
            GovernanceOperation::RunAudit => {
                // By default anyone can run an audit report
                AuthorizationOutcome::Allowed
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::override_policy::{OverrideLevel, ResolvedGovernanceRoleBinding};
    use crate::config::policy::NamingRule;

    fn dummy_effective_policy(
        roles: Vec<ResolvedGovernanceRoleBinding>,
    ) -> EffectivePolicy {
        EffectivePolicy {
            module_naming: (
                NamingRule::KebabCase,
                OverrideLevel::Default,
                "default".to_string(),
            ),
            artifact_naming: (
                NamingRule::KebabCase,
                OverrideLevel::Default,
                "default".to_string(),
            ),
            overrides: vec![],
            forbidden_dependencies: vec![],
            required_files: vec![],
            locked_rules: vec![],
            governance_roles: roles,
        }
    }

    #[test]
    fn unmapped_actor_cannot_edit_policy() {
        let policy = dummy_effective_policy(vec![]);
        let outcome =
            RbacEvaluator::authorize_operation(&policy, "alice", GovernanceOperation::EditPolicy);
        assert!(!outcome.is_allowed());
        if let AuthorizationOutcome::Denied(reason) = outcome {
            assert!(reason.contains("policy_admin"));
        }
    }

    #[test]
    fn policy_admin_can_edit_policy_and_approve_overrides() {
        let policy = dummy_effective_policy(vec![ResolvedGovernanceRoleBinding {
            role: "policy_admin".to_string(),
            members: vec!["admin-bob".to_string()],
            source_level: OverrideLevel::Org,
            source_label: "org".to_string(),
        }]);

        let outcome = RbacEvaluator::authorize_operation(
            &policy,
            "admin-bob",
            GovernanceOperation::EditPolicy,
        );
        assert!(outcome.is_allowed());

        let outcome = RbacEvaluator::authorize_operation(
            &policy,
            "admin-bob",
            GovernanceOperation::ApproveOverride,
        );
        assert!(outcome.is_allowed());
    }

    #[test]
    fn architect_can_approve_overrides_but_not_edit_policy() {
        let policy = dummy_effective_policy(vec![ResolvedGovernanceRoleBinding {
            role: "architect".to_string(),
            members: vec!["arch-charlie".to_string()],
            source_level: OverrideLevel::Org,
            source_label: "org".to_string(),
        }]);

        let outcome = RbacEvaluator::authorize_operation(
            &policy,
            "arch-charlie",
            GovernanceOperation::ApproveOverride,
        );
        assert!(outcome.is_allowed());

        let outcome = RbacEvaluator::authorize_operation(
            &policy,
            "arch-charlie",
            GovernanceOperation::EditPolicy,
        );
        assert!(!outcome.is_allowed());
    }
}
