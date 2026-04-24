//! Generic CLI execution pipeline helpers.
//!
//! These utilities contain no product-specific wording. They standardise the
//! three plumbing concerns that every command arm must handle:
//!
//! 1. Running a usecase and exiting cleanly on error.
//! 2. Printing a consistent command-start banner.
//! 3. Writing rendered output through the output port.

use std::fmt::Display;

// ---------------------------------------------------------------------------
// Public helpers
// ---------------------------------------------------------------------------

/// Execute `f` and return the output. On `Err`, print `"[!] {label}: {err}"` to
/// stderr and terminate with exit code 1.
///
/// `label` should be a short, lowercase description of what was attempted (e.g.
/// `"init project"`, `"plan architecture"`). It **must not** contain
/// product-specific output phrasing — that belongs in the caller.
pub fn run_usecase<T, E, F>(label: &str, f: F) -> T
where
    E: Display,
    F: FnOnce() -> Result<T, E>,
{
    match f() {
        Ok(output) => output,
        Err(err) => {
            eprintln!("[!] {}: {}", label, err);
            std::process::exit(1);
        }
    }
}

/// Print a two-line command banner to stdout: the title followed by a `"="`
/// separator of the same width.
///
/// Example output:
/// ```text
/// Batonel Initialization
/// ======================
/// ```
///
/// Only call this for commands where a start banner improves readability. Do
/// not call it for streaming or machine-readable output commands.
pub fn print_command_header(title: &str) {
    println!("{}", title);
    println!("{}", "=".repeat(title.len()));
}

/// If `success` is `false`, print `"[!] {label} failed"` to stderr and
/// terminate with exit code 1.
///
/// Used as the final gate for usecase-backed commands after all output has
/// been rendered.
pub fn exit_on_failure(success: bool, label: &str) {
    if !success {
        eprintln!("[!] {} failed", label);
        std::process::exit(1);
    }
}

/// Write each line of `output` to stdout via the `OutputPort`.
///
/// This is the standard way to emit pre-rendered multi-line output (e.g. from
/// `PlanRendererAdapter`) without coupling the rendering layer to `println!`
/// directly.
pub fn write_output(output: &str) {
    let mut adapter = crate::infra::ConsoleOutputAdapter;
    for line in output.lines() {
        crate::ports::OutputPort::write_line(
            &mut adapter,
            crate::ports::OutputLevel::Info,
            line,
        );
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{exit_on_failure, run_usecase};

    #[test]
    fn run_usecase_returns_value_on_ok() {
        let result: u32 = run_usecase("demo", || Ok::<u32, String>(42));
        assert_eq!(result, 42);
    }

    #[test]
    fn run_usecase_propagates_inner_value() {
        let result: Vec<u8> = run_usecase("demo", || Ok::<Vec<u8>, String>(vec![1, 2, 3]));
        assert_eq!(result, vec![1, 2, 3]);
    }

    // exit_on_failure with success=true is a no-op — just verify it doesn't panic.
    #[test]
    fn exit_on_failure_does_not_panic_on_success() {
        exit_on_failure(true, "test context");
    }
}
