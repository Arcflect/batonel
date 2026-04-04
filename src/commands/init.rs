use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use serde_yaml::{Mapping, Value};

use crate::config::project::SUPPORTED_PROJECT_SCHEMA_VERSION;

#[derive(Debug, Clone, PartialEq, Eq)]
enum InitActionKind {
  Create,
  SkipExisting,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InitAction {
  filename: String,
  content: String,
  kind: InitActionKind,
}

pub fn execute(preset: Option<&str>, project_name: Option<&str>, dry_run: bool) {
  let actions = match plan_init_actions(preset, project_name) {
    Ok(actions) => actions,
    Err(err) => {
      eprintln!("  [!] {}", err);
      std::process::exit(1);
    }
  };

  if let Some(preset_id) = preset {
    println!("Archflow Initialization (preset: {})", preset_id);
  } else {
    println!("Archflow Initialization");
  }
  println!("=======================");

  if dry_run {
    println!("  [i] Dry run mode enabled. No files will be written.");
  }

  let mut generated_count = 0;
  let mut skipped_count = 0;

  for action in actions {
    match action.kind {
      InitActionKind::Create if dry_run => {
        println!("  [plan] create {}", action.filename);
        generated_count += 1;
      }
      InitActionKind::SkipExisting if dry_run => {
        println!("  [plan] skip {} (already exists)", action.filename);
        skipped_count += 1;
      }
      InitActionKind::Create => match fs::write(Path::new(&action.filename), &action.content) {
        Ok(_) => {
          println!("  [+] Generated {}", action.filename);
          generated_count += 1;
        }
        Err(e) => {
          eprintln!("  [!] Failed to generate {}: {}", action.filename, e);
          std::process::exit(1);
        }
      },
      InitActionKind::SkipExisting => {
        println!("  [~] {} already exists, skipping.", action.filename);
        skipped_count += 1;
      }
    }
  }

  println!();
  if dry_run {
    println!(
      "Dry run complete. {} file(s) would be generated, {} file(s) would be skipped.",
      generated_count, skipped_count
    );
    println!("Review the plan above, then run the same command without --dry-run to generate files.");
  } else if generated_count > 0 {
    println!("Initialization complete! Explore your configuration files, then run:");
    println!("  archflow plan");
    println!("  archflow scaffold");
  } else {
    println!("Initialization finished. No new configuration files were generated.");
  }
}

fn plan_init_actions(
  preset: Option<&str>,
  project_name: Option<&str>,
) -> Result<Vec<InitAction>, String> {
  let files = collect_init_files(preset)?;
  let mut actions = Vec::with_capacity(files.len());

  for (filename, original_content) in files {
    let mut content = original_content;
    if filename == "project.arch.yaml" {
      content = ensure_project_arch_metadata(&content, preset)?;
      if let Some(name) = project_name {
        validate_project_name(name)
          .map_err(|err| format!("Invalid --project-name value: {}", err))?;
        content = override_project_name(&content, name).map_err(|err| {
          format!("Failed to override project name in {}: {}", filename, err)
        })?;
      }
    }

    let kind = if Path::new(&filename).exists() {
      InitActionKind::SkipExisting
    } else {
      InitActionKind::Create
    };

    actions.push(InitAction {
      filename,
      content,
      kind,
    });
  }

  Ok(actions)
}

fn collect_init_files(preset: Option<&str>) -> Result<Vec<(String, String)>, String> {
  if let Some(preset_id) = preset {
    collect_preset_files(preset_id)
  } else {
    Ok(default_init_files())
  }
}

fn default_init_files() -> Vec<(String, String)> {
  vec![
    (
      "project.arch.yaml".to_string(),
      format!(r#"archflow:
  schema_version: "{}"

project:
  name: archflow-app
  architecture_style: simple
  language: generic

modules:
  - name: user
    features:
      - create_user
      - user_entity
"#, SUPPORTED_PROJECT_SCHEMA_VERSION),
        ),
        (
            "placement.rules.yaml".to_string(),
            r#"roles:
  usecase:
    path: "src/application/usecases/"
    file_extension: rs
  entity:
    path: "src/domain/entities/"
    file_extension: rs
"#
            .to_string(),
        ),
        (
            "artifacts.plan.yaml".to_string(),
            r#"artifacts:
  - name: create_user
    module: user
    role: usecase
    inputs:
      - CreateUserCommand
    outputs:
      - CreateUserResult

  - name: user
    module: user
    role: entity
    outputs:
      - User
"#
            .to_string(),
        ),
        (
            "contracts.template.yaml".to_string(),
            r#"role_templates:
  usecase:
    responsibilities:
      - "Execute one application use case"
      - "Coordinate domain behavior"
    must_not:
      - "Access infrastructure details directly"
      - "Return transport-specific responses"
    implementation_size: "small"

  entity:
    responsibilities:
      - "Represent a core business concept"
      - "Protect domain invariants"
    must_not:
      - "Depend on transport or persistence details"
    implementation_size: "small"
  "#
        .to_string(),
        ),
    ]
  }

  fn collect_preset_files(preset_id: &str) -> Result<Vec<(String, String)>, String> {
    let preset_dir = match find_preset_dir(preset_id) {
      Some(path) => path,
      None => {
        let available = list_available_presets();
        let hint = if available.is_empty() {
          "No presets are currently available under presets/".to_string()
        } else {
          format!("Available presets: {}", available.join(", "))
        };
        return Err(format!(
          "Preset '{}' was not found. {}",
          preset_id, hint
        ));
      }
    };

    if !preset_dir.exists() {
      let available = list_available_presets();
      let hint = if available.is_empty() {
        "No presets are currently available under presets/".to_string()
      } else {
        format!("Available presets: {}", available.join(", "))
      };
      return Err(format!(
            "Preset '{}' was not found at {}. {}",
        preset_id,
        preset_dir.display(),
        hint
      ));
    }

    let mut files = Vec::new();
    let required = [
      "project.arch.yaml",
      "placement.rules.yaml",
      "contracts.template.yaml",
    ];
    for filename in required {
      let source = preset_dir.join(filename);
      let contents = fs::read_to_string(&source).map_err(|e| {
        format!(
          "Failed to read required preset file {}: {}",
          source.display(),
          e
        )
      })?;
      files.push((filename.to_string(), contents));
    }

    let optional = ["artifacts.plan.yaml"];
    for filename in optional {
      let source = preset_dir.join(filename);
      if source.exists() {
        let contents = fs::read_to_string(&source).map_err(|e| {
          format!(
            "Failed to read optional preset file {}: {}",
            source.display(),
            e
          )
        })?;
        files.push((filename.to_string(), contents));
      }
    }

    Ok(files)
  }

  fn list_available_presets() -> Vec<String> {
    let mut presets = BTreeSet::new();
    for root in preset_roots() {
      if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
          if entry.path().is_dir() {
            if let Some(name) = entry.file_name().to_str() {
              presets.insert(name.to_string());
            }
          }
        }
      }
    }

    presets.into_iter().collect()
  }

  fn find_preset_dir(preset_id: &str) -> Option<PathBuf> {
    for root in preset_roots() {
      let candidate = root.join(preset_id);
      if candidate.is_dir() {
        return Some(candidate);
      }
    }
    None
  }

  fn preset_roots() -> Vec<PathBuf> {
    vec![
      PathBuf::from("presets"),
      PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("presets"),
    ]
  }

fn ensure_project_arch_metadata(contents: &str, preset: Option<&str>) -> Result<String, String> {
  let mut value: Value = serde_yaml::from_str(contents)
    .map_err(|e| format!("invalid YAML in project.arch.yaml: {}", e))?;

  let root = value
    .as_mapping_mut()
    .ok_or_else(|| "root YAML document must be a mapping".to_string())?;

  let archflow_key = Value::String("archflow".to_string());
  if !root.contains_key(&archflow_key) {
    root.insert(archflow_key.clone(), Value::Mapping(Mapping::new()));
  }

  let archflow = root
    .get_mut(&archflow_key)
    .and_then(Value::as_mapping_mut)
    .ok_or_else(|| "archflow must be a mapping".to_string())?;

  archflow.insert(
    Value::String("schema_version".to_string()),
    Value::String(SUPPORTED_PROJECT_SCHEMA_VERSION.to_string()),
  );

  if let Some(preset_id) = preset {
    let mut preset_mapping = Mapping::new();
    preset_mapping.insert(
      Value::String("id".to_string()),
      Value::String(preset_id.to_string()),
    );
    archflow.insert(
      Value::String("preset".to_string()),
      Value::Mapping(preset_mapping),
    );
  }

  serde_yaml::to_string(&value).map_err(|e| format!("failed to serialize updated YAML: {}", e))
}

  fn override_project_name(contents: &str, project_name: &str) -> Result<String, String> {
    let mut value: Value = serde_yaml::from_str(contents)
      .map_err(|e| format!("invalid YAML in project.arch.yaml: {}", e))?;

    let root = value
      .as_mapping_mut()
      .ok_or_else(|| "root YAML document must be a mapping".to_string())?;

    let project_key = Value::String("project".to_string());
    if !root.contains_key(&project_key) {
      root.insert(project_key.clone(), Value::Mapping(Mapping::new()));
    }

    let project = root
      .get_mut(&project_key)
      .and_then(Value::as_mapping_mut)
      .ok_or_else(|| "project must be a mapping".to_string())?;

    project.insert(
      Value::String("name".to_string()),
      Value::String(project_name.to_string()),
    );

    serde_yaml::to_string(&value).map_err(|e| format!("failed to serialize updated YAML: {}", e))
}

fn validate_project_name(project_name: &str) -> Result<(), String> {
  if project_name.trim().is_empty() {
    return Err("project name cannot be empty".to_string());
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::{
    ensure_project_arch_metadata, plan_init_actions, override_project_name, validate_project_name,
    InitActionKind,
  };
  use crate::config::project::SUPPORTED_PROJECT_SCHEMA_VERSION;
  use serde_yaml::Value;
  use std::env;
  use std::fs;
  use tempfile::tempdir;

  struct CurrentDirGuard {
    original: std::path::PathBuf,
  }

  impl CurrentDirGuard {
    fn set_to(path: &std::path::Path) -> Self {
      let original = env::current_dir().expect("current dir should resolve");
      env::set_current_dir(path).expect("current dir should be changed for test");
      Self { original }
    }
  }

  impl Drop for CurrentDirGuard {
    fn drop(&mut self) {
      let _ = env::set_current_dir(&self.original);
    }
  }

  #[test]
  fn override_project_name_updates_existing_name() {
    let input = r#"project:
  name: old-name
  architecture_style: layered
"#;

    let updated = override_project_name(input, "new-name").expect("override should succeed");
    let value: Value = serde_yaml::from_str(&updated).expect("yaml should parse");

    let name = value
      .get("project")
      .and_then(|p| p.get("name"))
      .and_then(Value::as_str)
      .expect("project.name must exist");
    assert_eq!(name, "new-name");
  }

  #[test]
  fn override_project_name_creates_project_block_when_missing() {
    let input = r#"modules:
  - name: user
"#;

    let updated = override_project_name(input, "new-name").expect("override should succeed");
    let value: Value = serde_yaml::from_str(&updated).expect("yaml should parse");

    let name = value
      .get("project")
      .and_then(|p| p.get("name"))
      .and_then(Value::as_str)
      .expect("project.name must exist");
    assert_eq!(name, "new-name");
  }

  #[test]
  fn validate_project_name_rejects_empty_value() {
    let err = validate_project_name("   ").expect_err("empty value should be rejected");
    assert_eq!(err, "project name cannot be empty");
  }

  #[test]
  fn plan_init_actions_preserves_default_file_order_and_marks_existing_files() {
    let temp = tempdir().expect("tempdir should be created");
    let _guard = CurrentDirGuard::set_to(temp.path());
    fs::write("placement.rules.yaml", "existing").expect("existing file should be created");

    let actions = plan_init_actions(None, Some("demo-service")).expect("plan should succeed");
    let filenames: Vec<_> = actions.iter().map(|action| action.filename.as_str()).collect();
    let placement_action = actions
      .iter()
      .find(|action| action.filename == "placement.rules.yaml")
      .expect("placement action should exist");

    assert_eq!(
      filenames,
      vec![
        "project.arch.yaml",
        "placement.rules.yaml",
        "artifacts.plan.yaml",
        "contracts.template.yaml"
      ]
    );
    assert_eq!(actions[0].kind, InitActionKind::Create);
    assert_eq!(placement_action.kind, InitActionKind::SkipExisting);
    assert!(actions[0].content.contains("name: demo-service"));
  }

  #[test]
  fn plan_init_actions_rejects_invalid_project_name_before_writing() {
    let temp = tempdir().expect("tempdir should be created");
    let _guard = CurrentDirGuard::set_to(temp.path());

    let err = plan_init_actions(None, Some("   ")).expect_err("invalid name should fail");
    assert_eq!(err, "Invalid --project-name value: project name cannot be empty");
  }

  #[test]
  fn ensure_project_arch_metadata_injects_schema_version_and_preset_id() {
    let input = r#"project:
  name: sample-app
  architecture_style: layered
  language: generic

modules:
  - name: user
"#;

    let updated = ensure_project_arch_metadata(input, Some("generic-layered"))
      .expect("metadata injection should succeed");
    let value: Value = serde_yaml::from_str(&updated).expect("yaml should parse");

    let schema_version = value
      .get("archflow")
      .and_then(|entry| entry.get("schema_version"))
      .and_then(Value::as_str)
      .expect("schema version should exist");
    let preset_id = value
      .get("archflow")
      .and_then(|entry| entry.get("preset"))
      .and_then(|entry| entry.get("id"))
      .and_then(Value::as_str)
      .expect("preset id should exist");

    assert_eq!(schema_version, SUPPORTED_PROJECT_SCHEMA_VERSION);
    assert_eq!(preset_id, "generic-layered");
  }
}
