//! Parrot Package Manager
//!
//! Manages CatLang project dependencies.
//! - Folder projects: reads Parrot.toml from project root
//! - Single-file projects: searches global config directory

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Parrot.toml configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParrotConfig {
    pub package: PackageInfo,
    #[serde(default)]
    pub dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "dev-dependencies")]
    pub dev_dependencies: std::collections::HashMap<String, String>,
    #[serde(default, rename = "SandboxPath")]
    pub sandbox_path: Option<String>,
    #[serde(default, rename = "SandboxFlag")]
    pub sandbox_flag: Option<bool>,
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
    Network(String),
}

impl std::fmt::Display for ParrotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParrotError::Io(msg) => write!(f, "IO error: {}", msg),
            ParrotError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ParrotError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ParrotError::Network(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for ParrotError {}

/// Package registry URLs
const GITHUB_REGISTRY: &str = "https://github.com/wumingshiali/catlang-pkg";
const GITHUB_ARCHIVE_BASE: &str = "https://github.com/wumingshiali/catlang-pkg/archive/refs/heads";
const MIRROR_ARCHIVE_BASE: &str = "https://cdn.jsdmirror.com/gh/wumingshiali/catlang-pkg/archive/refs/heads";
const SLOW_THRESHOLD_MS: u64 = 3000;
const BRANCH_NAME: &str = "main";

/// Check if GitHub connection is slow
pub fn is_github_slow() -> bool {
    let start = Instant::now();
    let result = ureq::get(GITHUB_REGISTRY)
        .timeout(Duration::from_millis(SLOW_THRESHOLD_MS))
        .call();

    let elapsed = start.elapsed();
    eprintln!("[parrot] GitHub connection check: {}ms", elapsed.as_millis());

    match result {
        Ok(_) => elapsed.as_millis() > SLOW_THRESHOLD_MS as u128,
        Err(_) => true,
    }
}

/// Get the archive download URL for a package
fn get_archive_url_for_mirror(package_name: &str, use_mirror: bool) -> String {
    let base = if use_mirror {
        MIRROR_ARCHIVE_BASE
    } else {
        GITHUB_ARCHIVE_BASE
    };
    format!("{}/{}/{}.tar.gz", base, BRANCH_NAME, package_name)
}

/// Get the archive download URL for a package
fn get_archive_url(package_name: &str) -> String {
    get_archive_url_for_mirror(package_name, is_github_slow())
}

/// Fetch and install a package from the registry
///
/// Package name format: author/name
/// Downloads from GitHub registry, falls back to mirror if slow
pub fn fetch_package(package_name: &str, dest_dir: &Path) -> Result<(), ParrotError> {
    let archive_url = get_archive_url(package_name);
    eprintln!("[parrot] Fetching {} from {}", package_name, archive_url);

    let response = ureq::get(&archive_url)
        .timeout(Duration::from_secs(60))
        .call()
        .map_err(|e| ParrotError::Network(format!("Failed to download {}: {}", package_name, e)))?;

    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| ParrotError::Io(e.to_string()))?;

    let tar_gz = flate2::read::GzDecoder::new(io::Cursor::new(bytes));
    let mut archive = tar::Archive::new(tar_gz);

    let prefix = format!("catlang-pkg-{}", BRANCH_NAME);
    let package_path = format!("{}/{}", prefix, package_name);

    let entries = archive
        .entries()
        .map_err(|e| ParrotError::Io(format!("Failed to read archive: {}", e)))?;

    let mut extracted = false;
    for entry in entries {
        let mut entry = entry.map_err(|e| ParrotError::Io(format!("Failed to read entry: {}", e)))?;
        let path = entry.path().map_err(|e| ParrotError::Io(e.to_string()))?;

        if path.starts_with(&package_path) {
            let relative = path.strip_prefix(&package_path).unwrap();
            let target = dest_dir.join(relative);

            if entry.header().entry_type() == tar::EntryType::Directory {
                fs::create_dir_all(&target).map_err(|e| ParrotError::Io(e.to_string()))?;
            } else {
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent).map_err(|e| ParrotError::Io(e.to_string()))?;
                }
                entry.unpack(&target).map_err(|e| ParrotError::Io(e.to_string()))?;
            }
            extracted = true;
        }
    }

    if !extracted {
        return Err(ParrotError::NotFound(format!(
            "Package '{}' not found in registry archive",
            package_name
        )));
    }

    eprintln!("[parrot] Successfully installed {} to {}", package_name, dest_dir.display());
    Ok(())
}

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

    #[test]
    fn test_github_archive_url() {
        let url = get_archive_url_for_mirror("catlang/std", false);
        assert_eq!(
            url,
            "https://github.com/wumingshiali/catlang-pkg/archive/refs/heads/main/catlang/std.tar.gz"
        );
    }

    #[test]
    fn test_mirror_archive_url() {
        let url = get_archive_url_for_mirror("catlang/std", true);
        assert_eq!(
            url,
            "https://cdn.jsdmirror.com/gh/wumingshiali/catlang-pkg/archive/refs/heads/main/catlang/std.tar.gz"
        );
    }

    #[test]
    fn test_archive_url_with_author_slash_name() {
        let url = get_archive_url_for_mirror("wumingshiali/utils", false);
        assert!(url.contains("wumingshiali/utils"));
        assert!(url.ends_with(".tar.gz"));
    }

    #[test]
    fn test_parrot_error_display() {
        let io_err = ParrotError::Io("file not found".into());
        assert!(io_err.to_string().contains("IO error"));

        let parse_err = ParrotError::Parse("invalid toml".into());
        assert!(parse_err.to_string().contains("Parse error"));

        let not_found = ParrotError::NotFound("package missing".into());
        assert!(not_found.to_string().contains("Not found"));

        let network_err = ParrotError::Network("timeout".into());
        assert!(network_err.to_string().contains("Network error"));
    }

    #[test]
    fn test_parrot_error_is_error_trait() {
        let err = ParrotError::NotFound("test".into());
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_resolve_dependencies_empty() {
        let content = r#"
[package]
name = "test"
version = "0.1.0"
"#;
        let config: ParrotConfig = toml::from_str(content).unwrap();
        let deps = resolve_dependencies(&config, Path::new("/tmp"));
        assert!(deps.is_empty());
    }

    #[test]
    fn test_resolve_dependencies_nonexistent() {
        let content = r#"
[package]
name = "test"
version = "0.1.0"

[dependencies]
nonexistent = "1.0.0"
"#;
        let config: ParrotConfig = toml::from_str(content).unwrap();
        let deps = resolve_dependencies(&config, Path::new("/tmp"));
        assert!(deps.is_empty());
    }

    #[test]
    fn test_generate_import_paths_empty() {
        let content = r#"
[package]
name = "test"
version = "0.1.0"
"#;
        let config: ParrotConfig = toml::from_str(content).unwrap();
        let paths = generate_import_paths(&config, Path::new("/tmp"));
        assert!(paths.is_empty());
    }

    #[test]
    fn test_dependency_struct() {
        let dep = Dependency {
            name: "utils".into(),
            version: "1.0.0".into(),
            path: PathBuf::from("/tmp/utils"),
        };
        assert_eq!(dep.name, "utils");
        assert_eq!(dep.version, "1.0.0");
        assert_eq!(dep.path, PathBuf::from("/tmp/utils"));
    }

    #[test]
    fn test_package_info_struct() {
        let info = PackageInfo {
            name: "my_package".into(),
            version: "2.0.0".into(),
        };
        assert_eq!(info.name, "my_package");
        assert_eq!(info.version, "2.0.0");
    }

    #[test]
    fn test_parrot_config_clone() {
        let content = r#"
[package]
name = "test"
version = "0.1.0"

[dependencies]
utils = "1.0.0"
"#;
        let config: ParrotConfig = toml::from_str(content).unwrap();
        let cloned = config.clone();
        assert_eq!(cloned.package.name, config.package.name);
        assert_eq!(cloned.dependencies, config.dependencies);
    }

    #[test]
    fn test_branch_name_constant() {
        assert_eq!(BRANCH_NAME, "main");
    }

    #[test]
    fn test_slow_threshold_constant() {
        assert_eq!(SLOW_THRESHOLD_MS, 3000);
    }

    #[test]
    fn test_fetch_package_invalid_dest() {
        let result = fetch_package("test/pkg", Path::new("/nonexistent/invalid/path/that/cannot/exist"));
        assert!(result.is_err());
    }

    #[test]
    fn test_is_github_slow_real_check() {
        let slow = is_github_slow();
        eprintln!("GitHub slow: {}", slow);
    }

    #[test]
    fn test_fetch_package_real_download() {
        let dir = tempfile::tempdir().unwrap();
        let result = fetch_package("catlang/std", dir.path());
        eprintln!("Download result: {:?}", result);
    }
}
