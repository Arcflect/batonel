use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_init_dry_run() {
    let temp = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("batonel").unwrap();
    cmd.current_dir(temp.path())
        .arg("init")
        .arg("--preset")
        .arg("generic-layered")
        .arg("--project-name")
        .arg("test-project")
        .arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("create project.baton.yaml"));
        
    // Since it's a dry run, the project file shouldn't be created
    assert!(!temp.path().join("project.baton.yaml").exists());
}

#[test]
fn test_init_success() {
    let temp = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("batonel").unwrap();
    cmd.current_dir(temp.path())
        .arg("init")
        .arg("--preset")
        .arg("generic-layered")
        .arg("--project-name")
        .arg("test-project");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated project.baton.yaml"));

    // Ensure the basic project file is created
    let project_yaml = temp.path().join("project.baton.yaml");
    assert!(project_yaml.exists());

    // Verify the project name was injected correctly
    let content = fs::read_to_string(&project_yaml).unwrap();
    assert!(content.contains("test-project"));
}

