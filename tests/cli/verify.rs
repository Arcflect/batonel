use assert_cmd::Command;
use std::path::PathBuf;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/valid_project")
}

#[test]
fn test_verify_success() {
    let mut cmd = Command::cargo_bin("batonel").unwrap();
    cmd.current_dir(fixture_dir())
        .arg("verify");

    cmd.assert()
        .success();
}

#[test]
fn test_verify_missing_project() {
    let temp = tempfile::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("batonel").unwrap();
    
    // Running in an empty directory should fail because project.baton.yaml is missing
    cmd.current_dir(temp.path())
        .arg("verify");

    cmd.assert()
        .failure();
}
