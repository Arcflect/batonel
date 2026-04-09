#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PlannedArtifactStatus {
    Planned,
    Error,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlannedArtifact {
    pub name: String,
    pub role: String,
    pub status: PlannedArtifactStatus,
    pub resolved_path: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArchitecturePlan {
    pub project_name: String,
    pub architecture_style: String,
    pub language: String,
    pub module_count: usize,
    pub artifacts: Vec<PlannedArtifact>,
}

impl ArchitecturePlan {
    pub fn planned_count(&self) -> usize {
        self.artifacts
            .iter()
            .filter(|a| a.status == PlannedArtifactStatus::Planned)
            .count()
    }

    pub fn error_count(&self) -> usize {
        self.artifacts
            .iter()
            .filter(|a| a.status == PlannedArtifactStatus::Error)
            .count()
    }
}
