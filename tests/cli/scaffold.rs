use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_scaffold_success() {
    let temp = TempDir::new().unwrap();
    
    // Seed project
    let mut init_cmd = Command::cargo_bin("batonel").unwrap();
    init_cmd.current_dir(temp.path())
        .arg("init")
        .arg("--preset")
        .arg("generic-layered")
        .arg("--project-name")
        .arg("test-scaffold");
    init_cmd.assert().success();

    let mut cmd = Command::cargo_bin("batonel").unwrap();
    cmd.current_dir(temp.path())
        .arg("scaffold");

    cmd.assert()
        .success();
}

