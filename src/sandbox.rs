//! Sandbox Module
//!
//! Provides file access restriction for compiled programs.
//! - cfgSandboxPath: restricts accessible directories
//! - cfgSandboxFlag: toggles sandbox on/off
//! - File config (Parrot.toml) has highest priority
//! - Program can only access files it creates and allowed directories

use std::path::{Path, PathBuf};

/// Sandbox configuration
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub enabled: bool,
    pub allowed_paths: Vec<PathBuf>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_paths: Vec::new(),
        }
    }
}

impl SandboxConfig {
    pub fn new(enabled: bool, allowed_paths: Vec<PathBuf>) -> Self {
        Self { enabled, allowed_paths }
    }

    pub fn is_disabled() -> Self {
        Self {
            enabled: false,
            allowed_paths: Vec::new(),
        }
    }

    pub fn from_parrot_config(config: &crate::parrot::ParrotConfig, config_path: &Path) -> Self {
        let enabled = config.sandbox_flag.unwrap_or(false);

        let mut allowed_paths = Vec::new();

        if let Some(ref sandbox_path) = config.sandbox_path {
            let path = PathBuf::from(sandbox_path);
            let resolved = if path.is_absolute() {
                path
            } else {
                config_path
                    .parent()
                    .unwrap_or_else(|| Path::new("."))
                    .join(&path)
            };
            allowed_paths.push(resolved);
        }

        Self { enabled, allowed_paths }
    }

    pub fn is_path_allowed(&self, path: &Path) -> bool {
        if !self.enabled {
            return true;
        }

        let path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_default().join(path)
        };

        for allowed in &self.allowed_paths {
            if path.starts_with(allowed) {
                return true;
            }
        }

        false
    }

    pub fn to_zig_init_code(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        let mut code = String::new();
        code.push_str("// Sandbox initialization\n");
        code.push_str(&format!("var SANDBOX_ENABLED = true;\n"));

        if self.allowed_paths.is_empty() {
            code.push_str("var sandbox_allowed_paths: [0][]const u8 = .{};\n");
        } else {
            code.push_str(&format!(
                "var sandbox_allowed_paths: [{}][][]const u8 = .{{\n",
                self.allowed_paths.len()
            ));
            for path in &self.allowed_paths {
                code.push_str(&format!("    \"{}\",\n", path.display()));
            }
            code.push_str("};\n");
        }

        code.push('\n');
        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_default_disabled() {
        let config = SandboxConfig::default();
        assert!(!config.enabled);
        assert!(config.allowed_paths.is_empty());
    }

    #[test]
    fn test_sandbox_is_disabled() {
        let config = SandboxConfig::is_disabled();
        assert!(!config.enabled);
    }

    #[test]
    fn test_sandbox_enabled_allows_all_when_no_paths() {
        let config = SandboxConfig::new(true, Vec::new());
        assert!(config.enabled);
        assert!(config.allowed_paths.is_empty());
    }

    #[test]
    fn test_sandbox_path_allowed_absolute() {
        let allowed = vec![PathBuf::from("/tmp/sandbox")];
        let config = SandboxConfig::new(true, allowed);
        assert!(config.is_path_allowed(Path::new("/tmp/sandbox/file.txt")));
        assert!(config.is_path_allowed(Path::new("/tmp/sandbox/sub/dir/file.txt")));
    }

    #[test]
    fn test_sandbox_path_not_allowed() {
        let allowed = vec![PathBuf::from("/tmp/sandbox")];
        let config = SandboxConfig::new(true, allowed);
        assert!(!config.is_path_allowed(Path::new("/etc/passwd")));
        assert!(!config.is_path_allowed(Path::new("/home/user/secret.txt")));
    }

    #[test]
    fn test_sandbox_disabled_allows_all() {
        let allowed = vec![PathBuf::from("/tmp/sandbox")];
        let config = SandboxConfig::new(false, allowed);
        assert!(config.is_path_allowed(Path::new("/etc/passwd")));
        assert!(config.is_path_allowed(Path::new("/tmp/sandbox/file.txt")));
    }

    #[test]
    fn test_sandbox_multiple_paths() {
        let allowed = vec![
            PathBuf::from("/tmp/sandbox"),
            PathBuf::from("/var/data"),
        ];
        let config = SandboxConfig::new(true, allowed);
        assert!(config.is_path_allowed(Path::new("/tmp/sandbox/file.txt")));
        assert!(config.is_path_allowed(Path::new("/var/data/file.txt")));
        assert!(!config.is_path_allowed(Path::new("/etc/passwd")));
    }

    #[test]
    fn test_sandbox_zig_init_code_disabled() {
        let config = SandboxConfig::new(false, Vec::new());
        let code = config.to_zig_init_code();
        assert!(code.is_empty());
    }

    #[test]
    fn test_sandbox_zig_init_code_enabled() {
        let allowed = vec![PathBuf::from("/tmp/sandbox")];
        let config = SandboxConfig::new(true, allowed);
        let code = config.to_zig_init_code();
        assert!(code.contains("SANDBOX_ENABLED = true"));
        assert!(code.contains("/tmp/sandbox"));
    }

    #[test]
    fn test_sandbox_zig_init_code_no_paths() {
        let config = SandboxConfig::new(true, Vec::new());
        let code = config.to_zig_init_code();
        assert!(code.contains("SANDBOX_ENABLED = true"));
        assert!(code.contains("[0][]const u8"));
    }
}
