pub mod filesystem;
pub mod git;
pub mod llm;
pub mod output;
pub mod rendering;
pub mod template;

#[allow(unused_imports)]
pub use filesystem::LocalFilesystemAdapter;
#[allow(unused_imports)]
pub use git::GitProcessAdapter;
#[allow(unused_imports)]
pub use llm::NoopLlmAdapter;
pub use output::ConsoleOutputAdapter;
pub use rendering::PlanRendererAdapter;
#[allow(unused_imports)]
pub use template::SimpleTemplateRendererAdapter;
