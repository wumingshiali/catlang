//! Parrot Package Manager
//!
//! Manages CatLang project dependencies.
//! - Folder projects: reads Parrot.toml from project root
//! - Single-file projects: searches global config directory

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Parrot.toml configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParrotConfig {
    pub package: PackageInfo,
    #[serde(default)]
    pub dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "dev-dependencies")]
    pub dev_dependencies: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
}

/// Resolved dependency
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
}

/// Find Parrot.toml for a given source file
///
/// - If source is inside a directory with Parrot.toml, use that
/// - Otherwise search global config directory
pub fn find_config(source_path: &Path) -> Result<PathBuf, ParrotError> {
    // Try folder project: search upward from source file
    if let Some(parent) = source_path.parent() {
        if let Ok(config) = search_upward(parent) {
            return Ok(config);
        }
    }

    // Fall back to global config
    global_config_path()
}

/// Search upward for Parrot.toml
fn search_upward(start: &Path) -> Result<PathBuf, ParrotError> {
    let mut current = start.canonicalize().map_err(|e| ParrotError::Io(e.to_string()))?;

    loop {
        let config = current.join("Parrot.toml");
        if config.exists() {
            return Ok(config);
        }

        if !current.pop() {
            break;
        }
    }

    Err(ParrotError::NotFound("Parrot.toml not found in project tree".into()))
}

/// Get global config path
///
/// Linux/macOS: ~/.config/parrot/Parrot.toml
/// Windows: %APPDATA%\parrot\Parrot.toml
fn global_config_path() -> Result<PathBuf, ParrotError> {
    let config_dir = if cfg!(windows) {
        std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("parrot")
    } else {
        std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                dirs_home()
                    .map(|p| p.join(".config"))
                    .unwrap_or_else(|| PathBuf::from("."))
            })
            .join("parrot")
    };

    let config = config_dir.join("Parrot.toml");
    if config.exists() {
        Ok(config)
    } else {
        Err(ParrotError::NotFound("Global Parrot.toml not found".into()))
    }
}

fn dirs_home() -> Option<PathBuf> {
    if cfg!(windows) {
        std::env::var("USERPROFILE").ok().map(PathBuf::from)
    } else {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}

/// Parse Parrot.toml
pub fn parse_config(path: &Path) -> Result<ParrotConfig, ParrotError> {
    let content = fs::read_to_string(path)
        .map_err(|e| ParrotError::Io(e.to_string()))?;

    let config: ParrotConfig = toml::from_str(&content)
        .map_err(|e| ParrotError::Parse(e.to_string()))?;

    Ok(config)
}

/// Get global packages directory
///
/// Linux/macOS: ~/.local/share/parrot/packages
/// Windows: %APPDATA%\parrot\packages
pub fn global_packages_dir() -> PathBuf {
    if cfg!(windows) {
        std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("parrot")
            .join("packages")
    } else {
        std::env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                dirs_home()
                    .map(|p| p.join(".local").join("share"))
                    .unwrap_or_else(|| PathBuf::from("."))
            })
            .join("parrot")
            .join("packages")
    }
}

/// Resolve dependencies to their paths (all packages stored globally)
pub fn resolve_dependencies(config: &ParrotConfig, _config_path: &Path) -> Vec<Dependency> {
    let global_dir = global_packages_dir();

    config
        .dependencies
        .iter()
        .filter_map(|(name, version)| {
            let global_path = global_dir.join(name);

            if global_path.exists() {
                Some(Dependency {
                    name: name.clone(),
                    version: version.clone(),
                    path: global_path,
                })
            } else {
                None
            }
        })
        .collect()
}

/// Generate import paths for codegen
pub fn generate_import_paths(config: &ParrotConfig, config_path: &Path) -> Vec<(String, PathBuf)> {
    resolve_dependencies(config, config_path)
        .into_iter()
        .map(|dep| (dep.name, dep.path))
        .collect()
}

#[derive(Debug, Clone)]
pub enum ParrotError {
    Io(String),
    Parse(String),
    NotFound(String),
}

impl std::fmt::Display for ParrotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParrotError::Io(msg) => write!(f, "IO error: {}", msg),
            ParrotError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ParrotError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for ParrotError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn create_temp_config(content: &str) -> (PathBuf, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let config_path = dir.path().join("Parrot.toml");
        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        (config_path, dir)
    }

    #[test]
    fn test_parse_valid_config() {
        let content = r#"
[package]
name = "test_project"
version = "0.1.0"

[dependencies]
utils = "1.0.0"
"#;
        let (_config_path, _dir) = create_temp_config(content);
        // Config parsing tested via toml directly
        let config: ParrotConfig = toml::from_str(content).unwrap();
        assert_eq!(config.package.name, "test_project");
        assert_eq!(config.package.version, "0.1.0");
        assert!(config.dependencies.contains_key("utils"));
    }

    #[test]
    fn test_parse_config_with_dev_dependencies() {
        let content = r#"
[package]
name = "my_app"
version = "1.0.0"

[dependencies]
http = "0.5.0"

[dev-dependencies]
test_framework = "1.0.0"
"#;
        let config: ParrotConfig = toml::from_str(content).unwrap();
        assert_eq!(config.dependencies.len(), 1);
        assert_eq!(config.dev_dependencies.len(), 1);
    }

    #[test]
    fn test_parse_config_no_dependencies() {
        let content = r#"
[package]
name = "standalone"
version = "0.0.1"
"#;
        let config: ParrotConfig = toml::from_str(content).unwrap();
        assert!(config.dependencies.is_empty());
    }

    #[test]
    fn test_invalid_config_missing_package() {
        let content = r#"
[dependencies]
utils = "1.0.0"
"#;
        let result: Result<ParrotConfig, _> = toml::from_str(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_global_packages_dir() {
        let dir = global_packages_dir();
        assert!(dir.ends_with("parrot/packages") || dir.ends_with("parrot\\packages"));
    }
}
