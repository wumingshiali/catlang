//! CatLang Compiler CLI
//!
//! Usage: catc <INPUT>
//!
//! Arguments:
//!   <INPUT>    Input CatLang source file (e.g., abc.cat)
//!
//! Options:
//!   -h, --help       Print help
//!   -V, --version    Print version
//!   -O, --opt <LVL>  Optimization level (0-3, default: 2)
//!   --release        Enable release mode (equivalent to -O 3)

use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process::Command;

use catlang::compile_with_opts;
use catlang::parrot;

#[derive(Parser, Debug)]
#[command(name = "catc")]
#[command(author = "CatLang Team")]
#[command(version = "0.1.0")]
#[command(about = "CatLang Compiler - Compiles .cat files to .exe", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Input CatLang source file
    #[arg(required = false)]
    input: Option<String>,

    /// Optimization level (0-3)
    #[arg(short = 'O', long = "opt", default_value = "2")]
    opt_level: u8,

    /// Enable release mode (maximum optimization)
    #[arg(long = "release")]
    release: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile a CatLang source file
    Build {
        /// Input CatLang source file
        input: String,
    },
    /// Parrot package manager
    Parrot {
        #[command(subcommand)]
        action: ParrotAction,
    },
}

#[derive(Subcommand, Debug)]
enum ParrotAction {
    /// Install dependencies from Parrot.toml
    Install,
    /// Add a dependency
    Add {
        /// Package name
        name: String,
        /// Version (default: "latest")
        #[arg(default_value = "latest")]
        version: String,
    },
    /// Remove a dependency
    Remove {
        /// Package name
        name: String,
    },
    /// Update dependencies
    Update,
    /// List installed dependencies
    List,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Build { input }) => {
            compile_file(&input, args.opt_level, args.release);
        }
        Some(Commands::Parrot { action }) => {
            handle_parrot_action(action);
        }
        None => {
            // Backward compatibility: if input is provided without subcommand
            if let Some(input) = args.input {
                compile_file(&input, args.opt_level, args.release);
            } else {
                eprintln!("Usage: catc <INPUT> or catc build <INPUT> or catc parrot <ACTION>");
                std::process::exit(1);
            }
        }
    }
}

fn compile_file(input: &str, opt_level: u8, release: bool) {
    let input_path = Path::new(input);
    if !input_path.exists() {
        eprintln!("Error: Input file '{}' does not exist", input);
        std::process::exit(1);
    }

    let source = match fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input, e);
            std::process::exit(1);
        }
    };

    let output_name = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let output_exe = format!("{}.exe", output_name);
    let zig_file = format!("{}.zig", output_name);

    let opt_level = if release {
        3
    } else {
        opt_level.min(3)
    };
    let zig_opt = if opt_level >= 3 {
        "ReleaseFast"
    } else if opt_level >= 2 {
        "ReleaseSafe"
    } else if opt_level >= 1 {
        "Debug"
    } else {
        "Debug"
    };

    eprintln!(
        "[catc] Compiling '{}' (optimization level: {})...",
        input, opt_level
    );

    let zig_code = match compile_with_opts(&source, opt_level) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Compilation failed: {}", e);
            std::process::exit(1);
        }
    };

    match fs::write(&zig_file, &zig_code) {
        Ok(_) => eprintln!("[catc] Generated '{}'", zig_file),
        Err(e) => {
            eprintln!("Error writing Zig file: {}", e);
            std::process::exit(1);
        }
    };

    eprintln!("[catc] Building executable with Zig (-O {})...", zig_opt);
    let output_arg = format!("-femit-bin={}", output_exe);

    let mut zig_args = vec![
        "build-exe".to_string(),
        zig_file.clone(),
        output_arg,
        "-O".to_string(),
        zig_opt.to_string(),
    ];

    if opt_level == 0 {
        zig_args.push("-fno-omit-frame-pointer".to_string());
    }

    let zig_build = Command::new("zig").args(&zig_args).output();

    match zig_build {
        Ok(result) => {
            if !result.status.success() {
                eprintln!("Zig compilation failed:");
                eprintln!("{}", String::from_utf8_lossy(&result.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!(
                "Error: Could not find 'zig' compiler. Please ensure Zig is installed and in PATH."
            );
            eprintln!("Error details: {}", e);
            std::process::exit(1);
        }
    }

    let _ = fs::remove_file(&zig_file);
    eprintln!("[catc] Successfully created '{}'", output_exe);
}

fn handle_parrot_action(action: ParrotAction) {
    match action {
        ParrotAction::Install => {
            cmd_install();
        }
        ParrotAction::Add { name, version } => {
            cmd_add(&name, &version);
        }
        ParrotAction::Remove { name } => {
            cmd_remove(&name);
        }
        ParrotAction::Update => {
            cmd_update();
        }
        ParrotAction::List => {
            cmd_list();
        }
    }
}

fn find_config_or_exit() -> (parrot::ParrotConfig, std::path::PathBuf) {
    let cwd = std::env::current_dir().unwrap_or_else(|e| {
        eprintln!("Error: Cannot determine current directory: {}", e);
        std::process::exit(1);
    });

    match parrot::find_config(&cwd.join("dummy.cat")) {
        Ok(config_path) => {
            match parrot::parse_config(&config_path) {
                Ok(config) => (config, config_path),
                Err(e) => {
                    eprintln!("Error parsing Parrot.toml: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Run 'catc parrot init' to create a new project");
            std::process::exit(1);
        }
    }
}

fn cmd_install() {
    let (config, _config_path) = find_config_or_exit();
    let global_dir = parrot::global_packages_dir();

    eprintln!("[parrot] Installing dependencies...");

    let _ = fs::create_dir_all(&global_dir);

    let mut installed = 0;
    let mut failed = 0;

    for (name, version) in &config.dependencies {
        let dest = global_dir.join(name);

        if dest.exists() {
            eprintln!("[parrot] {}@{} already installed, skipping", name, version);
            installed += 1;
            continue;
        }

        eprintln!("[parrot] Installing {}@{}...", name, version);
        match parrot::fetch_package(name, &dest) {
            Ok(_) => {
                installed += 1;
            }
            Err(e) => {
                eprintln!("[parrot] Failed to install {}: {}", name, e);
                failed += 1;
                let _ = fs::remove_dir_all(&dest);
            }
        }
    }

    if failed > 0 {
        eprintln!("[parrot] Installation complete: {} installed, {} failed", installed, failed);
        std::process::exit(1);
    } else {
        eprintln!("[parrot] {} dependencies installed successfully", installed);
    }
}

fn cmd_add(name: &str, version: &str) {
    let (mut config, config_path) = find_config_or_exit();
    let global_dir = parrot::global_packages_dir();

    config.dependencies.insert(name.to_string(), version.to_string());

    let toml_content = toml::to_string_pretty(&config).unwrap_or_else(|e| {
        eprintln!("Error serializing config: {}", e);
        std::process::exit(1);
    });

    fs::write(&config_path, toml_content).unwrap_or_else(|e| {
        eprintln!("Error writing Parrot.toml: {}", e);
        std::process::exit(1);
    });

    eprintln!("[parrot] Added {}@{} to dependencies", name, version);

    let dest = global_dir.join(name);
    if dest.exists() {
        eprintln!("[parrot] {}@{} already installed, skipping download", name, version);
        return;
    }

    eprintln!("[parrot] Fetching {}@{}...", name, version);
    let _ = fs::create_dir_all(&global_dir);
    match parrot::fetch_package(name, &dest) {
        Ok(_) => {
            eprintln!("[parrot] Successfully installed {}@{}", name, version);
        }
        Err(e) => {
            eprintln!("[parrot] Failed to fetch {}: {}", name, e);
            let _ = fs::remove_dir_all(&dest);
            std::process::exit(1);
        }
    }
}

fn cmd_remove(name: &str) {
    let (mut config, config_path) = find_config_or_exit();

    if config.dependencies.remove(name).is_none() {
        eprintln!("[parrot] Warning: {} not found in dependencies", name);
        return;
    }

    let toml_content = toml::to_string_pretty(&config).unwrap_or_else(|e| {
        eprintln!("Error serializing config: {}", e);
        std::process::exit(1);
    });

    fs::write(&config_path, toml_content).unwrap_or_else(|e| {
        eprintln!("Error writing Parrot.toml: {}", e);
        std::process::exit(1);
    });

    eprintln!("[parrot] Removed {} from dependencies", name);
}

fn cmd_update() {
    let (_config, _config_path) = find_config_or_exit();
    eprintln!("[parrot] Update not yet implemented");
}

fn cmd_list() {
    let (config, config_path) = find_config_or_exit();
    let deps = parrot::resolve_dependencies(&config, &config_path);

    if config.dependencies.is_empty() {
        eprintln!("[parrot] No dependencies declared");
        return;
    }

    eprintln!("[parrot] Dependencies:");
    for (name, version) in &config.dependencies {
        let status = deps.iter().find(|d| &d.name == name);
        match status {
            Some(dep) => {
                eprintln!("  {}@{} (installed: {})", name, version, dep.path.display());
            }
            None => {
                eprintln!("  {}@{} (not installed)", name, version);
            }
        }
    }
}
