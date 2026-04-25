use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_project")
}

#[test]
fn test_plan_text_format() {
    let mut cmd = Command::cargo_bin("batonel").unwrap();
    cmd.current_dir(fixture_dir())
        .arg("plan")
        .arg("--format")
        .arg("text");

    cmd.assert()
        .success()
        // Text format often outputs "Plan" or the project name
        .stdout(predicate::str::contains("minimal-app"));
}

#[test]
fn test_plan_json_format() {
    let mut cmd = Command::cargo_bin("batonel").unwrap();
    cmd.current_dir(fixture_dir())
        .arg("plan")
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"minimal-app\""));
}
