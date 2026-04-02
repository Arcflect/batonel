use std::fs;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use serde_yaml::{Mapping, Value};

pub fn execute(preset: Option<&str>, project_name: Option<&str>) {
  let files = match collect_init_files(preset) {
    Ok(files) => files,
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

  let mut generated_count = 0;

  for (filename, original_content) in files {
    let mut content = original_content;
    if filename == "project.arch.yaml" {
      if let Some(name) = project_name {
        if let Err(err) = validate_project_name(name) {
          eprintln!("  [!] Invalid --project-name value: {}", err);
          std::process::exit(1);
        }
        match override_project_name(&content, name) {
          Ok(updated) => content = updated,
          Err(err) => {
            eprintln!("  [!] Failed to override project name in {}: {}", filename, err);
            std::process::exit(1);
          }
        }
      }
    }

    let path = Path::new(&filename);
    if path.exists() {
      println!("  [~] {} already exists, skipping.", filename);
    } else {
      match fs::write(path, &content) {
        Ok(_) => {
          println!("  [+] Generated {}", filename);
          generated_count += 1;
        }
        Err(e) => {
          eprintln!("  [!] Failed to generate {}: {}", filename, e);
          std::process::exit(1);
        }
      }
    }
  }

  println!();
  if generated_count > 0 {
    println!("Initialization complete! Explore your configuration files, then run:");
    println!("  archflow plan");
    println!("  archflow scaffold");
  } else {
    println!("Initialization finished. No new configuration files were generated.");
  }
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
      r#"project:
  name: archflow-app
  architecture_style: simple
  language: generic

modules:
  - name: user
    features:
      - create_user
      - user_entity
"#
            .to_string(),
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
  use super::{override_project_name, validate_project_name};
  use serde_yaml::Value;

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
}
