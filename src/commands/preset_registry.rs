use crate::commands::preset_verify;
use crate::config::policy::SUPPORTED_POLICY_PROFILE_VERSION;
use crate::config::project::SUPPORTED_PROJECT_SCHEMA_VERSION;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

const REGISTRY_INDEX_FILENAME: &str = "index.yaml";
const PRESET_MANIFEST_FILENAME: &str = "preset.yaml";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PresetManifest {
    name: String,
    summary: String,
    architecture_style: String,
    ecosystem: String,
    source_example: Option<String>,
    includes: PresetIncludes,
    package: PresetPackageMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PresetIncludes {
    required: Vec<String>,
    #[serde(default)]
    optional: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PresetPackageMetadata {
    version: String,
    #[serde(default = "default_visibility")]
    visibility: String,
    compatibility: PresetCompatibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PresetCompatibility {
    min_batonel_version: String,
    max_batonel_version: String,
    project_schema_version: String,
    policy_profile_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PresetRegistryIndex {
    version: u32,
    #[serde(default)]
    presets: Vec<PresetRegistryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PresetRegistryEntry {
    id: String,
    version: String,
    visibility: String,
    summary: String,
    architecture_style: String,
    ecosystem: String,
    compatibility: PresetCompatibility,
    package_path: String,
}

fn default_visibility() -> String {
    "public".to_string()
}

pub fn publish(preset_dir: &str, registry_dir: &str) {
    let preset_dir_path = PathBuf::from(preset_dir);
    let registry_dir_path = PathBuf::from(registry_dir);

    let manifest = match load_and_validate_manifest_for_publish(&preset_dir_path) {
        Ok(manifest) => manifest,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    // Run contract-first and sidecar-first alignment checks before publishing.
    let alignment_report = preset_verify::run_alignment_check(&preset_dir_path);
    if !alignment_report.findings.is_empty() {
        preset_verify::render_report(&alignment_report);
        if alignment_report.has_errors() {
            eprintln!(
                "[!] preset alignment check failed; resolve errors before publishing"
            );
            std::process::exit(1);
        }
        println!();
    }

    if let Err(err) = fs::create_dir_all(&registry_dir_path) {
        eprintln!(
            "[!] failed to create registry directory '{}': {}",
            registry_dir_path.display(),
            err
        );
        std::process::exit(1);
    }

    let mut index = match load_registry_index(&registry_dir_path) {
        Ok(index) => index,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    if index
        .presets
        .iter()
        .any(|entry| entry.id == manifest.name && entry.version == manifest.package.version)
    {
        eprintln!(
            "[!] preset '{}' version '{}' is already published",
            manifest.name, manifest.package.version
        );
        std::process::exit(1);
    }

    let package_rel_path = format!("packages/{}/{}", manifest.name, manifest.package.version);
    let package_dst_path = registry_dir_path.join(&package_rel_path);
    if package_dst_path.exists() {
        eprintln!(
            "[!] package destination already exists: {}",
            package_dst_path.display()
        );
        std::process::exit(1);
    }

    if let Err(err) = copy_directory_recursive(&preset_dir_path, &package_dst_path) {
        eprintln!("[!] failed to copy preset package: {}", err);
        std::process::exit(1);
    }

    index.presets.push(PresetRegistryEntry {
        id: manifest.name.clone(),
        version: manifest.package.version.clone(),
        visibility: manifest.package.visibility.clone(),
        summary: manifest.summary.clone(),
        architecture_style: manifest.architecture_style.clone(),
        ecosystem: manifest.ecosystem.clone(),
        compatibility: manifest.package.compatibility.clone(),
        package_path: package_rel_path,
    });

    index.presets.sort_by(|left, right| match left.id.cmp(&right.id) {
        Ordering::Equal => compare_semver(&right.version, &left.version),
        other => other,
    });

    if let Err(err) = write_registry_index(&registry_dir_path, &index) {
        eprintln!("[!] {}", err);
        std::process::exit(1);
    }

    println!("Preset published successfully");
    println!("  id: {}", manifest.name);
    println!("  version: {}", manifest.package.version);
    println!("  registry: {}", registry_dir_path.display());
    println!("  package_path: {}", package_dst_path.display());
}

pub fn install(preset_id: &str, version: Option<&str>, registry_dir: &str, destination_dir: &str) {
    let registry_dir_path = PathBuf::from(registry_dir);
    let destination_root = PathBuf::from(destination_dir);

    let index = match load_registry_index(&registry_dir_path) {
        Ok(index) => index,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    let selected = match select_registry_entry(&index, preset_id, version) {
        Ok(entry) => entry,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = validate_install_compatibility(selected) {
        eprintln!("[!] {}", err);
        std::process::exit(1);
    }

    let source_package_dir = registry_dir_path.join(&selected.package_path);
    if !source_package_dir.exists() {
        eprintln!(
            "[!] registry entry points to missing package path: {}",
            source_package_dir.display()
        );
        std::process::exit(1);
    }

    let destination_preset_dir = destination_root.join(&selected.id);
    if destination_preset_dir.exists() {
        eprintln!(
            "[!] destination preset directory already exists: {}",
            destination_preset_dir.display()
        );
        std::process::exit(1);
    }

    if let Err(err) = copy_directory_recursive(&source_package_dir, &destination_preset_dir) {
        eprintln!("[!] failed to install preset package: {}", err);
        std::process::exit(1);
    }

    println!("Preset installed successfully");
    println!("  id: {}", selected.id);
    println!("  version: {}", selected.version);
    println!("  source: {}", source_package_dir.display());
    println!("  destination: {}", destination_preset_dir.display());
}

fn load_and_validate_manifest_for_publish(preset_dir: &Path) -> Result<PresetManifest, String> {
    if !preset_dir.is_dir() {
        return Err(format!(
            "preset directory does not exist: {}",
            preset_dir.display()
        ));
    }

    let manifest_path = preset_dir.join(PRESET_MANIFEST_FILENAME);
    let manifest = load_manifest(&manifest_path)?;

    validate_manifest_identity(preset_dir, &manifest)?;
    validate_manifest_compatibility(&manifest)?;
    validate_manifest_file_includes(preset_dir, &manifest)?;

    Ok(manifest)
}

fn load_manifest(path: &Path) -> Result<PresetManifest, String> {
    let contents = fs::read_to_string(path)
        .map_err(|err| format!("failed to read manifest '{}': {}", path.display(), err))?;

    serde_yaml::from_str::<PresetManifest>(&contents)
        .map_err(|err| format!("failed to parse manifest '{}': {}", path.display(), err))
}

fn validate_manifest_identity(preset_dir: &Path, manifest: &PresetManifest) -> Result<(), String> {
    if !is_kebab_case(&manifest.name) {
        return Err(format!(
            "preset name '{}' must use lowercase kebab-case",
            manifest.name
        ));
    }

    let dir_name = preset_dir
        .file_name()
        .and_then(|entry| entry.to_str())
        .ok_or_else(|| "invalid preset directory name".to_string())?;
    if manifest.name != dir_name {
        return Err(format!(
            "preset name '{}' must match directory name '{}'",
            manifest.name, dir_name
        ));
    }

    if !is_semver(&manifest.package.version) {
        return Err(format!(
            "preset package.version '{}' must follow semver format x.y.z",
            manifest.package.version
        ));
    }

    if !matches!(manifest.package.visibility.as_str(), "public" | "private") {
        return Err(format!(
            "preset package.visibility '{}' must be 'public' or 'private'",
            manifest.package.visibility
        ));
    }

    Ok(())
}

fn validate_manifest_compatibility(manifest: &PresetManifest) -> Result<(), String> {
    let compatibility = &manifest.package.compatibility;
    if !is_semver(&compatibility.min_batonel_version) {
        return Err(format!(
            "compatibility.min_batonel_version '{}' must be semver x.y.z",
            compatibility.min_batonel_version
        ));
    }
    if !is_semver(&compatibility.max_batonel_version) {
        return Err(format!(
            "compatibility.max_batonel_version '{}' must be semver x.y.z",
            compatibility.max_batonel_version
        ));
    }
    if compare_semver(&compatibility.min_batonel_version, &compatibility.max_batonel_version)
        == Ordering::Greater
    {
        return Err("compatibility.min_batonel_version cannot be greater than max_batonel_version"
            .to_string());
    }

    if compatibility.project_schema_version.trim().is_empty() {
        return Err("compatibility.project_schema_version cannot be empty".to_string());
    }

    Ok(())
}

fn validate_manifest_file_includes(preset_dir: &Path, manifest: &PresetManifest) -> Result<(), String> {
    let hard_required = [
        "project.baton.yaml",
        "placement.rules.yaml",
        "contracts.template.yaml",
    ];

    for required in hard_required {
        if !manifest.includes.required.iter().any(|entry| entry == required) {
            return Err(format!(
                "preset manifest must list '{}' under includes.required",
                required
            ));
        }
        let path = preset_dir.join(required);
        if !path.exists() {
            return Err(format!(
                "required preset file listed in includes.required is missing: {}",
                path.display()
            ));
        }
    }

    for listed in manifest
        .includes
        .required
        .iter()
        .chain(manifest.includes.optional.iter())
    {
        if !preset_dir.join(listed).exists() {
            return Err(format!(
                "preset include '{}' is listed but file does not exist in {}",
                listed,
                preset_dir.display()
            ));
        }
    }

    Ok(())
}

fn load_registry_index(registry_dir: &Path) -> Result<PresetRegistryIndex, String> {
    let index_path = registry_dir.join(REGISTRY_INDEX_FILENAME);
    if !index_path.exists() {
        return Ok(PresetRegistryIndex {
            version: 1,
            presets: vec![],
        });
    }

    let contents = fs::read_to_string(&index_path)
        .map_err(|err| format!("failed to read registry index '{}': {}", index_path.display(), err))?;
    let index: PresetRegistryIndex = serde_yaml::from_str(&contents)
        .map_err(|err| format!("failed to parse registry index '{}': {}", index_path.display(), err))?;

    if index.version != 1 {
        return Err(format!(
            "unsupported registry index version '{}', expected '1'",
            index.version
        ));
    }

    Ok(index)
}

fn write_registry_index(registry_dir: &Path, index: &PresetRegistryIndex) -> Result<(), String> {
    let index_path = registry_dir.join(REGISTRY_INDEX_FILENAME);
    let serialized = serde_yaml::to_string(index)
        .map_err(|err| format!("failed to serialize registry index: {}", err))?;
    fs::write(&index_path, serialized)
        .map_err(|err| format!("failed to write registry index '{}': {}", index_path.display(), err))
}

fn select_registry_entry<'a>(
    index: &'a PresetRegistryIndex,
    preset_id: &str,
    version: Option<&str>,
) -> Result<&'a PresetRegistryEntry, String> {
    let candidates: Vec<&PresetRegistryEntry> = index
        .presets
        .iter()
        .filter(|entry| entry.id == preset_id)
        .collect();

    if candidates.is_empty() {
        return Err(format!("preset '{}' was not found in registry index", preset_id));
    }

    if let Some(target_version) = version {
        if !is_semver(target_version) {
            return Err(format!(
                "requested version '{}' must follow semver x.y.z",
                target_version
            ));
        }
        return candidates
            .into_iter()
            .find(|entry| entry.version == target_version)
            .ok_or_else(|| {
                format!(
                    "preset '{}' version '{}' was not found in registry index",
                    preset_id, target_version
                )
            });
    }

    candidates
        .into_iter()
        .max_by(|left, right| compare_semver(&left.version, &right.version))
        .ok_or_else(|| format!("failed to select preset '{}'", preset_id))
}

fn validate_install_compatibility(entry: &PresetRegistryEntry) -> Result<(), String> {
    let current_batonel_version = env!("CARGO_PKG_VERSION");
    if compare_semver(current_batonel_version, &entry.compatibility.min_batonel_version)
        == Ordering::Less
        || compare_semver(current_batonel_version, &entry.compatibility.max_batonel_version)
            == Ordering::Greater
    {
        return Err(format!(
            "preset '{}' version '{}' is incompatible with batonel '{}'; supported range: {} - {}",
            entry.id,
            entry.version,
            current_batonel_version,
            entry.compatibility.min_batonel_version,
            entry.compatibility.max_batonel_version
        ));
    }

    if entry.compatibility.project_schema_version != SUPPORTED_PROJECT_SCHEMA_VERSION {
        return Err(format!(
            "preset '{}' version '{}' requires project schema version '{}' but current is '{}'",
            entry.id,
            entry.version,
            entry.compatibility.project_schema_version,
            SUPPORTED_PROJECT_SCHEMA_VERSION
        ));
    }

    if entry.compatibility.policy_profile_version != SUPPORTED_POLICY_PROFILE_VERSION {
        return Err(format!(
            "preset '{}' version '{}' requires policy profile version '{}' but current is '{}'",
            entry.id,
            entry.version,
            entry.compatibility.policy_profile_version,
            SUPPORTED_POLICY_PROFILE_VERSION
        ));
    }

    Ok(())
}

fn copy_directory_recursive(source: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination).map_err(|err| {
        format!(
            "failed to create destination directory '{}': {}",
            destination.display(),
            err
        )
    })?;

    for entry in fs::read_dir(source)
        .map_err(|err| format!("failed to read directory '{}': {}", source.display(), err))?
    {
        let entry = entry.map_err(|err| format!("failed to read directory entry: {}", err))?;
        let file_type = entry
            .file_type()
            .map_err(|err| format!("failed to read file type: {}", err))?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if file_type.is_dir() {
            copy_directory_recursive(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            fs::copy(&source_path, &destination_path).map_err(|err| {
                format!(
                    "failed to copy '{}' to '{}': {}",
                    source_path.display(),
                    destination_path.display(),
                    err
                )
            })?;
        }
    }

    Ok(())
}

fn is_kebab_case(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
}

fn is_semver(value: &str) -> bool {
    let mut parts = value.split('.');
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(major), Some(minor), Some(patch), None) => {
            major.chars().all(|ch| ch.is_ascii_digit())
                && minor.chars().all(|ch| ch.is_ascii_digit())
                && patch.chars().all(|ch| ch.is_ascii_digit())
        }
        _ => false,
    }
}

fn compare_semver(left: &str, right: &str) -> Ordering {
    let left_parts = parse_semver(left).unwrap_or((0, 0, 0));
    let right_parts = parse_semver(right).unwrap_or((0, 0, 0));
    left_parts.cmp(&right_parts)
}

fn parse_semver(value: &str) -> Option<(u64, u64, u64)> {
    if !is_semver(value) {
        return None;
    }

    let mut parts = value.split('.');
    let major = parts.next()?.parse::<u64>().ok()?;
    let minor = parts.next()?.parse::<u64>().ok()?;
    let patch = parts.next()?.parse::<u64>().ok()?;
    Some((major, minor, patch))
}

#[cfg(test)]
mod tests {
    use super::{
        compare_semver, is_semver, select_registry_entry, PresetCompatibility, PresetRegistryEntry,
        PresetRegistryIndex,
    };
    use std::cmp::Ordering;

    #[test]
    fn semver_validation_accepts_basic_triplet() {
        assert!(is_semver("1.2.3"));
        assert!(!is_semver("1.2"));
        assert!(!is_semver("v1.2.3"));
    }

    #[test]
    fn semver_comparison_orders_versions() {
        assert_eq!(compare_semver("1.2.4", "1.2.3"), Ordering::Greater);
        assert_eq!(compare_semver("1.2.3", "1.2.3"), Ordering::Equal);
        assert_eq!(compare_semver("1.2.3", "2.0.0"), Ordering::Less);
    }

    #[test]
    fn select_registry_entry_uses_latest_when_version_not_specified() {
        let entry_v1 = PresetRegistryEntry {
            id: "generic-layered".to_string(),
            version: "0.1.0".to_string(),
            visibility: "public".to_string(),
            summary: "preset".to_string(),
            architecture_style: "layered".to_string(),
            ecosystem: "generic".to_string(),
            compatibility: PresetCompatibility {
                min_batonel_version: "0.1.0".to_string(),
                max_batonel_version: "0.1.9".to_string(),
                project_schema_version: "1".to_string(),
                policy_profile_version: 1,
            },
            package_path: "packages/generic-layered/0.1.0".to_string(),
        };
        let entry_v2 = PresetRegistryEntry {
            version: "0.2.0".to_string(),
            package_path: "packages/generic-layered/0.2.0".to_string(),
            ..entry_v1.clone()
        };

        let index = PresetRegistryIndex {
            version: 1,
            presets: vec![entry_v1, entry_v2],
        };

        let selected = select_registry_entry(&index, "generic-layered", None)
            .expect("latest preset should be selected");
        assert_eq!(selected.version, "0.2.0");
    }
}
