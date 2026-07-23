use std::path::Path;
use std::sync::OnceLock;

/// Returns the default feature name configured in the consuming crate's
/// `Cargo.toml` under `[package.metadata.might_be_async.default_feature_name]`.
///
/// If the metadata key is absent or unreadable, falls back to `"async"`.
pub(crate) fn default_feature_name() -> &'static str {
    static DEFAULT: OnceLock<String> = OnceLock::new();
    DEFAULT.get_or_init(read_default_feature_name)
}

fn read_default_feature_name() -> String {
    let manifest_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(dir) => dir,
        Err(_) => return "async".to_string(),
    };

    let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");
    let content = match std::fs::read_to_string(cargo_toml_path) {
        Ok(c) => c,
        Err(_) => return "async".to_string(),
    };

    parse_feature_name_from_toml(&content).unwrap_or_else(|| "async".to_string())
}

/// Pure function: parse `default_feature_name` from TOML content.
/// Returns `None` if the key is missing, unreadable, or not a string.
fn parse_feature_name_from_toml(content: &str) -> Option<String> {
    let value: toml::Value = content.parse().ok()?;
    value
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("might_be_async"))
        .and_then(|a| a.get("default_feature_name"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_custom_feature_name() {
        let toml = r#"
            [package]
            [package.metadata.might_be_async]
            default_feature_name = "my_async"
        "#;
        assert_eq!(
            parse_feature_name_from_toml(toml),
            Some("my_async".to_string())
        );
    }

    #[test]
    fn missing_metadata_returns_none() {
        let toml = "[package]\nname = \"foo\"\n";
        assert_eq!(parse_feature_name_from_toml(toml), None);
    }

    #[test]
    fn missing_might_be_async_section_returns_none() {
        let toml = r#"
            [package]
            [package.metadata]
            some_other_key = "value"
        "#;
        assert_eq!(parse_feature_name_from_toml(toml), None);
    }

    #[test]
    fn empty_string_returns_none() {
        assert_eq!(parse_feature_name_from_toml(""), None);
    }

    #[test]
    fn invalid_toml_returns_none() {
        assert_eq!(parse_feature_name_from_toml("not valid toml {{"), None);
    }

    #[test]
    fn value_is_not_string_returns_none() {
        let toml = r#"
            [package.metadata.might_be_async]
            default_feature_name = 42
        "#;
        assert_eq!(parse_feature_name_from_toml(toml), None);
    }

    #[test]
    fn env_default_returns_foo_async() {
        // In `cargo test`, CARGO_MANIFEST_DIR is set to this crate's directory,
        // whose Cargo.toml has `default_feature_name = "foo_async"`.
        assert_eq!(default_feature_name(), "foo_async");
    }
}
