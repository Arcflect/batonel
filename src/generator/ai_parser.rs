//! AI Response Parsing Boundary
//! 
//! This module provides strict boundaries for how unstructured or semi-structured
//! text responses from LLMs are parsed into safe, injectable code blocks.

pub struct AiResponseParser;

impl AiResponseParser {
    /// Extracts the core code block from an AI response.
    /// 
    /// Standard AI responses often wrap the target code inside markdown code blocks,
    /// e.g. ```rust ... ```. This function attempts to safely extract the longest
    /// matching block of code. If no code block markers are found, it assumes the 
    /// entire response is the code payload (raw mode).
    pub fn extract_code_block(response_text: &str) -> String {
        let mut blocks = Vec::new();
        let mut in_block = false;
        let mut current_block = String::new();

        for line in response_text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("```") {
                if in_block {
                    // End of block
                    in_block = false;
                    blocks.push(current_block.clone());
                    current_block.clear();
                } else {
                    // Start of block
                    in_block = true;
                }
                continue;
            }

            if in_block {
                current_block.push_str(line);
                current_block.push('\n');
            }
        }

        // If we didn't find any closed markdown blocks, return the whole text, trimmed
        if blocks.is_empty() {
            return response_text.trim().to_string();
        }

        // Return the largest block found (heuristically, this is usually the main artifact code)
        // rather than small snippets of configuration or bash commands.
        blocks.into_iter()
            .max_by_key(|b| b.len())
            .unwrap_or_default()
            .trim_end()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_code_block_from_markdown() {
        let ai_response = "\
Here is your requested code:

```rust
fn example() {
    println!(\"Hello World\");
}
```

Hope this helps!
";
        let extracted = AiResponseParser::extract_code_block(ai_response);
        assert_eq!(extracted, "fn example() {\n    println!(\"Hello World\");\n}");
    }

    #[test]
    fn test_extract_largest_code_block() {
        let ai_response = "\
Here is a small snippet:
```rust
use std::fs;
```
And here is the main implementation:
```rust
fn example() {
    let mut i = 0;
    while i < 10 {
        i += 1;
    }
}
```
";
        let extracted = AiResponseParser::extract_code_block(ai_response);
        assert_eq!(extracted, "fn example() {\n    let mut i = 0;\n    while i < 10 {\n        i += 1;\n    }\n}");
    }

    #[test]
    fn test_fallback_to_raw_text() {
        let ai_response = "fn raw_function() {\n    println!(\"No markdown blocks\");\n}";
        let extracted = AiResponseParser::extract_code_block(ai_response);
        assert_eq!(extracted, "fn raw_function() {\n    println!(\"No markdown blocks\");\n}");
    }
}
