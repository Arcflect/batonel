#[derive(Debug, Clone, Default)]
pub struct GenerateArtifactsInput;

#[derive(Debug, Clone)]
pub struct GenerateArtifactsOutput {
    pub success: bool,
}

pub struct GenerateArtifactsUseCase;

impl GenerateArtifactsUseCase {
    pub fn execute(
        _input: GenerateArtifactsInput,
    ) -> Result<GenerateArtifactsOutput, crate::app::error::GenerationError> {
        crate::commands::scaffold::execute();
        Ok(GenerateArtifactsOutput { success: true })
    }
}
