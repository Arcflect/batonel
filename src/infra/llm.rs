pub struct NoopLlmAdapter;

impl crate::ports::LlmPort for NoopLlmAdapter {
    type Error = String;

    fn complete(
        &self,
        _request: &crate::ports::LlmRequest,
    ) -> Result<crate::ports::LlmResponse, Self::Error> {
        Err("No LLM adapter configured".to_string())
    }
}

pub struct DummyLlmAdapter;

impl crate::ports::LlmPort for DummyLlmAdapter {
    type Error = String;

    fn complete(
        &self,
        request: &crate::ports::LlmRequest,
    ) -> Result<crate::ports::LlmResponse, Self::Error> {
        let preview = if request.prompt.len() > 50 {
            format!("{}...", &request.prompt[..50].replace('\n', " "))
        } else {
            request.prompt.replace('\n', " ")
        };
        
        Ok(crate::ports::LlmResponse {
            content: format!("Mock LLM Response generated for prompt:\n> {}\n\n```rust\n// AI-generated code based on contracts\nfn hello_ai() {{\n    println!(\"Successfully parsed contracts and generated code!\");\n}}\n```\n", preview)
        })
    }
}
