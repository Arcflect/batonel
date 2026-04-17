use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub project_name: String,
    pub architecture_style: String,
    pub language: String,
    module_names: BTreeSet<String>,
}

impl ProjectContext {
    pub fn new(
        project_name: String,
        architecture_style: String,
        language: String,
        module_names: BTreeSet<String>,
    ) -> Self {
        Self {
            project_name,
            architecture_style,
            language,
            module_names,
        }
    }

    pub fn from_project_config(config: &crate::config::ProjectConfig) -> Self {
        let module_names = config.modules.iter().map(|m| m.name.clone()).collect();
        Self::new(
            config.project.name.clone(),
            config.project.architecture_style.clone(),
            config.project.language.clone(),
            module_names,
        )
    }

    pub fn has_module(&self, module_name: &str) -> bool {
        self.module_names.contains(module_name)
    }

    pub fn module_count(&self) -> usize {
        self.module_names.len()
    }
}

#[cfg(test)]
mod tests {
    use super::ProjectContext;
    use crate::config::ProjectConfig;
    use crate::model::project::{Module, Project};

    fn sample_project() -> ProjectConfig {
        ProjectConfig {
            batonel: Some(crate::config::project::BatonelMetadata {
                schema_version: crate::config::project::SUPPORTED_PROJECT_SCHEMA_VERSION.to_string(),
                preset: None,
            }),
            project: Project {
                name: "demo".to_string(),
                architecture_style: "layered".to_string(),
                language: "rust".to_string(),
            },
            workspace: None,
            modules: vec![
                Module {
                    name: "user".to_string(),
                    features: None,
                },
                Module {
                    name: "order".to_string(),
                    features: None,
                },
            ],
            metadata: None,
        }
    }

    #[test]
    fn context_knows_modules() {
        let ctx = ProjectContext::from_project_config(&sample_project());
        assert!(ctx.has_module("user"));
        assert!(!ctx.has_module("billing"));
        assert_eq!(ctx.module_count(), 2);
    }
}
