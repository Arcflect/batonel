use super::preset::Preset;

pub struct PresetResolver;

impl PresetResolver {
    pub fn resolve(
        explicit_preset_id: Option<&str>,
        explicit_version: Option<&str>,
        project_default_preset: Option<&str>,
    ) -> Option<Preset> {
        let id = explicit_preset_id.or(project_default_preset)?;
        Some(Preset {
            id: id.to_string(),
            version: explicit_version.map(ToString::to_string),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PresetResolver;

    #[test]
    fn resolve_prefers_explicit_id() {
        let preset = PresetResolver::resolve(Some("rust-clean-hexagonal"), Some("1.2.0"), Some("generic-layered"))
            .expect("preset");
        assert_eq!(preset.id, "rust-clean-hexagonal");
        assert_eq!(preset.version.as_deref(), Some("1.2.0"));
    }

    #[test]
    fn resolve_falls_back_to_project_default() {
        let preset = PresetResolver::resolve(None, None, Some("generic-layered")).expect("preset");
        assert_eq!(preset.id, "generic-layered");
        assert!(preset.version.is_none());
    }

    #[test]
    fn resolve_returns_none_when_no_source_is_provided() {
        let preset = PresetResolver::resolve(None, Some("1.0.0"), None);
        assert!(preset.is_none());
    }
}
