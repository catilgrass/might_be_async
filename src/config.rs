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

    let value: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(_) => return "async".to_string(),
    };

    value
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("might_be_async"))
        .and_then(|a| a.get("default_feature_name"))
        .and_then(|v| v.as_str())
        .unwrap_or("async")
        .to_string()
}
